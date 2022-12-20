#[cfg(test)]
mod tests;

mod dom;

use html5ever::{
    driver,
    interface::tree_builder::{AppendNode, NodeOrText, TreeSink},
    local_name, namespace_url, ns,
    serialize::{serialize, SerializeOpts},
    tendril::*,
    Attribute, QualName,
};
use std::{
    cell::RefCell,
    fmt::{self, Display},
    mem,
    rc::Rc,
};
use url::Url;

use dom::{Handle, Node, NodeData, RcDom, SerializableHandle};

#[derive(Debug)]
pub struct Builder<'a> {
    link_rel: Option<&'a str>,
}

impl<'a> Default for Builder<'a> {
    fn default() -> Self {
        Builder {
            link_rel: Some("noopener noreferrer"),
        }
    }
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn link_rel(&mut self, value: Option<&'a str>) -> &mut Self {
        self.link_rel = value;
        self
    }

    fn process_child(&self, _child: &mut Handle) -> bool {
        true
    }

    pub fn process_dom(&self, mut dom: RcDom) -> Document {
        let mut stack = Vec::new();
        let link_rel = self.link_rel.map(|link_rel| format_tendril!("{link_rel}"));
        let body = {
            let children = dom.document.children.borrow();
            children[0].clone()
        };
        stack.extend(
            mem::take(&mut *body.children.borrow_mut())
                .into_iter()
                .rev(),
        );

        while let Some(mut node) = stack.pop() {
            let parent = node.parent.replace(None).expect("a node in the DO will have a parent, except the root, which is not processed")
                .upgrade().expect("a node's parent will be pointed to by its parent (or the root pointer), and will not be dropped");
            let pass_process = self.process_child(&mut node);
            if pass_process {
                self.adjust_node_attributes(&mut node, &link_rel);
                self.adjust_node_children(&mut node, &mut dom);
                dom.append(&parent.clone(), NodeOrText::AppendNode(node.clone()));
            } else {
                for sub in node.children.borrow_mut().iter_mut() {
                    sub.parent.replace(Some(Rc::downgrade(&parent)));
                }
            }
            stack.extend(
                mem::take(&mut *node.children.borrow_mut())
                    .into_iter()
                    .rev(),
            );
        }
        Document(dom)
    }

    pub fn process(&self, src: &str) -> Document {
        let parser = Self::make_parser();
        let dom = parser.one(src);
        self.process_dom(dom)
    }

    fn adjust_node_attributes(&self, child: &mut Handle, link_rel: &Option<StrTendril>) {
        if let NodeData::Element {
            ref name,
            ref attrs,
            ..
        } = child.data
        {
            if let Some(ref link_rel) = *link_rel {
                if &*name.local == "a" {
                    let mut attrs = attrs.borrow_mut();
                    if let Some(attr) = attrs.iter().find(|attr| &*attr.name.local == "href") {
                        if !relative_url(&attr.value) {
                            attrs.push(Attribute {
                                name: QualName::new(None, ns!(), local_name!("rel")),
                                value: link_rel.clone(),
                            })
                        }
                    } else {
                        // TODO: anchor tag has no href - can emit a warning
                    };
                }
            }
        }
    }

    fn adjust_node_children(&self, child: &mut Handle, dom: &mut RcDom) {
        if let NodeData::Element {
            ref name,
            ref attrs,
            ..
        } = child.data
        {
            if &*name.local == "h2" {
                let attrs = attrs.borrow();
                let href = if let Some(attr) = attrs.iter().find(|attr| &*attr.name.local == "id") {
                    &*attr.value
                } else {
                    return;
                };

                let mut new_node_attrs: Vec<Attribute> = Vec::new();
                let new_node_attr = Attribute {
                    name: QualName::new(None, ns!(), "href".into()),
                    value: format!("#{href}").into(),
                };
                let new_node_class = Attribute {
                    name: QualName::new(None, ns!(), "class".into()),
                    value: "heading-anchor".into(),
                };
                new_node_attrs.push(new_node_attr);
                new_node_attrs.push(new_node_class);
                let new_node_text = Node::new(NodeData::Text {
                    contents: RefCell::new("#".into()),
                });
                let new_node = Node::new(NodeData::Element {
                    name: QualName::new(None, ns!(), local_name!("a")),
                    attrs: RefCell::new(new_node_attrs),
                    template_contents: RefCell::new(None),
                    mathml_annotation_xml_integration_point: false,
                });
                dom.append(&new_node, AppendNode(new_node_text));
                dom.append(
                    child,
                    AppendNode(Node::new(NodeData::Text {
                        contents: RefCell::new(" ".into()),
                    })),
                );
                dom.append(child, AppendNode(new_node));
            }
        }
    }

    pub fn make_parser() -> driver::Parser<RcDom> {
        driver::parse_fragment(
            RcDom::default(),
            driver::ParseOpts::default(),
            QualName::new(None, ns!(html), local_name!("div")),
            vec![],
        )
    }
}

pub struct Document(RcDom);

impl Document {
    fn serialize_opts() -> SerializeOpts {
        SerializeOpts::default()
    }
}

impl Clone for Document {
    fn clone(&self) -> Self {
        let parser = Builder::make_parser();
        let dom = parser.one(&self.to_string()[..]);
        Document(dom)
    }
}

impl Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opts = Self::serialize_opts();
        let mut ret_val = Vec::new();
        let inner: SerializableHandle = self.0.document.children.borrow()[0].clone().into();
        serialize(&mut ret_val, &inner, opts)
            .expect("Writing to a string shouldn't fail (expect on OOM)");
        String::from_utf8(ret_val)
            .expect("html5ever only supports UTF8")
            .fmt(f)
    }
}

fn relative_url(url: &str) -> bool {
    match Url::parse(url) {
        Ok(_) => false,
        Err(url::ParseError::RelativeUrlWithoutBase) => true,
        Err(_) => false,
    }
}

pub fn process_html(html: &str) -> String {
    Builder::new()
        .link_rel(Some("nofollow noopener noreferrer"))
        .process(html)
        .to_string()
}
