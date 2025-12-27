var __defProp = Object.defineProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};

// pkg/snowfall_core.js
var snowfall_core_exports = {};
__export(snowfall_core_exports, {
  _test_create_array_handle: () => _test_create_array_handle,
  _test_create_dictionary_handle: () => _test_create_dictionary_handle,
  _test_error_propagation: () => _test_error_propagation,
  _test_implicit_comparison: () => _test_implicit_comparison,
  _test_prototype_lookup: () => _test_prototype_lookup,
  _test_static_validation: () => _test_static_validation,
  allocate_memory: () => allocate_memory,
  default: () => snowfall_core_default,
  find_property_on_prototype: () => find_property_on_prototype,
  free_memory: () => free_memory,
  get_element_by_handle: () => get_element_by_handle,
  initSync: () => initSync,
  main_init: () => main_init,
  release_handle: () => release_handle,
  set_element_by_handle: () => set_element_by_handle
});
var wasm;
function addToExternrefTable0(obj) {
  const idx = wasm.__externref_table_alloc();
  wasm.__wbindgen_externrefs.set(idx, obj);
  return idx;
}
function debugString(val) {
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
  const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
  let className;
  if (builtInMatches && builtInMatches.length > 1) {
    className = builtInMatches[1];
  } else {
    return toString.call(val);
  }
  if (className == "Object") {
    try {
      return "Object(" + JSON.stringify(val) + ")";
    } catch (_) {
      return "Object";
    }
  }
  if (val instanceof Error) {
    return `${val.name}: ${val.message}
${val.stack}`;
  }
  return className;
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
function isLikeNone(x) {
  return x === void 0 || x === null;
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
function _test_create_array_handle() {
  const ret = wasm._test_create_array_handle();
  return ret;
}
function _test_create_dictionary_handle() {
  const ret = wasm._test_create_dictionary_handle();
  return ret;
}
function _test_error_propagation() {
  const ret = wasm._test_error_propagation();
  return ret;
}
function _test_implicit_comparison(left, right) {
  const ret = wasm._test_implicit_comparison(left, right);
  if (ret[2]) {
    throw takeFromExternrefTable0(ret[1]);
  }
  return ret[0] !== 0;
}
function _test_prototype_lookup(key) {
  const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm._test_prototype_lookup(ptr0, len0);
  return ret;
}
function _test_static_validation(type_id, property_name) {
  const ptr0 = passStringToWasm0(property_name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm._test_static_validation(type_id, ptr0, len0);
  return ret;
}
function allocate_memory(size) {
  const ret = wasm.allocate_memory(size);
  return ret >>> 0;
}
function find_property_on_prototype(obj, key) {
  const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.find_property_on_prototype(obj, ptr0, len0);
  return ret;
}
function free_memory(ptr, size) {
  wasm.free_memory(ptr, size);
}
function get_element_by_handle(handle_id, key) {
  const ret = wasm.get_element_by_handle(handle_id, key);
  return ret;
}
function main_init() {
  wasm.main_init();
}
function release_handle(handle_id) {
  wasm.release_handle(handle_id);
}
function set_element_by_handle(handle_id, key, value) {
  wasm.set_element_by_handle(handle_id, key, value);
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
  imports.wbg.__wbg_Number_2d1dcfcf4ec51736 = function(arg0) {
    const ret = Number(arg0);
    return ret;
  };
  imports.wbg.__wbg_String_8f0eb39a4a4c2f66 = function(arg0, arg1) {
    const ret = String(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
  };
  imports.wbg.__wbg___wbindgen_bigint_get_as_i64_6e32f5e6aff02e1d = function(arg0, arg1) {
    const v = arg1;
    const ret = typeof v === "bigint" ? v : void 0;
    getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
  };
  imports.wbg.__wbg___wbindgen_boolean_get_dea25b33882b895b = function(arg0) {
    const v = arg0;
    const ret = typeof v === "boolean" ? v : void 0;
    return isLikeNone(ret) ? 16777215 : ret ? 1 : 0;
  };
  imports.wbg.__wbg___wbindgen_debug_string_adfb662ae34724b6 = function(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
  };
  imports.wbg.__wbg___wbindgen_in_0d3e1e8f0c669317 = function(arg0, arg1) {
    const ret = arg0 in arg1;
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_bigint_0e1a2e3f55cfae27 = function(arg0) {
    const ret = typeof arg0 === "bigint";
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_function_8d400b8b1af978cd = function(arg0) {
    const ret = typeof arg0 === "function";
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_object_ce774f3490692386 = function(arg0) {
    const val = arg0;
    const ret = typeof val === "object" && val !== null;
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_string_704ef9c8fc131030 = function(arg0) {
    const ret = typeof arg0 === "string";
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_undefined_f6b95eab589e0269 = function(arg0) {
    const ret = arg0 === void 0;
    return ret;
  };
  imports.wbg.__wbg___wbindgen_jsval_eq_b6101cc9cef1fe36 = function(arg0, arg1) {
    const ret = arg0 === arg1;
    return ret;
  };
  imports.wbg.__wbg___wbindgen_jsval_loose_eq_766057600fdd1b0d = function(arg0, arg1) {
    const ret = arg0 == arg1;
    return ret;
  };
  imports.wbg.__wbg___wbindgen_number_get_9619185a74197f95 = function(arg0, arg1) {
    const obj = arg1;
    const ret = typeof obj === "number" ? obj : void 0;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
  };
  imports.wbg.__wbg___wbindgen_string_get_a2a31e16edf96e42 = function(arg0, arg1) {
    const obj = arg1;
    const ret = typeof obj === "string" ? obj : void 0;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
  };
  imports.wbg.__wbg___wbindgen_throw_dd24417ed36fc46e = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
  };
  imports.wbg.__wbg_call_abb4ff46ce38be40 = function() {
    return handleError(function(arg0, arg1) {
      const ret = arg0.call(arg1);
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_codePointAt_6fd4439a1e465afd = function(arg0, arg1) {
    const ret = arg0.codePointAt(arg1 >>> 0);
    return ret;
  };
  imports.wbg.__wbg_done_62ea16af4ce34b24 = function(arg0) {
    const ret = arg0.done;
    return ret;
  };
  imports.wbg.__wbg_entries_83c79938054e065f = function(arg0) {
    const ret = Object.entries(arg0);
    return ret;
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
  imports.wbg.__wbg_fromCodePoint_50facac709b76f67 = function() {
    return handleError(function(arg0) {
      const ret = String.fromCodePoint(arg0 >>> 0);
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_get_6b7bd52aca3f9671 = function(arg0, arg1) {
    const ret = arg0[arg1 >>> 0];
    return ret;
  };
  imports.wbg.__wbg_get_af9dab7e9603ea93 = function() {
    return handleError(function(arg0, arg1) {
      const ret = Reflect.get(arg0, arg1);
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_get_with_ref_key_1dc361bd10053bfe = function(arg0, arg1) {
    const ret = arg0[arg1];
    return ret;
  };
  imports.wbg.__wbg_instanceof_ArrayBuffer_f3320d2419cd0355 = function(arg0) {
    let result;
    try {
      result = arg0 instanceof ArrayBuffer;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_instanceof_Uint8Array_da54ccc9d3e09434 = function(arg0) {
    let result;
    try {
      result = arg0 instanceof Uint8Array;
    } catch (_) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_isArray_51fd9e6422c0a395 = function(arg0) {
    const ret = Array.isArray(arg0);
    return ret;
  };
  imports.wbg.__wbg_isSafeInteger_ae7d3f054d55fa16 = function(arg0) {
    const ret = Number.isSafeInteger(arg0);
    return ret;
  };
  imports.wbg.__wbg_iterator_27b7c8b35ab3e86b = function() {
    const ret = Symbol.iterator;
    return ret;
  };
  imports.wbg.__wbg_length_1f83b8e5895c84aa = function(arg0) {
    const ret = arg0.length;
    return ret;
  };
  imports.wbg.__wbg_length_22ac23eaec9d8053 = function(arg0) {
    const ret = arg0.length;
    return ret;
  };
  imports.wbg.__wbg_length_d45040a40c570362 = function(arg0) {
    const ret = arg0.length;
    return ret;
  };
  imports.wbg.__wbg_new_1ba21ce319a06297 = function() {
    const ret = new Object();
    return ret;
  };
  imports.wbg.__wbg_new_25f239778d6112b9 = function() {
    const ret = new Array();
    return ret;
  };
  imports.wbg.__wbg_new_6421f6084cc5bc5a = function(arg0) {
    const ret = new Uint8Array(arg0);
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
  imports.wbg.__wbg_next_138a17bbf04e926c = function(arg0) {
    const ret = arg0.next;
    return ret;
  };
  imports.wbg.__wbg_next_3cfe5c0fe2a4cc53 = function() {
    return handleError(function(arg0) {
      const ret = arg0.next();
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_prototypesetcall_dfe9b766cdc1f1fd = function(arg0, arg1, arg2) {
    Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
  };
  imports.wbg.__wbg_set_3f1d0b984ed272ed = function(arg0, arg1, arg2) {
    arg0[arg1] = arg2;
  };
  imports.wbg.__wbg_set_781438a03c0c3c81 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = Reflect.set(arg0, arg1, arg2);
      return ret;
    }, arguments);
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
  imports.wbg.__wbg_value_57b7b035e117f7ee = function(arg0) {
    const ret = arg0.value;
    return ret;
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

// src/snowfall.ts
var SnowFallArrayProxy = class {
  handle;
  wasm;
  constructor(handle, wasm2) {
    this.handle = handle;
    this.wasm = wasm2;
    return new Proxy(this, {
      get: (target, prop) => {
        const index = Number(prop);
        if (!isNaN(index)) {
          return target.wasm.get_element_by_handle(target.handle.id, index);
        }
        if (prop === "length") {
          return target.handle.size;
        }
        return Reflect.get(target, prop);
      },
      set: (target, prop, value) => {
        const index = Number(prop);
        if (!isNaN(index)) {
          target.wasm.set_element_by_handle(target.handle.id, index, value);
          return true;
        }
        return Reflect.set(target, prop, value);
      }
    });
  }
};
var SnowFallDictionaryProxy = class {
  handle;
  wasm;
  constructor(handle, wasm2) {
    this.handle = handle;
    this.wasm = wasm2;
    return new Proxy(this, {
      get: (target, prop) => {
        if (typeof prop === "string") {
          return target.wasm.get_element_by_handle(target.handle.id, prop);
        }
        return Reflect.get(target, prop);
      },
      set: (target, prop, value) => {
        if (typeof prop === "string") {
          target.wasm.set_element_by_handle(target.handle.id, prop, value);
          return true;
        }
        return Reflect.set(target, prop, value);
      }
    });
  }
};
var SnowFall = class {
  wasm = null;
  isInitialized = false;
  handleRegistry;
  hostFunctions;
  constructor() {
    this.handleRegistry = new FinalizationRegistry((handleId) => {
      this.wasm?.release_handle(handleId);
      console.log(`Released handle: ${handleId}`);
    });
    this.hostFunctions = /* @__PURE__ */ new Map();
    this.registerHostFunction("console.log", (...args) => console.log(...args));
  }
  async init(wasmUrl) {
    if (this.isInitialized) {
      return;
    }
    try {
      await snowfall_core_default(wasmUrl);
      this.wasm = snowfall_core_exports;
      this.isInitialized = true;
      console.log("SnowFall Wasm module initialized successfully.");
    } catch (error) {
      console.error("Failed to initialize SnowFall Wasm module:", error);
      throw error;
    }
  }
  ensureInitialized() {
    if (!this.wasm || !this.isInitialized) {
      throw new Error("SnowFall has not been initialized. Please call init() first.");
    }
    return this.wasm;
  }
  /**
   * Rustから呼び出すためのTypeScript関数を登録します。
   * @param name 登録名 (例: "my_library.my_function")
   * @param func 実行する関数
   */
  registerHostFunction(name, func) {
    this.hostFunctions.set(name, func);
  }
  /**
   * Wasmからの要求に応じてホスト関数を呼び出します。
   * @param request Wasmから渡されるHostRequestオブジェクト
   * @returns 実行結果を含むHostResponseオブジェクト
   */
  async invokeHostFunction(request) {
    const func = this.hostFunctions.get(request.operation);
    if (!func) {
      return {
        status: "ERROR",
        error_info: {
          type: "RuntimeError",
          message: `Host function '${request.operation}' not found.`,
          code: "SF501",
          line: 0,
          // Wasmからの呼び出しのため、行情報は別途渡す必要がある
          column: 0,
          trace: []
        }
      };
    }
    try {
      const nativeArgs = request.args.map((arg) => this.deserializeSnowValue(arg));
      const result = await func(...nativeArgs);
      if (request.requires_return) {
        return {
          status: "OK",
          result: this.serializeTsValue(result)
        };
      } else {
        return { status: "OK" };
      }
    } catch (e) {
      return {
        status: "ERROR",
        error_info: {
          type: "RuntimeError",
          message: e.message || "An unexpected error occurred in host function.",
          code: "SF502",
          line: 0,
          column: 0,
          trace: e.stack ? e.stack.split("\n") : []
        }
      };
    }
  }
  /**
   * Wasmから渡された `SnowObject` (のJSON表現) をTypeScriptのネイティブ型に
   * 再帰的に変換します。`SnowFallHandle`はプロキシオブジェクトに変換されます。
   *
   * @param value 変換対象の `SnowObject`
   * @returns TypeScriptのネイティブ値
   */
  deserializeSnowValue(value) {
    if (value === null || typeof value !== "object") {
      return value;
    }
    if (value.__type === "SnowFallHandle") {
      return this._test_create_proxy_from_handle(value);
    }
    const data = value.data;
    if (data === "Void") {
      return void 0;
    }
    if (typeof data !== "object" || data === null) {
      return data;
    }
    const key = Object.keys(data)[0];
    const val = data[key];
    switch (key) {
      case "Int":
      case "Long":
      case "Float":
      case "Char":
      case "String":
      case "Boolean":
        return val;
      case "Array":
        return val.map((item) => this.deserializeSnowValue(item));
      case "Dictionary": {
        const map = /* @__PURE__ */ new Map();
        for (const [k, v] of Object.entries(val)) {
          map.set(k, this.deserializeSnowValue(v));
        }
        return map;
      }
    }
    return value;
  }
  /**
   * TypeScriptのネイティブ値を、Wasmが期待する `SnowObject` (のJSON表現) に
   * 再帰的にシリアライズします。
   * @param value シリアライズ対象のTypeScript値
   * @returns `SnowObject` のJSON表現
   */
  serializeTsValue(value) {
    let type_id;
    let data;
    if (value === null || value === void 0) {
      type_id = 1;
      data = "Void";
    } else {
      switch (typeof value) {
        case "string":
          type_id = 4;
          data = { String: value };
          break;
        case "number":
          if (Number.isInteger(value)) {
            type_id = 2;
            data = { Int: value };
          } else {
            type_id = 3;
            data = { Float: value };
          }
          break;
        case "boolean":
          type_id = 5;
          data = { Boolean: value };
          break;
        case "object":
          if (Array.isArray(value)) {
            type_id = 6;
            data = { Array: value.map((v) => this.serializeTsValue(v)) };
          } else {
            type_id = 7;
            const dict = {};
            const entries = value instanceof Map ? value.entries() : Object.entries(value);
            for (const [k, v] of entries) {
              dict[String(k)] = this.serializeTsValue(v);
            }
            data = { Dictionary: dict };
          }
          break;
        default:
          type_id = 1;
          data = "Void";
          break;
      }
    }
    return { type_id, data, properties: {}, version: 0 };
  }
  /**
   * テスト用にハンドルからプロキシを作成する。
   */
  _test_create_proxy_from_handle(handle) {
    const wasm2 = this.ensureInitialized();
    let proxy;
    if (handle.dataType === "Array") {
      proxy = new SnowFallArrayProxy(handle, wasm2);
    } else if (handle.dataType === "Dictionary") {
      proxy = new SnowFallDictionaryProxy(handle, wasm2);
    } else {
      throw new Error(`Unsupported handle dataType: ${handle.dataType}`);
    }
    this.handleRegistry.register(proxy, handle.id);
    return proxy;
  }
};
export {
  SnowFall
};
//# sourceMappingURL=snowfall.js.map
