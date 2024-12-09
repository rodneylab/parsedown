/**
 * WASM functions for processing Markdown text and MJML, written in Rust.  See README.md for
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
 * Markdown Parser.
 *
 * @param markdown The Markdown text to parse
 * @param {Object} [options={}] - Parse options
 * @param {boolean} options.enableSmartPunctuation - `true` if "something" should be replaced with
 *                                                   “something”, etc.
 * @param {string} options.canonicalRootUrl - if included, relative url gain this value as a prefix
 *                                            (`/home` becomes `https://example.com/home`)
 * @param {string} options.searchTerm - if included, output HTML wraps any instances of this value
 *                                      in `mark` tags (`A senctence with the-search-term` becomes
 *                                      `A sentence with <mark>the-search-term</mark>`), for use in
 *                                      highlighting search results, with CSS, for example.  The
 *                                      first instance also has  `id=search-match` added the mark
 *                                      tag.  You might use this to scroll the first match into view
 *                                      automatically.
 * @returns {Object} `markdown` parsed into HTML as an object or an error object.  If successful, the HTML is
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
 * RSS feed.
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
 * Convert the, input, `mjml` string to HTML, for use in a broadcast email, for example.
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
