<img src="./images/rodneylab-github-parsedown.png" alt="Rodney Lab parse down Git Hub banner">

<p align="center" style="display:grid;place-items:center;margin-block:2rem">
  <a aria-label="Open Rodney Lab site" href="https://rodneylab.com" rel="nofollow noopener noreferrer">
    <img alt="Rodney Lab logo" src="https://rodneylab.com/assets/icon.png" width="60" />
  </a>
</p>
<h1 align="center">
  parsedown
</h1>

Light touch Markdown parsing into HTML written in Rust. Generates WASM and can
be used with Deno Fresh.

- adds an id and anchor link to each h2 heading for easy linking
- adds pretty punctuation
- uses html5ever for HTML manipulation and pulldown-cmark for Markdown parsing

## Using Module

The module is hosted on deno.x, and you can import directly into your TypeScript
project (no need to touch WASM or Rust source). See the later sections below if
you want to compile the WASM yourself.

- Parse Markdown to HTML

```typescript
import {
  markdownToHtml,
  markdownToPlaintext,
  mjmlToHtml,
} from "https://deno.land/x/parsedown@1.4.3/mod.ts";

const { errors, headings, html, statistics } = await markdownToHtml(
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

```javascript
const { html } = await markdownToHtml(
  "Nobody likes maple in their apple flavoured Snapple.",
  { searchTerm: "apple" },
);

html:
`<p>Nobody likes maple in their <mark id="search-match">apple</mark> flavoured Sn<mark>apple</mark></p>`;
```

Note the `id` added to the first search match. You can use this to scroll the
first match into view.

- Parse Markdown to Plain Text

```typescript
import {
  markdownToHtml,
  markdownToPlaintext,
  mjmlToHtml,
} from "https://deno.land/x/parsedown@1.4.3/mod.ts";

const plaintext = await markdownToPlaintext(
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

- Parse MJML (email template) to HTML

```typescript
import {
  markdownToHtml,
  markdownToPlaintext,
  mjmlToHtml,
} from "https://deno.land/x/parsedown@1.4.3/mod.ts";

const html = await mjmlToHtml("<mjml lang="en-GB"></mjml>");

/*
plaintext: `<!doctype html><html lang="en-GB" dir="auto" xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office"><head><title></title><!--[if !mso]><!--><meta http-equiv="X-UA-Compatible" content="IE=edge"><!--<![endif]--><meta http-equiv="Content-Type" content="text/html; charset=UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1">
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

## Compile WASM (see next section instead, if working with Deno)

_Method above is tested with Deno, you only need to compile the WASM yourself if
you have issues in other runtimes or are customizing the Rust source code for
your own needs._

1. Clone the project and change into the project directory. Then run these
   commands:

```shell
cargo install wasm-pack # skip if you already have it installed
wasm-pack build --target web
```

2. Copy the generated `pkg` folder into your JavaScript or TypeScript project.
3. Import and use the code in one of your project source files (expected output
   is as shown in previous section):

- Parse Markdown to HTML

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
```

- Parse Markdown to Plain Text

```typescript
import init, {
  markdown_to_html,
  markdown_to_plaintext,
  mjml_to_html,
} from "pkg/parsedown.js";

await init();

const plaintext = markdown_to_plaintext(
  `
## 👋🏽 Hello You

* alpha
* beta

[Example Link](https://example.com/)
`,
  {},
);
```

- Parse MJML (email template) to HTML

```typescript
import init, {
  markdown_to_html,
  markdown_to_plaintext,
  mjml_to_html,
} from "pkg/parsedown.js";

await init();

const html = await mjml_to_html("<mjml></mjml>");
```

**You must call `init` once before using any of the other functions.**

## Compile WASM in Deno project

If you are working in Deno, you will probably find the `wasmbuild` package
useful.

1. Add a `wasmbuild` task to your `deno.json` file:

```json5
{
  "tasks": {
    // ...TRUNCATED
    "wasmbuild": "deno run -A https://deno.land/x/wasmbuild@0.10.2/main.ts"
  }
  // TRUNCATED...
}
```

2. Run `deno task wasmbuild new` to initialize `wasmbuild` in your project. This
   will create a skeleton WASM project with an `rs_lib` directory.

3. Clone this repo and replace the contents of the `rs_lib/src` directory in
   your project with the contents of this repo&rsquo;s `src` directory. Also
   replace `rs_lib/Cargo.toml` with `Cargo.toml` from this repo.

4. Run the `deno task wasmbuild` command. This will generate JavaScript code and
   WASM modules from the Rust source code. In particular, there should now be
   `lib/rs_lib_bg.wasm` and `lib/rs_lib.generated.js` files in your project.

5. You can now use the library functions in your JavaScript or TypeScript code.
   Usage is only slightly different from the descriptions above.

   - You can import the functions from `lib/rs_lib.generated.js`:

   ```typescript
   import {
     instantiate,
     markdown_to_html,
     markdown_to_plaintext,
     mjml_to_html,
   } from "@/lib/rs_lib.generated.js";
   ```

   - **Before using any of the functions call `instantiate`**:

   ```typescript
   await instantiate();
   ```

   - The functions will now work as above:

   ```typescript
   const { errors, headings, html, statistics } = await markdown_to_html(
     `
   ## 👋🏽 Hello You

   * alpha
   * beta
    `,
     {},
   );

   const plaintext = markdown_to_plaintext(
     `
   ## 👋🏽 Hello You

   * alpha
   * beta

   [Example Link](https://example.com/)
   `,
     {},
   );

   const html = mjml_to_html("<mjml></mjml>");
   ```

## 🗺️ Roadmap

- add text readability statistics (Gunning Fog index, for example)

## ☎️ Reach Out

Feel free to jump into the
[Rodney Lab matrix chat room](https://matrix.to/#/%23rodney:matrix.org).
