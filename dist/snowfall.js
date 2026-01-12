/*!
 * SnowFall2 v0.4.1
 * Copyright 2026 hi2ma-bu4
 * Licensed under the Apache License, Version 2.0
 * http://www.apache.org/licenses/LICENSE-2.0
 */
var __defProp = Object.defineProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};

// pkg/snowfall_core.js
var snowfall_core_exports = {};
__export(snowfall_core_exports, {
  WasmCompileResult: () => WasmCompileResult,
  compile: () => compile,
  default: () => snowfall_core_default,
  execute: () => execute,
  free_memory: () => free_memory,
  free_memory_with_len: () => free_memory_with_len,
  initSync: () => initSync,
  lexer: () => lexer,
  main_init: () => main_init,
  normalize: () => normalize,
  parser: () => parser,
  version: () => version
});
var wasm;
function addToExternrefTable0(obj) {
  const idx = wasm.__externref_table_alloc();
  wasm.__wbindgen_externrefs.set(idx, obj);
  return idx;
}
function getArrayU8FromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}
var cachedDataViewMemory0 = null;
function getDataViewMemory0() {
  if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || cachedDataViewMemory0.buffer.detached === void 0 && cachedDataViewMemory0.buffer !== wasm.memory.buffer) {
    cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
  }
  return cachedDataViewMemory0;
}
function getStringFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return decodeText(ptr, len);
}
var cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
  if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
    cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8ArrayMemory0;
}
function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    const idx = addToExternrefTable0(e);
    wasm.__wbindgen_exn_store(idx);
  }
}
function passArray8ToWasm0(arg, malloc) {
  const ptr = malloc(arg.length * 1, 1) >>> 0;
  getUint8ArrayMemory0().set(arg, ptr / 1);
  WASM_VECTOR_LEN = arg.length;
  return ptr;
}
function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === void 0) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr2 = malloc(buf.length, 1) >>> 0;
    getUint8ArrayMemory0().subarray(ptr2, ptr2 + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr2;
  }
  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0;
  const mem = getUint8ArrayMemory0();
  let offset = 0;
  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 127) break;
    mem[ptr + offset] = code;
  }
  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
    const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
    const ret = cachedTextEncoder.encodeInto(arg, view);
    offset += ret.written;
    ptr = realloc(ptr, len, offset, 1) >>> 0;
  }
  WASM_VECTOR_LEN = offset;
  return ptr;
}
function takeFromExternrefTable0(idx) {
  const value = wasm.__wbindgen_externrefs.get(idx);
  wasm.__externref_table_dealloc(idx);
  return value;
}
var cachedTextDecoder = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
var MAX_SAFARI_DECODE_BYTES = 2146435072;
var numBytesDecoded = 0;
function decodeText(ptr, len) {
  numBytesDecoded += len;
  if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
    cachedTextDecoder = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
    cachedTextDecoder.decode();
    numBytesDecoded = len;
  }
  return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}
var cachedTextEncoder = new TextEncoder();
if (!("encodeInto" in cachedTextEncoder)) {
  cachedTextEncoder.encodeInto = function(arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
      read: arg.length,
      written: buf.length
    };
  };
}
var WASM_VECTOR_LEN = 0;
var WasmCompileResultFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_wasmcompileresult_free(ptr >>> 0, 1));
var WasmCompileResult = class _WasmCompileResult {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(_WasmCompileResult.prototype);
    obj.__wbg_ptr = ptr;
    WasmCompileResultFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    WasmCompileResultFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_wasmcompileresult_free(ptr, 0);
  }
  /**
   * バイナリデータ (Uint8Array | undefined)
   * @returns {Uint8Array | undefined}
   */
  get binary() {
    const ret = wasm.wasmcompileresult_binary(this.__wbg_ptr);
    let v1;
    if (ret[0] !== 0) {
      v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
      wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    }
    return v1;
  }
  /**
   * エラーリスト (ISnowFallError[] | undefined)
   * @returns {any}
   */
  get errors() {
    const ret = wasm.wasmcompileresult_errors(this.__wbg_ptr);
    return ret;
  }
};
if (Symbol.dispose) WasmCompileResult.prototype[Symbol.dispose] = WasmCompileResult.prototype.free;
function compile(source, debug) {
  const ptr0 = passStringToWasm0(source, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.compile(ptr0, len0, debug);
  return WasmCompileResult.__wrap(ret);
}
function execute(binary) {
  const ptr0 = passArray8ToWasm0(binary, wasm.__wbindgen_malloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.execute(ptr0, len0);
  if (ret[2]) {
    throw takeFromExternrefTable0(ret[1]);
  }
  return takeFromExternrefTable0(ret[0]);
}
function free_memory(ptr, capacity) {
  wasm.free_memory(ptr, capacity);
}
function free_memory_with_len(ptr, length, capacity) {
  wasm.free_memory_with_len(ptr, length, capacity);
}
function lexer(source) {
  const ptr0 = passStringToWasm0(source, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.lexer(ptr0, len0);
  if (ret[2]) {
    throw takeFromExternrefTable0(ret[1]);
  }
  return takeFromExternrefTable0(ret[0]);
}
function main_init() {
  wasm.main_init();
}
function normalize(source) {
  const ptr0 = passStringToWasm0(source, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.normalize(ptr0, len0);
  if (ret[2]) {
    throw takeFromExternrefTable0(ret[1]);
  }
  return takeFromExternrefTable0(ret[0]);
}
function parser(source) {
  const ptr0 = passStringToWasm0(source, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.parser(ptr0, len0);
  if (ret[2]) {
    throw takeFromExternrefTable0(ret[1]);
  }
  return takeFromExternrefTable0(ret[0]);
}
function version() {
  let deferred1_0;
  let deferred1_1;
  try {
    const ret = wasm.version();
    deferred1_0 = ret[0];
    deferred1_1 = ret[1];
    return getStringFromWasm0(ret[0], ret[1]);
  } finally {
    wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
  }
}
var EXPECTED_RESPONSE_TYPES = /* @__PURE__ */ new Set(["basic", "cors", "default"]);
async function __wbg_load(module, imports) {
  if (typeof Response === "function" && module instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module, imports);
      } catch (e) {
        const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);
        if (validResponse && module.headers.get("Content-Type") !== "application/wasm") {
          console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
        } else {
          throw e;
        }
      }
    }
    const bytes = await module.arrayBuffer();
    return await WebAssembly.instantiate(bytes, imports);
  } else {
    const instance = await WebAssembly.instantiate(module, imports);
    if (instance instanceof WebAssembly.Instance) {
      return { instance, module };
    } else {
      return instance;
    }
  }
}
function __wbg_get_imports() {
  const imports = {};
  imports.wbg = {};
  imports.wbg.__wbg_Error_52673b7de5a0ca89 = function(arg0, arg1) {
    const ret = Error(getStringFromWasm0(arg0, arg1));
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_string_704ef9c8fc131030 = function(arg0) {
    const ret = typeof arg0 === "string";
    return ret;
  };
  imports.wbg.__wbg___wbindgen_throw_dd24417ed36fc46e = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
  };
  imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
      deferred0_0 = arg0;
      deferred0_1 = arg1;
      console.error(getStringFromWasm0(arg0, arg1));
    } finally {
      wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
  };
  imports.wbg.__wbg_getRandomValues_1c61fac11405ffdc = function() {
    return handleError(function(arg0, arg1) {
      globalThis.crypto.getRandomValues(getArrayU8FromWasm0(arg0, arg1));
    }, arguments);
  };
  imports.wbg.__wbg_new_1ba21ce319a06297 = function() {
    const ret = new Object();
    return ret;
  };
  imports.wbg.__wbg_new_25f239778d6112b9 = function() {
    const ret = new Array();
    return ret;
  };
  imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
    const ret = new Error();
    return ret;
  };
  imports.wbg.__wbg_new_b546ae120718850e = function() {
    const ret = /* @__PURE__ */ new Map();
    return ret;
  };
  imports.wbg.__wbg_set_3f1d0b984ed272ed = function(arg0, arg1, arg2) {
    arg0[arg1] = arg2;
  };
  imports.wbg.__wbg_set_7df433eea03a5c14 = function(arg0, arg1, arg2) {
    arg0[arg1 >>> 0] = arg2;
  };
  imports.wbg.__wbg_set_efaaf145b9377369 = function(arg0, arg1, arg2) {
    const ret = arg0.set(arg1, arg2);
    return ret;
  };
  imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
  };
  imports.wbg.__wbindgen_cast_2241b6af4c4b2941 = function(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
  };
  imports.wbg.__wbindgen_cast_4625c577ab2ec9ee = function(arg0) {
    const ret = BigInt.asUintN(64, arg0);
    return ret;
  };
  imports.wbg.__wbindgen_cast_9ae0607507abb057 = function(arg0) {
    const ret = arg0;
    return ret;
  };
  imports.wbg.__wbindgen_cast_d6cd19b81560fd6e = function(arg0) {
    const ret = arg0;
    return ret;
  };
  imports.wbg.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, void 0);
    table.set(offset + 0, void 0);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
  };
  return imports;
}
function __wbg_finalize_init(instance, module) {
  wasm = instance.exports;
  __wbg_init.__wbindgen_wasm_module = module;
  cachedDataViewMemory0 = null;
  cachedUint8ArrayMemory0 = null;
  wasm.__wbindgen_start();
  return wasm;
}
function initSync(module) {
  if (wasm !== void 0) return wasm;
  if (typeof module !== "undefined") {
    if (Object.getPrototypeOf(module) === Object.prototype) {
      ({ module } = module);
    } else {
      console.warn("using deprecated parameters for `initSync()`; pass a single object instead");
    }
  }
  const imports = __wbg_get_imports();
  if (!(module instanceof WebAssembly.Module)) {
    module = new WebAssembly.Module(module);
  }
  const instance = new WebAssembly.Instance(module, imports);
  return __wbg_finalize_init(instance, module);
}
async function __wbg_init(module_or_path) {
  if (wasm !== void 0) return wasm;
  if (typeof module_or_path !== "undefined") {
    if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
      ({ module_or_path } = module_or_path);
    } else {
      console.warn("using deprecated parameters for the initialization function; pass a single object instead");
    }
  }
  if (typeof module_or_path === "undefined") {
    module_or_path = new URL("snowfall_core_bg.wasm", import.meta.url);
  }
  const imports = __wbg_get_imports();
  if (typeof module_or_path === "string" || typeof Request === "function" && module_or_path instanceof Request || typeof URL === "function" && module_or_path instanceof URL) {
    module_or_path = fetch(module_or_path);
  }
  const { instance, module } = await __wbg_load(await module_or_path, imports);
  return __wbg_finalize_init(instance, module);
}
var snowfall_core_default = __wbg_init;

// src/common/SnowFallError.ts
var SnowFallError = class extends Error {
  type;
  code;
  line;
  column;
  trace;
  context;
  constructor(error) {
    super(error.message);
    this.name = this.constructor.name;
    this.type = error.type;
    this.code = error.code;
    this.line = error.line;
    this.column = error.column;
    this.trace = error.trace;
    this.context = error.context;
    if (Error.captureStackTrace) {
      Error.captureStackTrace(this, this.constructor);
    }
  }
};

// src/libs/compress.ts
var CHUNK = 16384;
function strFromU8Latin1(dat) {
  let out = "";
  for (let i = 0; i < dat.length; i += CHUNK) {
    out += String.fromCharCode(...dat.subarray(i, i + CHUNK));
  }
  return out;
}

// src/libs/Logger.ts
var Logger = class {
  static isDebug = false;
  static prefix = "SnowFall";
  static info(...args) {
    if (this.isDebug) console.log(`[${this.prefix}]`, ...args);
  }
  static warn(...args) {
    console.warn(`[${this.prefix}]`, ...args);
  }
  static error(...args) {
    console.error(`[${this.prefix}]`, ...args);
  }
};

// src/libs/lzbase62/src/config.ts
var HAS_TYPED = typeof Uint8Array !== "undefined" && typeof Uint16Array !== "undefined";
var canCharCodeApply = false;
try {
  if (String.fromCharCode.apply(null, [97]) === "a") {
    canCharCodeApply = true;
  }
} catch (e) {
}
var CAN_CHARCODE_APPLY = canCharCodeApply;
var canCharCodeApplyTyped = false;
if (HAS_TYPED) {
  try {
    if (String.fromCharCode.apply(null, new Uint8Array([97])) === "a") {
      canCharCodeApplyTyped = true;
    }
  } catch (e) {
  }
}
var CAN_CHARCODE_APPLY_TYPED = canCharCodeApplyTyped;
var APPLY_BUFFER_SIZE = 65533;
var APPLY_BUFFER_SIZE_OK = null;
function setApplyBufferSizeOk(value) {
  APPLY_BUFFER_SIZE_OK = value;
}
var stringLastIndexOfBug = false;
if ("abc\u307B\u3052".lastIndexOf("\u307B\u3052", 1) !== -1) {
  stringLastIndexOfBug = true;
}
var STRING_LASTINDEXOF_BUG = stringLastIndexOfBug;
var BASE62TABLE = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
var TABLE_LENGTH = BASE62TABLE.length;
var TABLE_DIFF = Math.max(TABLE_LENGTH, 62) - Math.min(TABLE_LENGTH, 62);
var BUFFER_MAX = TABLE_LENGTH - 1;
var WINDOW_MAX = 1024;
var WINDOW_BUFFER_MAX = 304;
var COMPRESS_CHUNK_SIZE = APPLY_BUFFER_SIZE;
var COMPRESS_CHUNK_MAX = COMPRESS_CHUNK_SIZE - TABLE_LENGTH;
var DECOMPRESS_CHUNK_SIZE = APPLY_BUFFER_SIZE;
var DECOMPRESS_CHUNK_MAX = DECOMPRESS_CHUNK_SIZE + WINDOW_MAX * 2;
var LATIN_CHAR_MAX = 11;
var LATIN_BUFFER_MAX = LATIN_CHAR_MAX * (LATIN_CHAR_MAX + 1);
var UNICODE_CHAR_MAX = 40;
var UNICODE_BUFFER_MAX = UNICODE_CHAR_MAX * (UNICODE_CHAR_MAX + 1);
var LATIN_INDEX = TABLE_LENGTH + 1;
var LATIN_INDEX_START = TABLE_DIFF + 20;
var UNICODE_INDEX = TABLE_LENGTH + 5;
var DECODE_MAX = TABLE_LENGTH - TABLE_DIFF - 19;
var LATIN_DECODE_MAX = UNICODE_CHAR_MAX + 7;
var CHAR_START = LATIN_DECODE_MAX + 1;
var COMPRESS_START = CHAR_START + 1;
var COMPRESS_FIXED_START = COMPRESS_START + 5;
var COMPRESS_INDEX = COMPRESS_FIXED_START + 5;

// src/libs/lzbase62/src/util.ts
var fromCharCode = String.fromCharCode;
function createBuffer(bits, size) {
  if (HAS_TYPED) {
    switch (bits) {
      case 8:
        return new Uint8Array(size);
      case 16:
        return new Uint16Array(size);
    }
  }
  return new Array(size);
}
function truncateBuffer(buffer, length) {
  if (buffer.length === length) {
    return buffer;
  }
  if (buffer.subarray) {
    return buffer.subarray(0, length);
  }
  buffer.length = length;
  return buffer;
}
function bufferToString_fast(buffer, length) {
  if (length == null) {
    length = buffer.length;
  } else {
    buffer = truncateBuffer(buffer, length);
  }
  if (CAN_CHARCODE_APPLY && CAN_CHARCODE_APPLY_TYPED) {
    const len = buffer.length;
    if (len < APPLY_BUFFER_SIZE && APPLY_BUFFER_SIZE_OK) {
      return fromCharCode.apply(null, buffer);
    }
    if (APPLY_BUFFER_SIZE_OK === null) {
      try {
        const s = fromCharCode.apply(null, buffer);
        if (len > APPLY_BUFFER_SIZE) {
          setApplyBufferSizeOk(true);
        }
        return s;
      } catch (e) {
        setApplyBufferSizeOk(false);
      }
    }
  }
  return bufferToString_chunked(buffer);
}
function bufferToString_chunked(buffer) {
  let string = "";
  const length = buffer.length;
  let i = 0;
  let sub;
  while (i < length) {
    if (!Array.isArray(buffer) && buffer.subarray) {
      sub = buffer.subarray(i, i + APPLY_BUFFER_SIZE);
    } else {
      sub = buffer.slice(i, i + APPLY_BUFFER_SIZE);
    }
    i += APPLY_BUFFER_SIZE;
    if (APPLY_BUFFER_SIZE_OK) {
      string += fromCharCode.apply(null, sub);
      continue;
    }
    if (APPLY_BUFFER_SIZE_OK === null) {
      try {
        string += fromCharCode.apply(null, sub);
        if (sub.length > APPLY_BUFFER_SIZE) {
          setApplyBufferSizeOk(true);
        }
        continue;
      } catch (e) {
        setApplyBufferSizeOk(false);
      }
    }
    return bufferToString_slow(buffer);
  }
  return string;
}
function bufferToString_slow(buffer) {
  let string = "";
  const length = buffer.length;
  for (let i = 0; i < length; i++) {
    string += fromCharCode(buffer[i]);
  }
  return string;
}
function createWindow() {
  let i = WINDOW_MAX >> 7;
  let win = "        ";
  while (!(i & WINDOW_MAX)) {
    win += win;
    i <<= 1;
  }
  return win;
}

// src/libs/lzbase62/src/compressor.ts
var Compressor = class {
  _data = null;
  _table = null;
  _result = null;
  _onDataCallback;
  _onEndCallback;
  _offset = 0;
  _dataLen = 0;
  _index = 0;
  _length = 0;
  /**
   * @param {CompressorOptions} [options] - Compression options.
   */
  constructor(options) {
    this._init(options);
  }
  /**
   * Initializes or re-initializes the compressor's state.
   * @private
   * @param {CompressorOptions} [options] - Compression options.
   */
  _init(options) {
    options = options || {};
    this._data = null;
    this._table = null;
    this._result = null;
    this._onDataCallback = options.onData;
    this._onEndCallback = options.onEnd;
  }
  /**
   * Creates the base62 lookup table for compression.
   * @private
   * @returns {BufferType} The character code table.
   */
  _createTable() {
    const table = createBuffer(8, TABLE_LENGTH);
    for (let i = 0; i < TABLE_LENGTH; i++) {
      table[i] = BASE62TABLE.charCodeAt(i);
    }
    return table;
  }
  /**
   * Handles a chunk of compressed data.
   * Either calls the onData callback or appends to the internal result string.
   * @private
   * @param {BufferType} buffer - The buffer containing the data chunk.
   * @param {number} length - The length of the data in the buffer.
   */
  _onData(buffer, length) {
    const chunk = bufferToString_fast(buffer, length);
    if (this._onDataCallback) {
      this._onDataCallback(chunk);
    } else if (this._result !== null) {
      this._result += chunk;
    }
  }
  /**
   * Finalizes the compression process.
   * @private
   */
  _onEnd() {
    if (this._onEndCallback) {
      this._onEndCallback();
    }
    this._data = this._table = null;
  }
  /**
   * Searches for the longest matching string in the sliding window.
   * @private
   * @returns {boolean} `true` if a match was found, otherwise `false`.
   */
  _search() {
    let i = 2;
    const data = this._data;
    const offset = this._offset;
    let len = BUFFER_MAX;
    if (this._dataLen - offset < len) {
      len = this._dataLen - offset;
    }
    if (i > len) {
      return false;
    }
    const pos = offset - WINDOW_BUFFER_MAX;
    const win = data.substring(pos, offset + len);
    const limit = offset + i - 3 - pos;
    let j, s, index, lastIndex, bestIndex, winPart;
    do {
      if (i === 2) {
        s = data.charAt(offset) + data.charAt(offset + 1);
        index = win.indexOf(s);
        if (index === -1 || index > limit) {
          break;
        }
      } else if (i === 3) {
        s = s + data.charAt(offset + 2);
      } else {
        s = data.substr(offset, i);
      }
      if (STRING_LASTINDEXOF_BUG) {
        winPart = data.substring(pos, offset + i - 1);
        lastIndex = winPart.lastIndexOf(s);
      } else {
        lastIndex = win.lastIndexOf(s, limit);
      }
      if (lastIndex === -1) {
        break;
      }
      bestIndex = lastIndex;
      j = pos + lastIndex;
      do {
        if (data.charCodeAt(offset + i) !== data.charCodeAt(j + i)) {
          break;
        }
      } while (++i < len);
      if (index === lastIndex) {
        i++;
        break;
      }
    } while (++i < len);
    if (i === 2) {
      return false;
    }
    this._index = WINDOW_BUFFER_MAX - bestIndex;
    this._length = i - 1;
    return true;
  }
  /**
   * Compresses the input data string.
   * @param {string | null} data - The string data to compress.
   * @returns {string} The compressed data as a base62 encoded string.
   */
  compress(data) {
    if (data == null || data.length === 0) {
      return "";
    }
    let result = "";
    const table = this._createTable();
    let win = createWindow();
    const buffer = createBuffer(8, COMPRESS_CHUNK_SIZE);
    let i = 0;
    this._result = "";
    this._offset = win.length;
    this._data = win + data;
    this._dataLen = this._data.length;
    let index = -1;
    let lastIndex = -1;
    let c, c1, c2, c3, c4;
    while (this._offset < this._dataLen) {
      if (!this._search()) {
        c = this._data.charCodeAt(this._offset++);
        if (c < LATIN_BUFFER_MAX) {
          if (c < UNICODE_CHAR_MAX) {
            c1 = c;
            index = LATIN_INDEX;
          } else {
            c1 = c % UNICODE_CHAR_MAX;
            c2 = (c - c1) / UNICODE_CHAR_MAX;
            index = c2 + LATIN_INDEX;
          }
          if (lastIndex === index) {
            buffer[i++] = table[c1];
          } else {
            buffer[i++] = table[index - LATIN_INDEX_START];
            buffer[i++] = table[c1];
            lastIndex = index;
          }
        } else {
          if (c < UNICODE_BUFFER_MAX) {
            index = UNICODE_INDEX;
            c1 = c;
          } else {
            c1 = c % UNICODE_BUFFER_MAX;
            c2 = (c - c1) / UNICODE_BUFFER_MAX;
            index = c2 + UNICODE_INDEX;
          }
          if (c1 < UNICODE_CHAR_MAX) {
            c3 = c1;
            c4 = 0;
          } else {
            c3 = c1 % UNICODE_CHAR_MAX;
            c4 = (c1 - c3) / UNICODE_CHAR_MAX;
          }
          if (lastIndex === index) {
            buffer[i++] = table[c3];
            buffer[i++] = table[c4];
          } else {
            buffer[i++] = table[CHAR_START];
            buffer[i++] = table[index - TABLE_LENGTH];
            buffer[i++] = table[c3];
            buffer[i++] = table[c4];
            lastIndex = index;
          }
        }
      } else {
        if (this._index < BUFFER_MAX) {
          c1 = this._index;
          c2 = 0;
        } else {
          c1 = this._index % BUFFER_MAX;
          c2 = (this._index - c1) / BUFFER_MAX;
        }
        if (this._length === 2) {
          buffer[i++] = table[c2 + COMPRESS_FIXED_START];
          buffer[i++] = table[c1];
        } else {
          buffer[i++] = table[c2 + COMPRESS_START];
          buffer[i++] = table[c1];
          buffer[i++] = table[this._length];
        }
        this._offset += this._length;
        if (~lastIndex) {
          lastIndex = -1;
        }
      }
      if (i >= COMPRESS_CHUNK_MAX) {
        this._onData(buffer, i);
        i = 0;
      }
    }
    if (i > 0) {
      this._onData(buffer, i);
    }
    this._onEnd();
    result = this._result;
    this._result = null;
    return result === null ? "" : result;
  }
};

// src/libs/lzbase62/src/index.ts
function compress(data, options) {
  return new Compressor(options).compress(data);
}

// src/libs/version_check.ts
function parseSemVer(v) {
  const m = /^v?(\d+)\.(\d+)\.(\d+)$/.exec(v);
  if (!m) return null;
  return {
    major: Number(m[1]),
    minor: Number(m[2]),
    patch: Number(m[3])
  };
}
function compareVersion(tsV, rustV) {
  if (tsV.major !== rustV.major) {
    return {
      kind: "err",
      message: `MAJOR mismatch: ts=${tsV.major}, rust=${rustV.major}`
    };
  }
  if (tsV.minor !== rustV.minor) {
    return {
      kind: "err",
      message: `MINOR mismatch: ts=${tsV.minor}, rust=${rustV.minor}`
    };
  }
  if (tsV.patch !== rustV.patch) {
    return {
      kind: "warn",
      message: `PATCH mismatch: ts=${tsV.patch}, rust=${rustV.patch}`
    };
  }
  return { kind: "ok" };
}

// src/version.ts
var VERSION = "v0.4.1";

// src/snowfall.ts
var SnowFall = class {
  _wasm = null;
  _isInitialized = false;
  constructor(isDebug = false) {
    Logger.isDebug = isDebug;
  }
  async init(wasmPath) {
    if (this._isInitialized) return;
    try {
      await snowfall_core_default(wasmPath);
    } catch (error) {
      Logger.error("Failed to initialize SnowFall Wasm module:", error);
      throw error;
    }
    this._wasm = snowfall_core_exports;
    this._versionCheck();
    this._isInitialized = true;
    Logger.info("SnowFall Wasm module initialized successfully.");
  }
  ensureInitialized() {
    if (!this._wasm || !this._isInitialized) {
      throw new Error("SnowFall has not been initialized. Please call init() first.");
    }
    return this._wasm;
  }
  /* ================================================== */
  /* 管理情報 */
  /* ================================================== */
  /**
   * ts(js)ライブラリのバージョン取得
   */
  version() {
    return VERSION;
  }
  /**
   * rust(wasm)ライブラリのバージョン取得
   */
  version_wasm() {
    const wasm2 = this.ensureInitialized();
    return wasm2.version();
  }
  /* ================================================== */
  /* 公開機能 */
  /* ================================================== */
  /**
   * sfソースコードをコンパイルする
   * @param input ソースコードの文字列
   * @param debug ソースマップを追加するか
   * @returns バイナリデータなど
   */
  compile_bin(input, debug) {
    const wasm2 = this.ensureInitialized();
    const result = wasm2.compile(input, debug);
    try {
      const errorsVal = result.errors;
      if (errorsVal) {
        const errors = errorsVal;
        if (Array.isArray(errors) && errors.length > 0) {
          return {
            errors: errors.map((err) => new SnowFallError(err))
          };
        }
      }
      const binary = result.binary;
      if (binary) {
        return { binary };
      }
      return {};
    } finally {
      result.free();
    }
  }
  /**
   * sfソースコードをコンパイルする
   * @param input ソースコードの文字列
   * @param debug ソースマップを追加するか
   * @returns テキストデータなど
   */
  compile(input, debug) {
    const result = this.compile_bin(input, debug);
    if (result.errors) {
      return {
        errors: result.errors
      };
    }
    if (result.binary) {
      return {
        data: compress(strFromU8Latin1(result.binary))
      };
    }
    return {};
  }
  /**
   * コンパイル済みのバイナリデータを実行する
   * @param binary コンパイル済みのバイナリデータ
   * @returns 実行結果
   */
  execute(binary) {
    const wasm2 = this.ensureInitialized();
    try {
      const result = wasm2.execute(binary);
      return { value: result };
    } catch (e) {
      console.error(e);
      return { error: e };
    }
  }
  /* ================================================== */
  /* デバッグ用機能 */
  /* ================================================== */
  /**
   * デバッグ用のLexer関数
   * @param input ソースコードの文字列
   * @returns トークンの配列
   * @deprecated 開発・デバッグ用の関数です。本番環境では使用しないでください
   */
  dev_lexer(input) {
    const wasm2 = this.ensureInitialized();
    return wasm2.lexer(input);
  }
  /**
   * デバッグ用のParser(Lexer含む)関数
   * @param input ソースコードの文字列
   * @returns トークンの配列
   * @deprecated 開発・デバッグ用の関数です。本番環境では使用しないでください
   */
  dev_parser(input) {
    const wasm2 = this.ensureInitialized();
    const result = wasm2.parser(input);
    if (result.errors) {
      return {
        ...result,
        errors: result.errors.map((err) => new SnowFallError(err))
      };
    }
    return result;
  }
  /**
   * デバッグ用のParser(normalize済)関数
   * @param input ソースコードの文字列
   * @returns トークンの配列
   * @deprecated 開発・デバッグ用の関数です。本番環境では使用しないでください
   */
  dev_normalize(input) {
    const wasm2 = this.ensureInitialized();
    const result = wasm2.normalize(input);
    if (result.errors) {
      return {
        ...result,
        errors: result.errors.map((err) => new SnowFallError(err))
      };
    }
    return result;
  }
  /* ================================================== */
  /* 共通利用 */
  /* ================================================== */
  /**
   * バージョンチェック
   * @throws {Error}
   */
  _versionCheck() {
    if (!this._wasm) return;
    const tsVer = parseSemVer(this.version());
    const rustVer = parseSemVer(this._wasm.version());
    if (!tsVer || !rustVer) {
      throw new Error("Invalid version format (expected x.y.z)");
    }
    const result = compareVersion(tsVer, rustVer);
    switch (result.kind) {
      case "ok":
        return;
      case "warn":
        Logger.warn("[Version]", result.message);
        return;
      case "err":
        throw new Error(`[Version] ${result.message}`);
    }
  }
};
export {
  SnowFall
};
/*!
 * lzbase62 v2.0.0 - LZ77(LZSS) based compression algorithm in base62 for JavaScript
 * Copyright (c) 2014-2020 polygon planet <polygon.planet.aqua@gmail.com>
 * https://github.com/polygonplanet/lzbase62
 * @license MIT
 *
 * Forked and modified by SnowFall2 Project
 * Modifications:
 * - Converted from JavaScript to TypeScript
 * - Added type definitions
 */
/**
 * forked from lzbase62
 * @module lzbase62
 * @see https://github.com/polygonplanet/lzbase62
 * @license MIT
 * @version 2.0.0
 */
//# sourceMappingURL=snowfall.js.map
