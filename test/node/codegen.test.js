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
	const compiler = snowfall.getCompiler();

	await t.test("should generate correct SIR for a simple integer expression", () => {
		const input = "1 + 2;";
		const bytecode = compiler.compile(input);
		const lines = bytecode.trim().split("\n");

		assert(lines[0].startsWith(".SNWF"), "Header section is missing");
		assert.strictEqual(lines[1], ".CONST", "Constants section is missing");
		assert.strictEqual(lines[2], "0:I:1", "Constant 0 (1) is incorrect");
		assert.strictEqual(lines[3], "1:I:2", "Constant 1 (2) is incorrect");
		assert.strictEqual(lines[4], ".CODE", "Code section is missing");

		const codeLines = lines.slice(5);
		const expected_opcodes = ["PUSH_CONST 0", "PUSH_CONST 1", "ADD", "POP"];
		assert.deepStrictEqual(codeLines, expected_opcodes, "Bytecode instructions do not match");
	});

	// Placeholder for a more complex test, e.g., an if statement
	await t.skip("should generate correct SIR for an if statement", () => {
		const input = "if (true) { 1; }";
		const bytecode = compiler.compile(input);
		// Add assertions for the if statement bytecode
	});
});
