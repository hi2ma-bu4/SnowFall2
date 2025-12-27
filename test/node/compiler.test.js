import assert from "node:assert";
import fs from "node:fs/promises";
import path from "node:path";
import { test } from "node:test";
import { SnowFall, SnowFallCompilerError } from "../../dist/snowfall.js";

const wasmPath = path.join(process.cwd(), "./pkg/snowfall_core_bg.wasm");
const wasmBuffer = await fs.readFile(wasmPath);

async function createCompiler() {
	const snowfall = new SnowFall();
	await snowfall.init(wasmBuffer);
	return snowfall.getCompiler();
}

test("Compiler Integration Tests", async (t) => {
	await t.skip("TDD-1: Size Comparison - Bytecode should be smaller than source", async () => {
		const compiler = await createCompiler();
		const source = `
// This is a simple program with comments and whitespace.
(Int) a = 10;
(Int) b = 20;
(Int) c = a + b;
`;
		const bytecode = compiler.compile(source);
		assert(
			bytecode.length < source.length,
			`Bytecode size (${bytecode.length}) should be less than source size (${source.length})`
		);
	});

	await t.test("TDD-2: Invariance - Same source produces same bytecode", async () => {
		const compiler = await createCompiler();
		const source = `(Int) a = 10;`;
		const bytecode1 = compiler.compile(source);
		const bytecode2 = compiler.compile(source);
		assert.strictEqual(bytecode1, bytecode2, "Bytecode should be identical for the same source");
	});

	await t.test("TDD-3: Error Propagation - Invalid syntax should report correct line/column", async () => {
		const compiler = await createCompiler();
		const source = `
(Int) a = 10;
(Int) b = ; // Syntax error here
`;
		try {
			compiler.compile(source);
			assert.fail("Expected a SnowFallCompilerError but none was thrown.");
		} catch (e) {
			assert(e instanceof SnowFallCompilerError, "Error should be an instance of SnowFallCompilerError");
			assert(e.errors.length > 0, "Should have at least one error");
			const error = e.errors[0]; // Check the first error for specifics
			assert.strictEqual(error.line, 4, "Error should be on line 4");
			assert(error.column > 1, `Error column (${error.column}) should be greater than 1`);
			assert(error.message.includes("No prefix parse function for Semicolon"));
		}
	});

	await t.test("Should produce correct SIR for a simple integer expression", async () => {
		const compiler = await createCompiler();
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
});

test("Comprehensive Compiler Tests", async (t) => {
	const compiler = await createCompiler();

	const testCases = [
		{
			name: "Integer Declaration",
			source: "(Int) a = 42;",
			expectedConstants: ["0:I:42"],
			expectedCode: ["PUSH_CONST 0", "SET_VAR 0"],
		},
		{
			name: "Float Declaration",
			source: "(Float) b = 3.14;",
			expectedConstants: ["0:F:3.14"],
			expectedCode: ["PUSH_CONST 0", "SET_VAR 0"],
		},
		{
			name: "String Declaration",
			source: `(String) c = "hello";`,
			expectedConstants: [`0:S:aGVsbG8=`],
			expectedCode: ["PUSH_CONST 0", "SET_VAR 0"],
		},
		{
			name: "Boolean Declaration",
			source: "(Boolean) d = true;",
			expectedConstants: ["0:B:1"],
			expectedCode: ["PUSH_CONST 0", "SET_VAR 0"],
		},
		{
			name: "If Statement",
			source: "if (true) { 1; }",
			expectedConstants: ["0:B:1", "1:I:1"],
			expectedCode: ["PUSH_CONST 0", "JUMP_IF_FALSE 4", "PUSH_CONST 1", "POP"],
		},
		{
			name: "Function Declaration",
			source: "function Int myFunc() { return 1; }",
			expectedConstants: ["0:I:1"],
			expectedCode: ["PUSH_CONST 0", "RETURN"],
		},
		{
			name: "Subroutine Declaration",
			source: "sub mySub() {}",
			expectedConstants: [],
			expectedCode: ["PUSH_CONST 4294967295", "RETURN"],
		},
		{
			name: "String with Special Characters",
			source: `(String) s = "こんにちは世界\nHello:World";`,
			// "こんにちは世界\nHello:World" -> base64
			expectedConstants: ["0:S:44GT44KT44Gr44Gh44Gv5LiW55WMCkhlbGxvOldvcmxk"],
			expectedCode: ["PUSH_CONST 0", "SET_VAR 0"],
		},
	];

	for (const tc of testCases) {
		await t.test(`should correctly compile: ${tc.name}`, () => {
			assert.doesNotThrow(() => {
				const bytecode = compiler.compile(tc.source);
				const lines = bytecode.trim().split("\n");

				const constIndex = lines.indexOf(".CONST");
				const codeIndex = lines.indexOf(".CODE");

				if (tc.expectedConstants.length > 0) {
					assert.notStrictEqual(constIndex, -1, ".CONST section missing");
					const constLines = lines.slice(constIndex + 1, codeIndex);
					assert.deepStrictEqual(constLines, tc.expectedConstants, "Constants do not match");
				} else {
					assert.strictEqual(constIndex, -1, ".CONST section should be absent");
				}

				assert.notStrictEqual(codeIndex, -1, ".CODE section missing");
				const codeLines = lines.slice(codeIndex + 1);
				assert.deepStrictEqual(codeLines, tc.expectedCode, "Code instructions do not match");
			}, `Compilation failed for: ${tc.name}`);
		});
	}
});
