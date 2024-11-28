/**
 * WASM functions for processing Markdown text and MJML, written in Rust.  See REAMDE.md for
 * examples.
 */

import { instantiate } from "./lib/parsedown.generated.js";

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

interface MarkdownToHtmlOptions {
  canonicalRootUrl?: string;
  enableSmartPunctuation?: string;
  searchTerm?: string;
}

type MarkdownToPlaintextOptions = Omit<MarkdownToHtmlOptions, "searchTerm">;

/**
 * Convert the, input, `markdown` string to HTML using a [CommonMark](https://commonmark.org/)
 * Markdown Parser
 *
 * @param markdown The Markdown text to parse
 * @param options.enableSmartPunctuation `true` if "something" should be replaced with
 *                                       “something”, etc.
 * @returns `markdown` parsed into HTML as an object or an error object.  If successful, the HTML is
 *           in the `.html` field of the returned object.
 */
const markdownToHtml: (
  markdown: string,
  options?: MarkdownToHtmlOptions,
) => Promise<MarkdownToHtmlOKOutput | MarkdownToHtmlErrorOutput> =
  async function markdownToHtml(markdown, options) {
    const { markdown_to_html } = await instantiate();
    const { canonicalRootUrl, enableSmartPunctuation, searchTerm } = options ??
      {};

    return markdown_to_html(markdown, {
      enable_smart_punctuation: true,
      ...(typeof canonicalRootUrl !== "undefined"
        ? { canonical_root_url: canonicalRootUrl }
        : {}),
      ...(typeof enableSmartPunctuation !== "undefined"
        ? { enable_smart_punctuation: enableSmartPunctuation }
        : {}),
      ...(typeof searchTerm !== "undefined" ? { search_term: searchTerm } : {}),
    });
  };

/**
 * Convert the, input, `markdown` string to plaintext, to use, for example in a broadcast email or
 * RSS feed
 *
 * @param markdown The Markdown text to parse
 * @returns `markdown` parsed into a plaintext string
 */
const markdownToPlaintext: (
  markdown: string,
  options?: MarkdownToPlaintextOptions,
) => Promise<string> = async function markdownToPlaintext(markdown, options) {
  const { markdown_to_plaintext } = await instantiate();
  const { canonicalRootUrl, enableSmartPunctuation } = options ?? {};
  return markdown_to_plaintext(markdown, {
    ...(typeof canonicalRootUrl !== "undefined"
      ? { canonical_root_url: canonicalRootUrl }
      : {}),
    ...(typeof enableSmartPunctuation !== "undefined"
      ? { enable_smart_punctuation: enableSmartPunctuation }
      : {}),
  });
};

/**
 * Convert the, input, `mjml` string to HTML, for use in a broadcast email, for example
 *
 * @param markdown The Markdown text to parse
 * @returns `markdown` parsed into a plaintext string
 */
const mjmlToHtml: (mjml: string) => Promise<string> = async function mjmlToHtml(
  mjml,
) {
  const { mjml_to_html } = await instantiate();
  return mjml_to_html(mjml);
};

export { markdownToHtml, markdownToPlaintext, mjmlToHtml };
