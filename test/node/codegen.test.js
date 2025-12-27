import assert from "node:assert";
import fs from "node:fs/promises";
import path from "node:path";
import { test } from "node:test";
import { SnowFall } from "../../dist/snowfall.js";

const wasmPath = path.join(process.cwd(), "./pkg/snowfall_core_bg.wasm");
const wasmBuffer = await fs.readFile(wasmPath);

test("Code Generator Functionality", async (t) => {
	const snowfall = new SnowFall();
	await snowfall.init(wasmBuffer);
	const wasm = snowfall.ensureInitialized();

	await t.test("should generate correct SIR for a simple integer expression", () => {
		const input = "1 + 2;";
		const sir = wasm._test_codegen(input);

		// Expected constants: 1, 2
		assert.strictEqual(sir.constants.length, 2);
		assert.deepStrictEqual(sir.constants[0].value, { Int: 1 });
		assert.deepStrictEqual(sir.constants[1].value, { Int: 2 });

		// Expected instructions: PUSH_CONST 0, PUSH_CONST 1, ADD, POP
		const expected_opcodes = ["PUSH_CONST", "PUSH_CONST", "ADD", "POP"];
		assert.deepStrictEqual(
			sir.code.map((c) => c.opcode),
			expected_opcodes
		);
	});

	// Placeholder for a more complex test, e.g., an if statement
	await t.skip("should generate correct SIR for an if statement", () => {
		const input = "if (true) { 1; }";
		const sir = wasm._test_codegen(input);

		const expected_opcodes = ["PUSH_CONST", "JUMP_IF_FALSE", "PUSH_CONST", "POP", "POP"];
		assert.deepStrictEqual(
			sir.code.map((c) => c.opcode),
			expected_opcodes
		);
	});
});
