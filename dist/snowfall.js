// src/snowfall.ts
var SnowFall = class {
  wasm = null;
  wasmModule = null;
  /**
   * Wasmモジュールを非同期で初期化します。
   * @param wasmUrl WasmファイルのURL
   */
  async init(wasmUrl) {
    try {
      const response = await fetch(wasmUrl);
      const buffer = await response.arrayBuffer();
      this.wasmModule = await WebAssembly.compile(buffer);
      await this.instantiate();
      console.log("SnowFall Wasm module initialized successfully.");
    } catch (error) {
      console.error("Failed to initialize SnowFall Wasm module:", error);
      throw error;
    }
  }
  /**
   * Wasmモジュールをインスタンス化します。
   * TS側から呼び出される関数などをインポートオブジェクトとして渡します。
   */
  async instantiate() {
    if (!this.wasmModule) {
      throw new Error("Wasm module is not compiled.");
    }
    const importObject = {
      env: {
        // Rust側から `invokeHostFunction` を呼び出すための実装
        invokeHostFunction: (ptr, size) => {
          const request = this.decodeObjectFromWasm(ptr, size);
          const response = this.handleHostRequest(request);
          return this.encodeObjectToWasm(response);
        }
      }
    };
    this.wasm = (await WebAssembly.instantiate(this.wasmModule, importObject)).exports;
  }
  /**
   * ホスト(TS)側の関数を呼び出すためのハンドラ。
   * Rustからの `HostRequest` に応じて、適切なTS関数を実行します。
   * @param request HostRequestオブジェクト
   * @returns HostResponseオブジェクト
   */
  handleHostRequest(request) {
    try {
      switch (request.operation) {
        case "console.log":
          console.log(...request.args);
          return { status: "OK", result: null };
        // ... 他のホスト関数の実装 ...
        default:
          throw new Error(`Unknown host operation: ${request.operation}`);
      }
    } catch (error) {
      const errData = {
        type: "RuntimeError",
        message: error instanceof Error ? error.message : String(error),
        code: "SFHOST001",
        line: 0,
        // ホスト関数呼び出しに行番号の概念はない
        column: 0,
        trace: (error instanceof Error ? error.stack?.split("\n") : []) || []
      };
      return { status: "ERROR", error_info: errData };
    }
  }
  /**
   * Wasmメモリからポインタを介してオブジェクトをデコード(デシリアライズ)します。
   */
  decodeObjectFromWasm(ptr, size) {
    if (!this.wasm) throw new Error("Wasm not initialized.");
    const wasmMemory = new Uint8Array(this.wasm.memory.buffer);
    const jsonDataBytes = wasmMemory.subarray(ptr, ptr + size);
    const jsonString = new TextDecoder().decode(jsonDataBytes);
    this.wasm.free_memory(ptr, size);
    return JSON.parse(jsonString);
  }
  /**
   * オブジェクトをエンコード(シリアライズ)してWasmメモリに書き込み、ポインタを返します。
   */
  encodeObjectToWasm(obj) {
    if (!this.wasm) throw new Error("Wasm not initialized.");
    const jsonString = JSON.stringify(obj);
    const bytes = new TextEncoder().encode(jsonString);
    const ptr = this.wasm.allocate_memory(bytes.length);
    if (ptr === 0) {
      throw new Error("Failed to allocate memory in Wasm module.");
    }
    const wasmMemory = new Uint8Array(this.wasm.memory.buffer);
    wasmMemory.set(bytes, ptr);
    return { ptr, size: bytes.length };
  }
  // ... compile, executeなどのメソッドを今後実装 ...
};
export {
  SnowFall
};
//# sourceMappingURL=snowfall.js.map
