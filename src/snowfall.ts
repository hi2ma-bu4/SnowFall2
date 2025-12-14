// src/snowfall.ts

import { createError, SnowFallError, SnowFallErrorData } from './errors';

// Rust(Wasm)側からエクスポートされる関数の型定義
// TODO: wasm-bindgenの型定義ファイルが生成されたら、そちらを参照するように変更する
interface SnowFallWasmExports {
  memory: WebAssembly.Memory;
  // メモリ確保用関数
  allocate_memory(size: number): number;
  // メモリ解放用関数
  free_memory(ptr: number, size: number): void;
  // コンパイル関数 (仮)
  compile(source: string): { ptr: number, size: number };
  // 実行関数 (仮)
  execute(intermediate_code_ptr: number, intermediate_code_size: number): { ptr: number, size: number };
}

/**
 * Host/TS側からWasmへ渡される要求データ。
 */
export interface HostRequest {
  operation: string;
  args: any[]; // SnowValueに相当するデータをTS側で表現したもの
  requires_return: boolean;
}

/**
 * Wasm側からHost/TSへ返される応答データ。
 */
export interface HostResponse {
  status: 'OK' | 'ERROR';
  result?: any; // 成功した場合の戻り値
  error_info?: SnowFallErrorData; // エラーが発生した場合の情報
}

/**
 * SnowFall言語の検証、コンパイル、実行を管理するメインクラス。
 */
export class SnowFall {
  private wasm: SnowFallWasmExports | null = null;
  private wasmModule: WebAssembly.Module | null = null;

  /**
   * Wasmモジュールを非同期で初期化します。
   * @param wasmUrl WasmファイルのURL
   */
  public async init(wasmUrl: string): Promise<void> {
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
  private async instantiate(): Promise<void> {
    if (!this.wasmModule) {
      throw new Error("Wasm module is not compiled.");
    }
    const importObject = {
      env: {
        // Rust側から `invokeHostFunction` を呼び出すための実装
        invokeHostFunction: (ptr: number, size: number): { ptr: number, size: number } => {
          const request = this.decodeObjectFromWasm<HostRequest>(ptr, size);
          const response = this.handleHostRequest(request);
          return this.encodeObjectToWasm(response);
        },
      },
    };
    this.wasm = (await WebAssembly.instantiate(this.wasmModule, importObject)).exports as unknown as SnowFallWasmExports;
  }

  /**
   * ホスト(TS)側の関数を呼び出すためのハンドラ。
   * Rustからの `HostRequest` に応じて、適切なTS関数を実行します。
   * @param request HostRequestオブジェクト
   * @returns HostResponseオブジェクト
   */
  private handleHostRequest(request: HostRequest): HostResponse {
    try {
      // TODO: operationに応じて、登録されたTS関数を呼び出す
      switch (request.operation) {
        case 'console.log':
          console.log(...request.args);
          return { status: 'OK', result: null };
        // ... 他のホスト関数の実装 ...
        default:
          throw new Error(`Unknown host operation: ${request.operation}`);
      }
    } catch (error) {
      // TS側で発生したエラーをSnowFallErrorの形式に変換して返す
      const errData: SnowFallErrorData = {
        type: 'RuntimeError',
        message: error instanceof Error ? error.message : String(error),
        code: 'SFHOST001',
        line: 0, // ホスト関数呼び出しに行番号の概念はない
        column: 0,
        trace: (error instanceof Error ? error.stack?.split('\n') : []) || [],
      };
      return { status: 'ERROR', error_info: errData };
    }
  }

  /**
   * Wasmメモリからポインタを介してオブジェクトをデコード(デシリアライズ)します。
   */
  private decodeObjectFromWasm<T>(ptr: number, size: number): T {
    if (!this.wasm) throw new Error("Wasm not initialized.");
    const wasmMemory = new Uint8Array(this.wasm.memory.buffer);
    const jsonDataBytes = wasmMemory.subarray(ptr, ptr + size);
    const jsonString = new TextDecoder().decode(jsonDataBytes);
    // Wasm側で確保したメモリは、デコード後に解放する必要がある
    this.wasm.free_memory(ptr, size);
    return JSON.parse(jsonString);
  }

  /**
   * オブジェクトをエンコード(シリアライズ)してWasmメモリに書き込み、ポインタを返します。
   */
  private encodeObjectToWasm(obj: any): { ptr: number, size: number } {
    if (!this.wasm) throw new Error("Wasm not initialized.");

    const jsonString = JSON.stringify(obj);
    const bytes = new TextEncoder().encode(jsonString);

    // Rust側にメモリ確保を依頼し、ポインタを取得
    const ptr = this.wasm.allocate_memory(bytes.length);
    if (ptr === 0) {
      throw new Error("Failed to allocate memory in Wasm module.");
    }

    // 確保したメモリ領域にデータを書き込む
    const wasmMemory = new Uint8Array(this.wasm.memory.buffer);
    wasmMemory.set(bytes, ptr);

    return { ptr, size: bytes.length };
  }

  // ... compile, executeなどのメソッドを今後実装 ...
}
