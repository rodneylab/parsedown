/* Copyright 2014-2017 The html5ever Project Developers. See the
 * COPYRIGHT file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

use html5ever::{
    interface::tree_builder::{self, ElementFlags, NodeOrText, QuirksMode, TreeSink},
    serialize::{
        Serialize, Serializer,
        TraversalScope::{self, ChildrenOnly, IncludeNode},
    },
    tendril::StrTendril,
    Attribute, ExpandedName, QualName,
};
use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    collections::{HashSet, VecDeque},
    fmt, io, mem,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub enum NodeData {
    Document,
    Doctype {
        name: StrTendril,

        #[allow(dead_code)]
        public_id: StrTendril,

        #[allow(dead_code)]
        system_id: StrTendril,
    },
    Text {
        contents: RefCell<StrTendril>,
    },
    Comment {
        contents: StrTendril,
    },
    Element {
        name: QualName,
        attrs: RefCell<Vec<Attribute>>,
        template_contents: RefCell<Option<Handle>>,
        mathml_annotation_xml_integration_point: bool,
    },

    ProcessingInstruction {
        target: StrTendril,
        contents: StrTendril,
    },
}

pub struct Node {
    pub parent: Cell<Option<WeakHandle>>,
    pub children: RefCell<Vec<Handle>>,
    pub data: NodeData,
}

impl Node {
    pub fn new(data: NodeData) -> Rc<Self> {
        Rc::new(Node {
            data,
            parent: Cell::new(None),
            children: RefCell::new(Vec::new()),
        })
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        let mut nodes = mem::take(&mut *self.children.borrow_mut());
        while let Some(node) = nodes.pop() {
            let children = mem::take(&mut *node.children.borrow_mut());
            nodes.extend(children.into_iter());
            if let NodeData::Element {
                ref template_contents,
                ..
            } = node.data
            {
                if let Some(template_contents) = template_contents.borrow_mut().take() {
                    nodes.push(template_contents);
                }
            }
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Node")
            .field("data", &self.data)
            .field("children", &self.children)
            .finish_non_exhaustive()
    }
}

pub type Handle = Rc<Node>;

pub type WeakHandle = Weak<Node>;

fn append(new_parent: &Handle, child: Handle) {
    let previous_parent = child.parent.replace(Some(Rc::downgrade(new_parent)));
    assert!(previous_parent.is_none());
    new_parent.children.borrow_mut().push(child);
}

fn get_parent_and_index(target: &Handle) -> Option<(Handle, usize)> {
    if let Some(weak) = target.parent.take() {
        let parent = weak.upgrade().expect("dangling weak pointer");
        target.parent.set(Some(weak));
        let Some((i, _)) = parent
            .children
            .borrow()
            .iter()
            .enumerate()
            .find(|&(_, child)| Rc::ptr_eq(child, target))
        else {
            panic!("have parent but couldn't find in parent's children!")
        };
        Some((parent, i))
    } else {
        None
    }
}

fn append_to_existing_text(prev: &Handle, text: &str) -> bool {
    match prev.data {
        NodeData::Text { ref contents } => {
            contents.borrow_mut().push_slice(text);
            true
        }
        _ => false,
    }
}

fn remove_from_parent(target: &Handle) {
    if let Some((parent, i)) = get_parent_and_index(target) {
        parent.children.borrow_mut().remove(i);
        target.parent.set(None);
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct RcDom {
    pub document: Handle,
    pub errors: Vec<Cow<'static, str>>,
    pub quirks_mode: QuirksMode,
}

impl TreeSink for RcDom {
    type Output = Self;
    fn finish(self) -> Self {
        self
    }
    type Handle = Handle;

    fn parse_error(&mut self, msg: Cow<'static, str>) {
        self.errors.push(msg);
    }

    fn get_document(&mut self) -> Handle {
        self.document.clone()
    }

    fn get_template_contents(&mut self, target: &Handle) -> Handle {
        if let NodeData::Element {
            ref template_contents,
            ..
        } = target.data
        {
            template_contents
                .borrow()
                .as_ref()
                .expect("not a template element!")
                .clone()
        } else {
            panic!("not a template element!")
        }
    }

    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        self.quirks_mode = mode;
    }

    fn same_node(&self, x: &Handle, y: &Handle) -> bool {
        Rc::ptr_eq(x, y)
    }

    fn elem_name<'a>(&self, target: &'a Handle) -> ExpandedName<'a> {
        return match target.data {
            NodeData::Element { ref name, .. } => name.expanded(),
            _ => panic!("not an element"),
        };
    }

    fn create_element(
        &mut self,
        name: QualName,
        attrs: Vec<Attribute>,
        flags: ElementFlags,
    ) -> Handle {
        Node::new(NodeData::Element {
            name,
            attrs: RefCell::new(attrs),
            template_contents: RefCell::new(if flags.template {
                Some(Node::new(NodeData::Document))
            } else {
                None
            }),
            mathml_annotation_xml_integration_point: flags.mathml_annotation_xml_integration_point,
        })
    }

    fn create_comment(&mut self, text: StrTendril) -> Handle {
        Node::new(NodeData::Comment { contents: text })
    }

    fn create_pi(&mut self, target: StrTendril, data: StrTendril) -> Handle {
        Node::new(NodeData::ProcessingInstruction {
            target,
            contents: data,
        })
    }

    fn append(&mut self, parent: &Handle, child: NodeOrText<Handle>) {
        if let NodeOrText::AppendText(ref text) = child {
            if let Some(h) = parent.children.borrow().last() {
                if append_to_existing_text(h, text) {
                    return;
                }
            }
        }

        append(
            parent,
            match child {
                NodeOrText::AppendText(text) => Node::new(NodeData::Text {
                    contents: RefCell::new(text),
                }),
                NodeOrText::AppendNode(node) => node,
            },
        );
    }

    fn append_before_sibling(&mut self, sibling: &Handle, child: NodeOrText<Handle>) {
        let (parent, i) = get_parent_and_index(sibling)
            .expect("append_before_sibling called on node without parent");

        let child = match (child, i) {
            (NodeOrText::AppendText(text), 0) => Node::new(NodeData::Text {
                contents: RefCell::new(text),
            }),
            (NodeOrText::AppendText(text), i) => {
                let children = parent.children.borrow();
                let prev = &children[i - 1];
                if append_to_existing_text(prev, &text) {
                    return;
                }
                Node::new(NodeData::Text {
                    contents: RefCell::new(text),
                })
            }
            (NodeOrText::AppendNode(node), _) => node,
        };

        remove_from_parent(&child);

        child.parent.set(Some(Rc::downgrade(&parent)));
        parent.children.borrow_mut().insert(i, child);
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        let parent = element.parent.take();
        let has_parent = parent.is_some();
        element.parent.set(parent);

        if has_parent {
            self.append_before_sibling(element, child);
        } else {
            self.append(prev_element, child);
        }
    }

    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        append(
            &self.document,
            Node::new(NodeData::Doctype {
                name,
                public_id,
                system_id,
            }),
        );
    }

    fn add_attrs_if_missing(&mut self, target: &Handle, attrs: Vec<Attribute>) {
        let mut existing = if let NodeData::Element { ref attrs, .. } = target.data {
            attrs.borrow_mut()
        } else {
            panic!("not an elemet")
        };

        let existing_names = existing
            .iter()
            .map(|e| e.name.clone())
            .collect::<HashSet<_>>();
        existing.extend(
            attrs
                .into_iter()
                .filter(|attr| !existing_names.contains(&attr.name)),
        );
    }

    fn remove_from_parent(&mut self, target: &Handle) {
        remove_from_parent(target);
    }

    fn reparent_children(&mut self, node: &Handle, new_parent: &Handle) {
        let mut children = node.children.borrow_mut();
        let mut new_children = new_parent.children.borrow_mut();
        for child in children.iter() {
            let previous_parent = child.parent.replace(Some(Rc::downgrade(new_parent)));
            assert!(Rc::ptr_eq(
                node,
                &previous_parent.unwrap().upgrade().expect("dangling weak")
            ));
        }
        new_children.extend(mem::take(&mut *children));
    }

    fn is_mathml_annotation_xml_integration_point(&self, target: &Handle) -> bool {
        if let NodeData::Element {
            mathml_annotation_xml_integration_point,
            ..
        } = target.data
        {
            mathml_annotation_xml_integration_point
        } else {
            panic!("not an element")
        }
    }
}

impl Default for RcDom {
    fn default() -> RcDom {
        RcDom {
            document: Node::new(NodeData::Document),
            errors: vec![],
            quirks_mode: tree_builder::NoQuirks,
        }
    }
}

enum SerializeOp {
    Open(Handle),
    Close(QualName),
}

pub struct SerializableHandle(Handle);

impl From<Handle> for SerializableHandle {
    fn from(h: Handle) -> SerializableHandle {
        SerializableHandle(h)
    }
}

impl Serialize for SerializableHandle {
    fn serialize<S>(&self, serializer: &mut S, traversal_scope: TraversalScope) -> io::Result<()>
    where
        S: Serializer,
    {
        let mut ops = VecDeque::new();
        match traversal_scope {
            IncludeNode => ops.push_back(SerializeOp::Open(self.0.clone())),
            ChildrenOnly(_) => ops.extend(
                self.0
                    .children
                    .borrow()
                    .iter()
                    .map(|h| SerializeOp::Open(h.clone())),
            ),
        }

        while let Some(op) = ops.pop_front() {
            match op {
                SerializeOp::Open(handle) => match handle.data {
                    NodeData::Element {
                        ref name,
                        ref attrs,
                        ..
                    } => {
                        serializer.start_elem(
                            name.clone(),
                            attrs.borrow().iter().map(|at| (&at.name, &at.value[..])),
                        )?;

                        ops.reserve(1 + handle.children.borrow().len());
                        ops.push_front(SerializeOp::Close(name.clone()));

                        for child in handle.children.borrow().iter().rev() {
                            ops.push_front(SerializeOp::Open(child.clone()));
                        }
                    }

                    NodeData::Doctype { ref name, .. } => serializer.write_doctype(name)?,

                    NodeData::Text { ref contents } => serializer.write_text(&contents.borrow())?,

                    NodeData::Comment { ref contents } => serializer.write_comment(contents)?,

                    NodeData::ProcessingInstruction {
                        ref target,
                        ref contents,
                    } => serializer.write_processing_instruction(target, contents)?,

                    NodeData::Document => panic!("Can't serialise Document node itself"),
                },

                SerializeOp::Close(name) => {
                    serializer.end_elem(name)?;
                }
            }
        }

        Ok(())
    }
}
