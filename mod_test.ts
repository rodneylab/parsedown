import {
  assert,
  assertEquals,
} from "https://deno.land/std@0.170.0/testing/asserts.ts";
import { markdownToHtml, markdownToPlaintext, mjmlToHtml } from "./mod.ts";

Deno.test("it parses markdown to html", async () => {
  // arrange
  const markdown = `
## 👋🏽 Hello You

* alpha
* beta
`;

  // act
  const { errors, html, headings, statistics } = await markdownToHtml(
    markdown,
    {},
  );

  // assert
  assert(typeof markdownToHtml === "function");

  assertEquals(typeof errors, "undefined");
  assert(typeof statistics !== "undefined");

  const { reading_time, word_count } = statistics;

  assertEquals(headings.length, 1);
  assertEquals(headings[0], {
    heading: "👋🏽 Hello You",
    id: "wave-skin-tone-4-hello-you",
  });
  assertEquals(
    html,
    `<h2 id="wave-skin-tone-4-hello-you">👋🏽 Hello You <a href="#wave-skin-tone-4-hello-you" class="heading-anchor">#</a></h2>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
`,
  );

  assertEquals(reading_time, 1);
  assertEquals(word_count, 4);
});

Deno.test("it parses markdown to plain text", async () => {
  // arrange
  const markdown = `
## 👋🏽 Hello You

* alpha
* beta

[Example Link](https://example.com/)
`;

  // act
  const plaintext = await markdownToPlaintext(markdown, {});

  // assert
  assert(typeof markdownToPlaintext === "function");
  assertEquals(
    plaintext,
    `👋🏽 Hello You

- alpha

- beta

Example Link (https://example.com/)
`,
  );
});

Deno.test("it parses mjml to html", async () => {
  // arrange
  const mjml = "<mjml></mjml>";

  // act
  const html = await mjmlToHtml(mjml);

  // assert
  assert(typeof mjmlToHtml === "function");
  assertEquals(
    html,
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
</head><body></body></html>`,
  );
});
