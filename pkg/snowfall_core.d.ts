/* tslint:disable */
/* eslint-disable */

/**
 * テスト用の配列ハンドルを作成して返す。
 */
export function _test_create_array_handle(): any;

/**
 * テスト用のディクショナリハンドルを作成して返す。
 */
export function _test_create_dictionary_handle(): any;

/**
 * エラー伝播をテストするための一時的な関数。
 */
export function _test_error_propagation(): any;

/**
 * 暗黙の型変換を伴う比較ロジックをテストするための関数。
 */
export function _test_implicit_comparison(left: any, right: any): boolean;

/**
 * プロトタイプチェーン検索をテストするための一時的な関数。
 */
export function _test_prototype_lookup(key: string): any;

/**
 * 静的検証ロジックをテストするための関数。
 */
export function _test_static_validation(type_id: number, property_name: string): any;

/**
 * Wasmモジュールのメモリを確保し、そのポインタを返す。
 */
export function allocate_memory(size: number): number;

/**
 * `find_property_recursive` をWasmに公開するためのラッパー関数。
 */
export function find_property_on_prototype(obj: any, key: string): any;

/**
 * Wasmモジュール内の確保されたメモリを解放する。
 */
export function free_memory(ptr: number, size: number): void;

/**
 * TypeScript側から参照されるオブジェクトの要素を取得します。
 */
export function get_element_by_handle(handle_id: number, key: any): any;

/**
 * ライブラリの初期化時に一度だけ呼び出されるべき関数。
 */
export function main_init(): void;

/**
 * TypeScript側がオブジェクトの参照を終えたことをWasmに通知します。
 */
export function release_handle(handle_id: number): void;

/**
 * TypeScript側から参照されるオブジェクトの要素を設定します。
 */
export function set_element_by_handle(handle_id: number, key: any, value: any): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly _test_create_array_handle: () => any;
  readonly _test_create_dictionary_handle: () => any;
  readonly _test_error_propagation: () => any;
  readonly _test_implicit_comparison: (a: any, b: any) => [number, number, number];
  readonly _test_prototype_lookup: (a: number, b: number) => any;
  readonly _test_static_validation: (a: number, b: number, c: number) => any;
  readonly allocate_memory: (a: number) => number;
  readonly find_property_on_prototype: (a: any, b: number, c: number) => any;
  readonly free_memory: (a: number, b: number) => void;
  readonly get_element_by_handle: (a: number, b: any) => any;
  readonly main_init: () => void;
  readonly release_handle: (a: number) => void;
  readonly set_element_by_handle: (a: number, b: any, c: any) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
