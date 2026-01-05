/* tslint:disable */
/* eslint-disable */

/**
 * Wasmモジュールのメモリを確保し、そのポインタを返す
 */
export function allocate_memory(size: number): number;

/**
 * Wasmモジュール内の確保されたメモリを解放する
 */
export function free_memory(ptr: number, size: number): void;

/**
 * ソースコードを受け取り、トークンのリストを返す
 * @deprecated 本番環境での使用は非推奨
 */export function lexer(source: string): any;

/**
 * ライブラリの初期化時に一度だけ呼び出されるべき関数
 */
export function main_init(): void;

/**
 * ソースコードを受け取り、解析したASTを返す
 * @deprecated 本番環境での使用は非推奨
 */export function parser(source: string): any;

export function version(): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main_init: () => void;
  readonly allocate_memory: (a: number) => number;
  readonly free_memory: (a: number, b: number) => void;
  readonly version: () => [number, number];
  readonly lexer: (a: number, b: number) => [number, number, number];
  readonly parser: (a: number, b: number) => [number, number, number];
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
