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

	// サブテスト: 暗黙の型変換 (==)
	await t.test("Implicit Type Conversion", () => {
		// String > Float
		assert.strictEqual(wasm._test_implicit_comparison({ String: "1.23" }, { Float: 1.23 }), true);
		// Float > Int
		assert.strictEqual(wasm._test_implicit_comparison({ Float: 123.0 }, { Int: 123 }), true);
		// Int > Char
		assert.strictEqual(wasm._test_implicit_comparison({ Int: 65 }, { Char: 'A' }), true);
		// Char > Boolean
		assert.strictEqual(wasm._test_implicit_comparison({ Char: '\u0001' }, { Boolean: true }), true); // non-zero char is true
		// String > Boolean
		assert.strictEqual(wasm._test_implicit_comparison({ String: "true" }, { Boolean: true }), true);
		// 異なる型だが変換できない -> false
		assert.strictEqual(wasm._test_implicit_comparison({ String: "abc" }, { Int: 123 }), false);
		// 複雑な型同士 -> false
		assert.strictEqual(wasm._test_implicit_comparison({ Array: [] }, { Array: [] }), false);
	});

	// サブテスト: ホスト関数インターフェース (TS/Wasm連携)
	await t.test("Host Function Interface", async () => {
		// テスト用のホスト関数を登録
		snowfall.registerHostFunction("testing.add", (a, b) => {
			if (typeof a !== "number" || typeof b !== "number") {
				throw new Error("Invalid arguments for testing.add");
			}
			return a + b;
		});

		const request = {
			operation: "testing.add",
			args: [
				{ type_id: 2, data: { Int: 10 } },
				{ type_id: 3, data: { Float: 5.5 } },
			],
			requires_return: true,
		};

		const response = await snowfall.invokeHostFunction(request);

		assert.strictEqual(response.status, "OK");
		// 10 + 5.5 = 15.5 (Float)
		assert.deepStrictEqual(response.result, {
			type_id: 3,
			data: { Float: 15.5 },
			properties: {},
			version: 0,
		});
	});
});
