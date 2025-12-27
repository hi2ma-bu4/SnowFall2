import assert from "node:assert";
import fs from "node:fs/promises";
import path from "node:path";
import { test } from "node:test";
import { SnowFall } from "../dist/snowfall.js";

const wasmPath = path.join(process.cwd(), "./pkg/snowfall_core_bg.wasm");
const wasmBuffer = await fs.readFile(wasmPath);

test("Parser Functionality", async (t) => {
    const snowfall = new SnowFall();
    await snowfall.init(wasmBuffer);
    const wasm = snowfall.ensureInitialized();

    await t.test("should parse a simple function declaration", () => {
        const input = "function Int main() { return 0; }";
        const result = wasm._test_parser(input);
        assert.strictEqual(result.errors.length, 0, `Parser reported errors: ${result.errors.join(", ")}`);

        const program = result.ast.Program;
        assert.strictEqual(program.length, 1, "Program should have one statement.");

        const func = program[0].Function;
        assert.strictEqual(func.name, "main");
        assert.strictEqual(func.return_type, "Int");
    });

    await t.test("should parse a sub declaration", () => {
        const input = "sub mySub() {}";
        const result = wasm._test_parser(input);
        assert.strictEqual(result.errors.length, 0, `Parser reported errors: ${result.errors.join(", ")}`);
        const sub = result.ast.Program[0].Sub;
        assert.strictEqual(sub.name, "mySub");
    });

    await t.test("should parse infix expressions with correct precedence", () => {
        const input = "1 + 2 * 3;";
        const result = wasm._test_parser(input);
        assert.strictEqual(result.errors.length, 0, `Parser reported errors: ${result.errors.join(", ")}`);
        const expr = result.ast.Program[0].Expression.Infix;

        assert.strictEqual(expr.operator.type, "Plus");
        assert.strictEqual(expr.left.IntLiteral, 1);

        const right = expr.right.Infix;
        assert.strictEqual(right.operator.type, "Asterisk");
        assert.strictEqual(right.left.IntLiteral, 2);
        assert.strictEqual(right.right.IntLiteral, 3);
    });

    await t.test("should parse C-style cast expressions in a declaration", () => {
        const input = `(Int) my_int = (Int) my_float;`;
        const result = wasm._test_parser(input);
        assert.strictEqual(result.errors.length, 0, `Parser reported errors: ${result.errors.join(", ")}`);

        const stmt = result.ast.Program[0].Let;
        assert.strictEqual(stmt.name, "my_int");
        assert.strictEqual(stmt.type_name, "Int");

        const expr = stmt.value.Cast;
        assert.strictEqual(expr.target_type, "Int");
        assert.strictEqual(expr.expression.Identifier, "my_float");
    });

    await t.test("should report syntax errors", () => {
        const input = "function Int main( { return 0; }"; // Missing parenthesis
        const result = wasm._test_parser(input);
        assert(result.errors.length > 0, "Parser should report errors for invalid syntax.");
        assert(result.errors[0].includes("expected next token to be RParen"), "Error message should be about missing parenthesis.");
    });
});
