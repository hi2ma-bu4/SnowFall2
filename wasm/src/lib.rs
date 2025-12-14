// wasm/src/lib.rs

// このライブラリのルートモジュールです。
// Wasmに公開する関数などを定義します。
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::mem;

// 共通モジュールを宣言
pub mod common;

/// Wasmモジュールのメモリを確保し、そのポインタを返す。
/// TS側でデータをWasmに渡す際に使用される。
#[wasm_bindgen]
pub fn allocate_memory(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let ptr = buffer.as_mut_ptr();
    // bufferがスコープを抜けてもメモリが解放されないようにする
    mem::forget(buffer);
    ptr
}

/// Wasmモジュール内の確保されたメモリを解放する。
/// TS側でWasmから受け取ったデータの処理が終わった後に呼び出す。
#[wasm_bindgen]
pub fn free_memory(ptr: *mut u8, size: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, size);
    }
}
