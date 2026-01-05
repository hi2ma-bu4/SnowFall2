import * as assert from "node:assert";
import { test } from "node:test";
import { SnowFall } from "../../dist/snowfall";
import { wasmBuffer } from "./lib/getWasm";

test("Error Test", async (t) => {
	const sf = new SnowFall();
	await sf.init(wasmBuffer);

	await t.test("should handle unexpected characters", () => {
		const code = `#`;
		const { errors } = sf.dev_parser(code);
		assert.ok(errors, "Expected a compiler error");
		assert.strictEqual(errors[0].code, "SF0001");
		assert.strictEqual(errors[0].message, "Unexpected character: #");
	});

	await t.test("should handle unterminated strings", () => {
		const code = `(String) a = "hello`;
		const { errors } = sf.dev_parser(code);
		assert.ok(errors, "Expected a compiler error");
		assert.strictEqual(errors[0].code, "SF0003");
		assert.strictEqual(errors[0].message, "Unterminated string");
	});

	await t.test("should handle unexpected tokens", () => {
		const code = `(Int) a = 1 +;`;
		const { errors } = sf.dev_parser(code);
		assert.ok(errors, "Expected a compiler error");
		assert.strictEqual(errors[0].code, "SF0015");
		assert.strictEqual(errors[0].message, "Unexpected token for expression: Token { kind: Delimiter(Semicolon), span: Span { start: 13, end: 14 } }");
	});
});
