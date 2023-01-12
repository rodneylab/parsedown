<img src="./images/rodneylab-github-parsedown.png" alt="Rodney Lab parse down Github banner">

<p align="center">
  <a aria-label="Open Rodney Lab site" href="https://rodneylab.com" rel="nofollow noopener noreferrer">
    <img alt="Rodney Lab logo" src="https://rodneylab.com/assets/icon.png" width="60" />
  </a>
</p>
<h1 align="center">
  parsedown
</h1>

Light touch Markdown parsing into HTML written in Rust. Generates WASM and can
be used with Deno Fresh.

- adds an id and anchor link to each h2 heading for easy linking,
- adds pretty punctuation,
- uses html5ever for HTML manipulation and pulldown-cmark for Markdown parsing.

## Compile WASM (see next section instead if working with Deno)

1. Clone the project and change into the project directory. Then run these
   commands:

```shell
cargo install wasm-pack # skip if you already have it installed
wasm-pack build --target web
```

2. Copy the generated `pkg` folder into your JavaScript or TypeScript project.
3. Import and use the code in one of your project source files: a. Parse
   Markdown to HTML

```typescript
import init, {
  markdown_to_html,
  markdown_to_plaintext,
  mjml_to_html,
} from "pkg/parsedown.js";

await init();

// alternative if top level await is not available
(async () => {
  await init();
})();

const { errors, headings, html, statistics } = await markdown_to_html(
  `
## 👋🏽 Hello You

* alpha
* beta
`,
  {},
);

/*
errors: "undefined"

headings: [{
  heading: "👋🏽 Hello You",
  id: "wave-skin-tone-4-hello-you",
}]

html: `<h2 id="wave-skin-tone-4-hello-you">👋🏽 Hello You <a href="#wave-skin-tone-4-hello-you" class="heading-anchor">#</a></h2>
<ul>
<li>alpha</li>
<li>beta</li>
</ul>
`

statistics: {
  reading_time: 1, // in minutes
  word_count: 4
}
*/
```

    b. Parse Markdown to Plaintext

```typescript
import init, {
  markdown_to_html,
  markdown_to_plaintext,
  mjml_to_html,
} from "pkg/parsedown.js";

await init();

const plaintext = await markdown_to_plaintext(
  `
## 👋🏽 Hello You

* alpha
* beta

[Example Link](https://example.com/)
`,
  {},
);

/*
plaintext: `👋🏽 Hello You

- alpha

- beta

Example Link (https://example.com/)
`
*/
```

    c. Parse MJML (email template) to HTML

```typescript
import init, {
  markdown_to_html,
  markdown_to_plaintext,
  mjml_to_html,
} from "pkg/parsedown.js";

await init();

const plaintext = await markdown_to_plaintext("<mjml></mjml>");

/*
plaintext: `<!doctype html><html xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office"><head><title></title><!--[if !mso]><!--><meta http-equiv="X-UA-Compatible" content="IE=edge"><!--<![endif]--><meta http-equiv="Content-Type" content="text/html; charset=UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1">
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
</head><body></body></html>`
*/
```

**You must call `init` once before using one of the functions.**

## Compile WASM in Deno project

WIP

## 🗺️ Roadmap

- and text readability statistics
