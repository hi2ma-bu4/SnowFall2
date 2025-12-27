import assert from "node:assert";
import fs from "node:fs/promises";
import path from "node:path";
import { test } from "node:test";
import { SnowFall } from "../../dist/snowfall.js";

const wasmPath = path.join(process.cwd(), "./pkg/snowfall_core_bg.wasm");
const wasmBuffer = await fs.readFile(wasmPath);

test("Lexer Functionality", async (t) => {
	const snowfall = new SnowFall();
	await snowfall.init(wasmBuffer);
	const wasm = snowfall.ensureInitialized();

	await t.test("should tokenize operators and delimiters", () => {
		const input = "=+(){},;";
		const expected_tokens = [{ type: "Assign" }, { type: "Plus" }, { type: "LParen" }, { type: "RParen" }, { type: "LBrace" }, { type: "RBrace" }, { type: "Comma" }, { type: "Semicolon" }, { type: "Eof" }];
		const tokens = wasm._test_lexer(input);
		assert.deepStrictEqual(
			tokens.map((t) => ({ type: t.type })),
			expected_tokens,
			"Failed to tokenize basic operators"
		);
	});

	await t.test("should tokenize a simple function declaration", () => {
		const input = `
            function Int add(Int a, Int b) {
                return a + b;
            }
        `;
		const expected_tokens = ["Function", "Ident", "Ident", "LParen", "Ident", "Ident", "Comma", "Ident", "Ident", "RParen", "LBrace", "Return", "Ident", "Plus", "Ident", "Semicolon", "RBrace", "Eof"];
		const tokens = wasm._test_lexer(input);
		assert.deepStrictEqual(
			tokens.map((t) => t.type),
			expected_tokens,
			"Failed to tokenize function declaration"
		);
	});

	await t.test("should correctly tokenize numeric literals including underscored", () => {
		const input = "10_000 123.456 0xff 0b101 0xf.f";
		const tokens = wasm._test_lexer(input);

		assert.deepStrictEqual(tokens[0], { type: "Int", value: 10000 });
		assert.deepStrictEqual(tokens[1], { type: "Float", value: 123.456 });
		assert.deepStrictEqual(tokens[2], { type: "Int", value: 255 });
		assert.deepStrictEqual(tokens[3], { type: "Int", value: 5 });
		assert.deepStrictEqual(tokens[4], { type: "Float", value: 15.9375 });
	});

	await t.test("should tokenize logical and comparison operators", () => {
		const input = "a == b === c && d || e and f or g";
		const expected_tokens = ["Ident", "Eq", "Ident", "StrictEq", "Ident", "LogicalAnd", "Ident", "LogicalOr", "Ident", "And", "Ident", "Or", "Ident", "Eof"];
		const tokens = wasm._test_lexer(input);
		assert.deepStrictEqual(
			tokens.map((t) => t.type),
			expected_tokens,
			"Failed to tokenize logical and comparison ops"
		);
	});

	await t.test("should tokenize string literals", () => {
		const input = `"hello world"`;
		const tokens = wasm._test_lexer(input);
		assert.deepStrictEqual(tokens[0], { type: "String", value: "hello world" });
	});
});
