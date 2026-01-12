/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export const main_init: () => void;
export const free_memory_with_len: (a: number, b: number, c: number) => void;
export const free_memory: (a: number, b: number) => void;
export const version: () => [number, number];
export const lexer: (a: number, b: number) => [number, number, number];
export const parser: (a: number, b: number) => [number, number, number];
export const normalize: (a: number, b: number) => [number, number, number];
export const __wbg_wasmcompileresult_free: (a: number, b: number) => void;
export const wasmcompileresult_binary: (a: number) => [number, number];
export const wasmcompileresult_errors: (a: number) => any;
export const compile: (a: number, b: number, c: number) => number;
export const execute: (a: number, b: number) => [number, number, number];
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __wbindgen_exn_store: (a: number) => void;
export const __externref_table_alloc: () => number;
export const __wbindgen_externrefs: WebAssembly.Table;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
export const __externref_table_dealloc: (a: number) => void;
export const __wbindgen_start: () => void;
