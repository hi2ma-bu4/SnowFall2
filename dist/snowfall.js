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
  initSync: () => initSync
});
var wasm;
function allocate_memory(size) {
  const ret = wasm.allocate_memory(size);
  return ret >>> 0;
}
function free_memory(ptr, size) {
  wasm.free_memory(ptr, size);
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
var SnowFall = class {
  wasm = null;
  isInitialized = false;
  /**
   * Wasmモジュールを非同期で初期化します。
   */
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
  /**
   * 初期化が完了していることを保証するヘルパーメソッド。
   */
  ensureInitialized() {
    if (!this.wasm || !this.isInitialized) {
      throw new Error("SnowFall has not been initialized. Please call init() first.");
    }
    return this.wasm;
  }
  /**
   * Wasmメモリからポインタを介してオブジェクトをデコード(デシリアライズ)します。
   */
  decodeObjectFromWasm(ptr, size) {
    const wasm2 = this.ensureInitialized();
    const wasmMemory = new Uint8Array(wasm2.memory.buffer);
    const jsonDataBytes = wasmMemory.subarray(ptr, ptr + size);
    const jsonString = new TextDecoder().decode(jsonDataBytes);
    wasm2.free_memory(ptr, size);
    return JSON.parse(jsonString);
  }
  /**
   * オブジェクトをエンコード(シリアライズ)してWasmメモリに書き込み、ポインタを返します。
   */
  encodeObjectToWasm(obj) {
    const wasm2 = this.ensureInitialized();
    const jsonString = JSON.stringify(obj);
    const bytes = new TextEncoder().encode(jsonString);
    const ptr = wasm2.allocate_memory(bytes.length);
    if (ptr === 0) {
      throw new Error("Failed to allocate memory in Wasm module.");
    }
    const wasmMemory = new Uint8Array(wasm2.memory.buffer);
    wasmMemory.set(bytes, ptr);
    return { ptr, size: bytes.length };
  }
};
export {
  SnowFall
};
//# sourceMappingURL=snowfall.js.map
