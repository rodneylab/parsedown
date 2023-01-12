import {
  instantiate,
  markdown_to_html,
  markdown_to_plaintext,
  mjml_to_html,
} from "./lib/parsedown.generated.js";

interface MarkdownToHtmlOKOutput {
  errors?: never;
  headings: { heading: string; id: string }[];
  html: string;
  statistics: {
    reading_time: number;
    word_count: number;
  };
}

interface MarkdownToHtmlErrorOutput {
  errors: string[];
  headings?: never;
  html?: never;
  statistics?: never;
}

const markdownToHtml: (
  markdown: string,
  options?: { canonical_root_url?: string },
) => Promise<MarkdownToHtmlOKOutput | MarkdownToHtmlErrorOutput> =
  async function markdownToHtml(markdown, options) {
    await instantiate();
    return markdown_to_html(markdown, options);
  };

const markdownToPlaintext: (
  markdown: string,
  options?: { canonical_root_url?: string },
) => Promise<string> = async function markdownToPlaintext(markdown, options) {
  await instantiate();
  return markdown_to_plaintext(markdown, options);
};

const mjmlToHtml: (mjml: string) => Promise<string> = async function mjmlToHtml(
  mjml,
) {
  await instantiate();
  return mjml_to_html(mjml);
};

export { markdownToHtml, markdownToPlaintext, mjmlToHtml };
