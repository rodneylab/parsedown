import {
  markdown_to_html as markdownToHtml,
  markdown_to_plaintext as markdownToPlaintext,
  mjml_to_html as mjmlToHtml,
} from "@rodneylab/parsedown";
import { assert, expect, test } from "vitest";

test("it parses markdown to html", async () => {
  // arrange
  const markdown = `
## 👋🏽 Hello You

* alpha
* beta
`;

  // act
  const { errors, html, headings, statistics } = (await markdownToHtml(
    markdown,
    {},
  ))!;

  // assert
  assert(typeof markdownToHtml === "function");

  expect(typeof errors).toBe("undefined");

  const { reading_time, word_count } = statistics;

  assert(typeof headings !== "undefined");
  expect(headings?.length).toBe(1);
  expect(headings?.[0]).toStrictEqual({
    heading: "👋🏽 Hello You",
    id: "wave-hello-you",
  });
  expect(html).toBe(
    `<h2 id="wave-hello-you">👋🏽 Hello You <a href="#wave-hello-you" class="heading-anchor">#</a></h2>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
`,
  );

  expect(reading_time).toBe(1);
  expect(word_count).toBe(4);
});

test("it highlights search terms in generated HTML", async () => {
  // prepare
  const markdown = "Nobody likes maple in their apple flavoured Snapple.";
  // act
  const { errors, html } = await markdownToHtml(markdown, {
    search_term: "apple",
  });

  // assert
  assert(typeof errors === "undefined");
  expect(html).toBe(
    `<p>Nobody likes maple in their <mark id="search-match">apple</mark> flavoured Sn<mark>apple</mark>.</p>\n`,
  );
});

test("it parses markdown to plain text", () => {
  // arrange
  const markdown = `
## 👋🏽 Hello You

* alpha
* beta

[Example Link](https://example.com/)
`;

  // act
  const plaintext = markdownToPlaintext(markdown, {});

  // assert
  assert(typeof markdownToPlaintext === "function");
  expect(plaintext).toBe(
    `👋🏽 Hello You

- alpha

- beta

Example Link (https://example.com/)
`,
  );
});

test("it parses mjml to html", () => {
  // arrange
  const mjml = "<mjml></mjml>";

  // act
  const html = mjmlToHtml(mjml);

  // assert
  assert(typeof mjmlToHtml === "function");
  expect(html).toBe(
    `<!doctype html><html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office"><head><title></title><!--[if !mso]><!--><meta http-equiv="X-UA-Compatible" content="IE=edge"><!--<![endif]--><meta http-equiv="Content-Type" content="text/html; charset=UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<style type="text/css">
#outlook a { padding: 0; }
body { margin: 0; padding: 0; -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; }
table, td { border-collapse: collapse; mso-table-lspace: 0pt; mso-table-rspace: 0pt; }
img { border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none; -ms-interpolation-mode: bicubic; }
p { display: block; margin: 13px 0; }
</style>
<!--[if mso]>
<noscript>
<xml>
<o:OfficeDocumentSettings>
  <o:AllowPNG/>
  <o:PixelsPerInch>96</o:PixelsPerInch>
</o:OfficeDocumentSettings>
</xml>
</noscript>
<![endif]-->
<!--[if lte mso 11]>
<style type="text/css">
.mj-outlook-group-fix { width:100% !important; }
</style>
<![endif]-->
<style type="text/css"></style></head><body></body></html>`,
  );
});
