/*!
 * SnowFall2 v0.0.4
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
  allocate_memory: () => allocate_memory,
  default: () => snowfall_core_default,
  free_memory: () => free_memory,
  initSync: () => initSync,
  lexer: () => lexer,
  main_init: () => main_init,
  version: () => version
});
var wasm;
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
function allocate_memory(size) {
  const ret = wasm.allocate_memory(size);
  return ret >>> 0;
}
function free_memory(ptr, size) {
  wasm.free_memory(ptr, size);
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
  imports.wbg.__wbg_String_8f0eb39a4a4c2f66 = function(arg0, arg1) {
    const ret = String(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
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
  imports.wbg.__wbg_set_3f1d0b984ed272ed = function(arg0, arg1, arg2) {
    arg0[arg1] = arg2;
  };
  imports.wbg.__wbg_set_7df433eea03a5c14 = function(arg0, arg1, arg2) {
    arg0[arg1 >>> 0] = arg2;
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
var VERSION = "v0.0.4";

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
  /* デバッグ用機能 */
  /* ================================================== */
  /**
   * デバッグ用のLexer関数
   * @param input ソースコードの文字列
   * @returns トークンの配列
   * @deprecated 開発・デバッグ用の関数です。本番環境では使用しないでください。
   */
  dev_lexer(input) {
    const wasm2 = this.ensureInitialized();
    return wasm2.lexer(input);
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
//# sourceMappingURL=snowfall.js.map
