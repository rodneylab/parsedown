// deno-lint-ignore-file
// deno-fmt-ignore-file

export interface InstantiateResult {
  instance: WebAssembly.Instance;
  exports: {
    markdown_to_html: typeof markdown_to_html;
    markdown_to_plaintext: typeof markdown_to_plaintext;
    mjml_to_html: typeof mjml_to_html
  };
}

/** Gets if the Wasm module has been instantiated. */
export function isInstantiated(): boolean;

/** Options for instantiating a Wasm instance. */
export interface InstantiateOptions {
  /** Optional url to the Wasm file to instantiate. */
  url?: URL;
  /** Callback to decompress the raw Wasm file bytes before instantiating. */
  decompress?: (bytes: Uint8Array) => Uint8Array;
}

/** Instantiates an instance of the Wasm module returning its functions.
* @remarks It is safe to call this multiple times and once successfully
* loaded it will always return a reference to the same object. */
export function instantiate(opts?: InstantiateOptions): Promise<InstantiateResult["exports"]>;

/** Instantiates an instance of the Wasm module along with its exports.
 * @remarks It is safe to call this multiple times and once successfully
 * loaded it will always return a reference to the same object. */
export function instantiateWithInstance(opts?: InstantiateOptions): Promise<InstantiateResult>;

/**
* # Panics
*
* Will panic if unable to parse options
* @param {string} markdown
* @param {any} options
* @returns {any}
*/
export function markdown_to_html(markdown: string, options: any): any;
/**
* # Panics
*
* Will panic if unable to parse options
* @param {string} markdown
* @param {any} options
* @returns {string}
*/
export function markdown_to_plaintext(markdown: string, options: any): string;
/**
* @param {string} mjml
* @returns {string}
*/
export function mjml_to_html(mjml: string): string;
