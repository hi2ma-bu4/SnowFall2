import assert from "node:assert";
import fs from "node:fs/promises";
import path from "node:path";
import { test } from "node:test";
import { SnowFall } from "../dist/snowfall.js";

// Wasmファイルを直接読み込んでArrayBufferとして初期化
const wasmPath = path.join(process.cwd(), "./pkg/snowfall_core_bg.wasm");
const wasmBuffer = await fs.readFile(wasmPath);

// テストスイート
test("SnowFall Language Core Features", async (t) => {
	const snowfall = new SnowFall();
	await snowfall.init(wasmBuffer);
	const wasm = snowfall.ensureInitialized();

	// サブテスト: 静的検証
	await t.test("Static Validation", () => {
		// Child (ID: 3) は grandparent_prop を持つはず
		const result1 = wasm._test_static_validation(3, "grandparent_prop");
		assert.strictEqual(result1, "Validation successful");

		// 存在しないプロパティはエラーになるはず
		const result2 = wasm._test_static_validation(3, "non_existent_prop");
		assert.strictEqual(result2.type, "CompilationError");
		assert.strictEqual(result2.code, "SF020");
	});

	// サブテスト: エラー伝播
	await t.test("Error Propagation", () => {
		const error = wasm._test_error_propagation();
		assert.strictEqual(error.type, "RuntimeError");
		assert.strictEqual(error.code, "SF999");
		assert.deepStrictEqual(error.trace, ["function_a at line 20", "main at line 5"]);
	});

	// サブテスト: プロトタイプチェーン検索
	await t.test("Prototype Chain Lookup", () => {
		const prop = wasm._test_prototype_lookup("grandparent_prop");
		assert.ok(prop, "Property should not be null");
		assert.deepStrictEqual(prop.data, { String: "from_grandparent" });
	});

	// サブテスト: ハンドル/プロキシシステム (Array)
	await t.test("Handle/Proxy System - Array Integration", async () => {
		const handle = wasm._test_create_array_handle();
		const proxy = snowfall._test_create_proxy_from_handle(handle);

		assert.strictEqual(proxy.length, 2);

		const val1 = await proxy[0];
		assert.deepStrictEqual(val1.data, { Int: 100 });

		// Wasm内の値を更新
		const newObj = { type_id: 25, data: { String: "updated" } };
		proxy[1] = newObj;

		// 更新された値を取得して確認
		const val2 = await proxy[1];
		assert.deepStrictEqual(val2.data, { String: "updated" });
	});

	// サブテスト: ハンドル/プロキシシステム (Dictionary)
	await t.test("Handle/Proxy System - Dictionary Integration", async () => {
		const handle = wasm._test_create_dictionary_handle();
		const proxy = snowfall._test_create_proxy_from_handle(handle);

		const valA = await proxy.a;
		assert.deepStrictEqual(valA.data, { Int: 100 });

		const newObj = { type_id: 50, data: { Boolean: true } };
		proxy.b = newObj;

		const valB = await proxy.b;
		assert.deepStrictEqual(valB.data, { Boolean: true });
	});
});
