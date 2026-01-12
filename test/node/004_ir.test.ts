import assert from "node:assert";
import { test } from "node:test";
import { SnowFall } from "../../dist/snowfall";
import { wasmBuffer } from "./lib/getWasm";

// Helper to check magic header
const checkMagic = (binary: Uint8Array) => {
	const magic = new TextDecoder().decode(binary.slice(0, 4));
	assert.strictEqual(magic, "SNFL", "Binary should start with magic header SNFL");
};

test("IR Compilation Test", async (t) => {
	const sf = new SnowFall();
	await sf.init(wasmBuffer);

	await t.test("should compile basic variable declaration", () => {
		const input = `Int a = 10;`;
		const result = sf.compile_bin(input, false);

		assert.ok(result.binary, "Should return binary data");
		assert.strictEqual(result.errors, undefined, "Should not have errors");
		checkMagic(result.binary!);
		assert.ok(result.binary!.length > 8, "Binary should be larger than header");
	});

	await t.test("should compile string literals", () => {
		const input = `String s = "Hello World";`;
		const result = sf.compile_bin(input, false);

		assert.ok(result.binary, "Should return binary data");
		assert.strictEqual(result.errors, undefined);
		checkMagic(result.binary!);

		// バイナリ内に文字列が含まれているか簡易チェック
		const text = new TextDecoder().decode(result.binary!);
		assert.ok(text.includes("Hello World"), "Binary should contain the string literal");
	});

	await t.test("should compile functions", () => {
		const input = `
            function Int add(Int a, Int b) {
                return a + b;
            }
        `;
		const result = sf.compile_bin(input, false);
		assert.ok(result.binary);
		assert.strictEqual(result.errors, undefined);
		checkMagic(result.binary!);
	});

	await t.test("should compile classes", () => {
		const input = `
            class MyClass {
                function Int getValue() { return 1; }
            }
        `;
		const result = sf.compile_bin(input, false);
		assert.ok(result.binary);
		assert.strictEqual(result.errors, undefined);
		checkMagic(result.binary!);

		const text = new TextDecoder().decode(result.binary!);
		assert.ok(text.includes("MyClass"), "Binary should contain class name");
		assert.ok(text.includes("getValue"), "Binary should contain method name");
	});

	await t.test("should compile control flow", () => {
		const input = `
            if (true) {
                Int a = 1;
            } else {
                Int a = 2;
            }
            while (false) {
                Int b = 3;
            }
        `;
		const result = sf.compile_bin(input, false);
		assert.ok(result.binary);
		assert.strictEqual(result.errors, undefined);
		checkMagic(result.binary!);
	});

	await t.test("should return errors for invalid code", () => {
		const input = `Int a = ;`; // Syntax error
		const result = sf.compile_bin(input, false);

		assert.strictEqual(result.binary, undefined, "Should not return binary on error");
		assert.ok(result.errors, "Should return errors");
		assert.ok(result.errors!.length > 0, "Should have at least one error");
		assert.strictEqual(result.errors![0].type, "CompilationError");
	});

	await t.test("should include debug info when debug=true", () => {
		const input = `
            Int a = 1;
            Int b = 2;
            Int c = a + b;
        `;
		const resNoDebug = sf.compile_bin(input, false);
		const resDebug = sf.compile_bin(input, true);

		assert.ok(resNoDebug.binary);
		assert.ok(resDebug.binary);

		// デバッグ情報が含まれる分、サイズが大きくなるはず
		assert.ok(resDebug.binary!.length > resNoDebug.binary!.length, "Debug binary should be larger than release binary");
	});
});
