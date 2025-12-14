// src/snowfall.ts

import { createError, SnowFallError, SnowFallErrorData } from './errors';

// wasm-packが生成したJSと型定義をインポートします。
// `init`はデフォルトエクスポート、その他は名前空間`wasm`にインポートされます。
// @ts-ignore - a build step will create this file
import init, * as wasm from '../pkg/snowfall_core.js';

// `wasm`名前空間に`memory`が存在することをTypeScriptに伝えるための型拡張
type WasmModule = typeof wasm & { memory: WebAssembly.Memory };

/**
 * Host/TS側からWasmへ渡される要求データ。
 */
export interface HostRequest {
  operation: string;
  args: any[];
  requires_return: boolean;
}

/**
 * Wasm側からHost/TSへ返される応答データ。
 */
export interface HostResponse {
  status: 'OK' | 'ERROR';
  result?: any;
  error_info?: SnowFallErrorData;
}

/**
 * SnowFall言語の検証、コンパイル、実行を管理するメインクラス。
 */
export class SnowFall {
  private wasm: WasmModule | null = null;
  private isInitialized = false;

  /**
   * Wasmモジュールを非同期で初期化します。
   */
  public async init(wasmUrl: string): Promise<void> {
    if (this.isInitialized) {
      return;
    }
    try {
      // wasm-packのinit関数を実行します。
      // これにより、`wasm`名前空間内の関数が利用可能になります。
      await init(wasmUrl);
      this.wasm = wasm as WasmModule;
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
  private ensureInitialized(): WasmModule {
    if (!this.wasm || !this.isInitialized) {
      throw new Error("SnowFall has not been initialized. Please call init() first.");
    }
    return this.wasm;
  }

  /**
   * Wasmメモリからポインタを介してオブジェクトをデコード(デシリアライズ)します。
   */
  private decodeObjectFromWasm<T>(ptr: number, size: number): T {
    const wasm = this.ensureInitialized();
    const wasmMemory = new Uint8Array(wasm.memory.buffer);
    const jsonDataBytes = wasmMemory.subarray(ptr, ptr + size);
    const jsonString = new TextDecoder().decode(jsonDataBytes);
    wasm.free_memory(ptr, size);
    return JSON.parse(jsonString);
  }

  /**
   * オブジェクトをエンコード(シリアライズ)してWasmメモリに書き込み、ポインタを返します。
   */
  private encodeObjectToWasm(obj: any): { ptr: number, size: number } {
    const wasm = this.ensureInitialized();

    const jsonString = JSON.stringify(obj);
    const bytes = new TextEncoder().encode(jsonString);

    const ptr = wasm.allocate_memory(bytes.length);
    if (ptr === 0) {
      throw new Error("Failed to allocate memory in Wasm module.");
    }

    const wasmMemory = new Uint8Array(wasm.memory.buffer);
    wasmMemory.set(bytes, ptr);

    return { ptr, size: bytes.length };
  }
}
