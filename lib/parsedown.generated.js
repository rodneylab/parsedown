// @generated file from wasmbuild -- do not edit
// @ts-nocheck: generated
// deno-lint-ignore-file
// deno-fmt-ignore-file
/// <reference types="./parsedown.generated.d.ts" />

// source-hash: f28def7121bbab3494416ee9b34fe483428d17ad
let wasm;

const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) {
  return heap[idx];
}

const cachedTextDecoder = typeof TextDecoder !== "undefined"
  ? new TextDecoder("utf-8", { ignoreBOM: true, fatal: true })
  : {
    decode: () => {
      throw Error("TextDecoder not available");
    },
  };

if (typeof TextDecoder !== "undefined") cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
  if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let heap_next = heap.length;

function addHeapObject(obj) {
  if (heap_next === heap.length) heap.push(heap.length + 1);
  const idx = heap_next;
  heap_next = heap[idx];

  heap[idx] = obj;
  return idx;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = typeof TextEncoder !== "undefined"
  ? new TextEncoder("utf-8")
  : {
    encode: () => {
      throw Error("TextEncoder not available");
    },
  };

const encodeString = function (arg, view) {
  return cachedTextEncoder.encodeInto(arg, view);
};

function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === undefined) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr = malloc(buf.length, 1) >>> 0;
    getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr;
  }

  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0;

  const mem = getUint8Memory0();

  let offset = 0;

  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7F) break;
    mem[ptr + offset] = code;
  }

  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);

    offset += ret.written;
    ptr = realloc(ptr, len, offset, 1) >>> 0;
  }

  WASM_VECTOR_LEN = offset;
  return ptr;
}

function isLikeNone(x) {
  return x === undefined || x === null;
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
  if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  }
  return cachedInt32Memory0;
}

function dropObject(idx) {
  if (idx < 132) return;
  heap[idx] = heap_next;
  heap_next = idx;
}

function takeObject(idx) {
  const ret = getObject(idx);
  dropObject(idx);
  return ret;
}

let cachedFloat64Memory0 = null;

function getFloat64Memory0() {
  if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
    cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
  }
  return cachedFloat64Memory0;
}

function debugString(val) {
  // primitive types
  const type = typeof val;
  if (type == "number" || type == "boolean" || val == null) {
    return `${val}`;
  }
  if (type == "string") {
    return `"${val}"`;
  }
  if (type == "symbol") {
    const description = val.description;
    if (description == null) {
      return "Symbol";
    } else {
      return `Symbol(${description})`;
    }
  }
  if (type == "function") {
    const name = val.name;
    if (typeof name == "string" && name.length > 0) {
      return `Function(${name})`;
    } else {
      return "Function";
    }
  }
  // objects
  if (Array.isArray(val)) {
    const length = val.length;
    let debug = "[";
    if (length > 0) {
      debug += debugString(val[0]);
    }
    for (let i = 1; i < length; i++) {
      debug += ", " + debugString(val[i]);
    }
    debug += "]";
    return debug;
  }
  // Test for built-in
  const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
  let className;
  if (builtInMatches.length > 1) {
    className = builtInMatches[1];
  } else {
    // Failed to match the standard '[object ClassName]'
    return toString.call(val);
  }
  if (className == "Object") {
    // we're a user defined class or Object
    // JSON.stringify avoids problems with cycles, and is generally much
    // easier than looping through ownProperties of `val`.
    try {
      return "Object(" + JSON.stringify(val) + ")";
    } catch (_) {
      return "Object";
    }
  }
  // errors
  if (val instanceof Error) {
    return `${val.name}: ${val.message}\n${val.stack}`;
  }
  // TODO we could test for more things here, like `Set`s and `Map`s.
  return className;
}
/**
 * # Panics
 *
 * Will panic if unable to parse options
 * @param {string} markdown
 * @param {any} options
 * @returns {any}
 */
export function markdown_to_html(markdown, options) {
  const ptr0 = passStringToWasm0(
    markdown,
    wasm.__wbindgen_malloc,
    wasm.__wbindgen_realloc,
  );
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.markdown_to_html(ptr0, len0, addHeapObject(options));
  return takeObject(ret);
}

/**
 * # Panics
 *
 * Will panic if unable to parse options
 * @param {string} markdown
 * @param {any} options
 * @returns {string}
 */
export function markdown_to_plaintext(markdown, options) {
  let deferred2_0;
  let deferred2_1;
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passStringToWasm0(
      markdown,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.markdown_to_plaintext(retptr, ptr0, len0, addHeapObject(options));
    var r0 = getInt32Memory0()[retptr / 4 + 0];
    var r1 = getInt32Memory0()[retptr / 4 + 1];
    deferred2_0 = r0;
    deferred2_1 = r1;
    return getStringFromWasm0(r0, r1);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
    wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
  }
}

/**
 * @param {string} mjml
 * @returns {string}
 */
export function mjml_to_html(mjml) {
  let deferred2_0;
  let deferred2_1;
  try {
    const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
    const ptr0 = passStringToWasm0(
      mjml,
      wasm.__wbindgen_malloc,
      wasm.__wbindgen_realloc,
    );
    const len0 = WASM_VECTOR_LEN;
    wasm.mjml_to_html(retptr, ptr0, len0);
    var r0 = getInt32Memory0()[retptr / 4 + 0];
    var r1 = getInt32Memory0()[retptr / 4 + 1];
    deferred2_0 = r0;
    deferred2_1 = r1;
    return getStringFromWasm0(r0, r1);
  } finally {
    wasm.__wbindgen_add_to_stack_pointer(16);
    wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
  }
}

const imports = {
  __wbindgen_placeholder__: {
    __wbg_new_72fb9a18b5ae2624: function () {
      const ret = new Object();
      return addHeapObject(ret);
    },
    __wbg_set_f975102236d3c502: function (arg0, arg1, arg2) {
      getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
    },
    __wbg_new_16b304a2cfa7ff4a: function () {
      const ret = new Array();
      return addHeapObject(ret);
    },
    __wbg_set_d4638f722068f043: function (arg0, arg1, arg2) {
      getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
    },
    __wbindgen_is_object: function (arg0) {
      const val = getObject(arg0);
      const ret = typeof val === "object" && val !== null;
      return ret;
    },
    __wbg_getwithrefkey_edc2c8960f0f1191: function (arg0, arg1) {
      const ret = getObject(arg0)[getObject(arg1)];
      return addHeapObject(ret);
    },
    __wbindgen_is_undefined: function (arg0) {
      const ret = getObject(arg0) === undefined;
      return ret;
    },
    __wbindgen_in: function (arg0, arg1) {
      const ret = getObject(arg0) in getObject(arg1);
      return ret;
    },
    __wbindgen_boolean_get: function (arg0) {
      const v = getObject(arg0);
      const ret = typeof v === "boolean" ? (v ? 1 : 0) : 2;
      return ret;
    },
    __wbindgen_string_new: function (arg0, arg1) {
      const ret = getStringFromWasm0(arg0, arg1);
      return addHeapObject(ret);
    },
    __wbindgen_number_new: function (arg0) {
      const ret = arg0;
      return addHeapObject(ret);
    },
    __wbindgen_string_get: function (arg0, arg1) {
      const obj = getObject(arg1);
      const ret = typeof obj === "string" ? obj : undefined;
      var ptr1 = isLikeNone(ret)
        ? 0
        : passStringToWasm0(
          ret,
          wasm.__wbindgen_malloc,
          wasm.__wbindgen_realloc,
        );
      var len1 = WASM_VECTOR_LEN;
      getInt32Memory0()[arg0 / 4 + 1] = len1;
      getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    },
    __wbg_log_24068652cee20220: function (arg0, arg1) {
      console.log(getStringFromWasm0(arg0, arg1));
    },
    __wbg_length_c20a40f15020d68a: function (arg0) {
      const ret = getObject(arg0).length;
      return ret;
    },
    __wbindgen_memory: function () {
      const ret = wasm.memory;
      return addHeapObject(ret);
    },
    __wbg_buffer_12d079cc21e14bdb: function (arg0) {
      const ret = getObject(arg0).buffer;
      return addHeapObject(ret);
    },
    __wbg_new_63b92bc8671ed464: function (arg0) {
      const ret = new Uint8Array(getObject(arg0));
      return addHeapObject(ret);
    },
    __wbg_set_a47bac70306a19a7: function (arg0, arg1, arg2) {
      getObject(arg0).set(getObject(arg1), arg2 >>> 0);
    },
    __wbindgen_object_drop_ref: function (arg0) {
      takeObject(arg0);
    },
    __wbindgen_error_new: function (arg0, arg1) {
      const ret = new Error(getStringFromWasm0(arg0, arg1));
      return addHeapObject(ret);
    },
    __wbindgen_jsval_loose_eq: function (arg0, arg1) {
      const ret = getObject(arg0) == getObject(arg1);
      return ret;
    },
    __wbindgen_number_get: function (arg0, arg1) {
      const obj = getObject(arg1);
      const ret = typeof obj === "number" ? obj : undefined;
      getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
      getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
    },
    __wbg_instanceof_Uint8Array_2b3bbecd033d19f6: function (arg0) {
      let result;
      try {
        result = getObject(arg0) instanceof Uint8Array;
      } catch (_) {
        result = false;
      }
      const ret = result;
      return ret;
    },
    __wbg_instanceof_ArrayBuffer_836825be07d4c9d2: function (arg0) {
      let result;
      try {
        result = getObject(arg0) instanceof ArrayBuffer;
      } catch (_) {
        result = false;
      }
      const ret = result;
      return ret;
    },
    __wbindgen_object_clone_ref: function (arg0) {
      const ret = getObject(arg0);
      return addHeapObject(ret);
    },
    __wbindgen_debug_string: function (arg0, arg1) {
      const ret = debugString(getObject(arg1));
      const ptr1 = passStringToWasm0(
        ret,
        wasm.__wbindgen_malloc,
        wasm.__wbindgen_realloc,
      );
      const len1 = WASM_VECTOR_LEN;
      getInt32Memory0()[arg0 / 4 + 1] = len1;
      getInt32Memory0()[arg0 / 4 + 0] = ptr1;
    },
    __wbindgen_throw: function (arg0, arg1) {
      throw new Error(getStringFromWasm0(arg0, arg1));
    },
  },
};

class WasmBuildLoader {
  #options;
  #lastLoadPromise;
  #instantiated;

  constructor(options) {
    this.#options = options;
  }

  get instance() {
    return this.#instantiated?.instance;
  }

  get module() {
    return this.#instantiated?.module;
  }

  load(
    url,
    decompress,
  ) {
    if (this.#instantiated) {
      return Promise.resolve(this.#instantiated);
    } else if (this.#lastLoadPromise == null) {
      this.#lastLoadPromise = (async () => {
        try {
          this.#instantiated = await this.#instantiate(url, decompress);
          return this.#instantiated;
        } finally {
          this.#lastLoadPromise = undefined;
        }
      })();
    }
    return this.#lastLoadPromise;
  }

  async #instantiate(url, decompress) {
    const imports = this.#options.imports;
    if (this.#options.cache != null && url.protocol !== "file:") {
      try {
        const result = await this.#options.cache(
          url,
          decompress ?? ((bytes) => bytes),
        );
        if (result instanceof URL) {
          url = result;
          decompress = undefined; // already decompressed
        } else if (result != null) {
          return WebAssembly.instantiate(result, imports);
        }
      } catch {
        // ignore if caching ever fails (ex. when on deploy)
      }
    }

    const isFile = url.protocol === "file:";

    // make file urls work in Node via dnt
    const isNode = globalThis.process?.versions?.node != null;
    if (isFile && typeof Deno !== "object") {
      throw new Error(
        "Loading local files are not supported in this environment",
      );
    }
    if (isNode && isFile) {
      // the deno global will be shimmed by dnt
      const wasmCode = await Deno.readFile(url);
      return WebAssembly.instantiate(
        decompress ? decompress(wasmCode) : wasmCode,
        imports,
      );
    }

    switch (url.protocol) {
      case "file:":
      case "https:":
      case "http:": {
        const wasmResponse = await fetchWithRetries(url);
        if (decompress) {
          const wasmCode = new Uint8Array(await wasmResponse.arrayBuffer());
          return WebAssembly.instantiate(decompress(wasmCode), imports);
        }
        if (
          isFile ||
          wasmResponse.headers.get("content-type")?.toLowerCase()
            .startsWith("application/wasm")
        ) {
          return WebAssembly.instantiateStreaming(wasmResponse, imports);
        } else {
          return WebAssembly.instantiate(
            await wasmResponse.arrayBuffer(),
            imports,
          );
        }
      }
      default:
        throw new Error(`Unsupported protocol: ${url.protocol}`);
    }
  }
}
const isNodeOrDeno = typeof Deno === "object" ||
  (typeof process !== "undefined" && process.versions != null &&
    process.versions.node != null);

const loader = new WasmBuildLoader({
  imports,
  cache: isNodeOrDeno ? cacheToLocalDir : undefined,
});

export async function instantiate(opts) {
  return (await instantiateWithInstance(opts)).exports;
}

export async function instantiateWithInstance(opts) {
  const { instance } = await loader.load(
    opts?.url ?? new URL("parsedown_bg.wasm", import.meta.url),
    opts?.decompress,
  );
  wasm = wasm ?? instance.exports;
  cachedInt32Memory0 = cachedInt32Memory0 ?? new Int32Array(wasm.memory.buffer);
  cachedUint8Memory0 = cachedUint8Memory0 ?? new Uint8Array(wasm.memory.buffer);
  return {
    instance,
    exports: getWasmInstanceExports(),
  };
}

function getWasmInstanceExports() {
  return { markdown_to_html, markdown_to_plaintext, mjml_to_html };
}

export function isInstantiated() {
  return loader.instance != null;
}
export async function cacheToLocalDir(url, decompress) {
  const localPath = await getUrlLocalPath(url);
  if (localPath == null) {
    return undefined;
  }
  if (!await exists(localPath)) {
    const fileBytes = decompress(new Uint8Array(await getUrlBytes(url)));
    try {
      await Deno.writeFile(localPath, fileBytes);
    } catch {
      // ignore and return the wasm bytes
      return fileBytes;
    }
  }
  return toFileUrl(localPath);
}
async function getUrlLocalPath(url) {
  try {
    const dataDirPath = await getInitializedLocalDataDirPath();
    const hash = await getUrlHash(url);
    return `${dataDirPath}/${hash}.wasm`;
  } catch {
    return undefined;
  }
}
async function getInitializedLocalDataDirPath() {
  const dataDir = localDataDir();
  if (dataDir == null) {
    throw new Error(`Could not find local data directory.`);
  }
  const dirPath = `${dataDir}/deno-wasmbuild`;
  await ensureDir(dirPath);
  return dirPath;
}
async function exists(filePath) {
  try {
    await Deno.lstat(filePath);
    return true;
  } catch (error) {
    if (error instanceof Deno.errors.NotFound) {
      return false;
    }
    throw error;
  }
}
async function ensureDir(dir) {
  try {
    const fileInfo = await Deno.lstat(dir);
    if (!fileInfo.isDirectory) {
      throw new Error(`Path was not a directory '${dir}'`);
    }
  } catch (err) {
    if (err instanceof Deno.errors.NotFound) {
      // if dir not exists. then create it.
      await Deno.mkdir(dir, { recursive: true });
      return;
    }
    throw err;
  }
}
async function getUrlHash(url) {
  // Taken from MDN: https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest
  const hashBuffer = await crypto.subtle.digest(
    "SHA-256",
    new TextEncoder().encode(url.href),
  );
  // convert buffer to byte array
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  // convert bytes to hex string
  const hashHex = hashArray
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
  return hashHex;
}
async function getUrlBytes(url) {
  const response = await fetchWithRetries(url);
  return await response.arrayBuffer();
}
// the below is extracted from deno_std/path
const WHITESPACE_ENCODINGS = {
  "\u0009": "%09",
  "\u000A": "%0A",
  "\u000B": "%0B",
  "\u000C": "%0C",
  "\u000D": "%0D",
  "\u0020": "%20",
};
function encodeWhitespace(string) {
  return string.replaceAll(/[\s]/g, (c) => {
    return WHITESPACE_ENCODINGS[c] ?? c;
  });
}
function toFileUrl(path) {
  return Deno.build.os === "windows"
    ? windowsToFileUrl(path)
    : posixToFileUrl(path);
}
function posixToFileUrl(path) {
  const url = new URL("file:///");
  url.pathname = encodeWhitespace(
    path.replace(/%/g, "%25").replace(/\\/g, "%5C"),
  );
  return url;
}
function windowsToFileUrl(path) {
  const [, hostname, pathname] = path.match(
    /^(?:[/\\]{2}([^/\\]+)(?=[/\\](?:[^/\\]|$)))?(.*)/,
  );
  const url = new URL("file:///");
  url.pathname = encodeWhitespace(pathname.replace(/%/g, "%25"));
  if (hostname != null && hostname != "localhost") {
    url.hostname = hostname;
    if (!url.hostname) {
      throw new TypeError("Invalid hostname.");
    }
  }
  return url;
}
export async function fetchWithRetries(url, maxRetries = 5) {
  let sleepMs = 250;
  let iterationCount = 0;
  while (true) {
    iterationCount++;
    try {
      const res = await fetch(url);
      if (res.ok || iterationCount > maxRetries) {
        return res;
      }
    } catch (err) {
      if (iterationCount > maxRetries) {
        throw err;
      }
    }
    console.warn(`Failed fetching. Retrying in ${sleepMs}ms...`);
    await new Promise((resolve) => setTimeout(resolve, sleepMs));
    sleepMs = Math.min(sleepMs * 2, 10000);
  }
}
// MIT License - Copyright (c) justjavac.
// https://github.com/justjavac/deno_dirs/blob/e8c001bbef558f08fd486d444af391729b0b8068/data_local_dir/mod.ts
function localDataDir() {
  switch (Deno.build.os) {
    case "linux": {
      const xdg = Deno.env.get("XDG_DATA_HOME");
      if (xdg) {
        return xdg;
      }
      const home = Deno.env.get("HOME");
      if (home) {
        return `${home}/.local/share`;
      }
      break;
    }
    case "darwin": {
      const home = Deno.env.get("HOME");
      if (home) {
        return `${home}/Library/Application Support`;
      }
      break;
    }
    case "windows":
      return Deno.env.get("LOCALAPPDATA") ?? undefined;
  }
  return undefined;
}
