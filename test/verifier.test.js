import assert from "node:assert";
import fs from "node:fs/promises";
import path from "node:path";
import { test } from "node:test";
import { SnowFall } from "../dist/snowfall.js";

const wasmPath = path.join(process.cwd(), "./pkg/snowfall_core_bg.wasm");
const wasmBuffer = await fs.readFile(wasmPath);

test("Static Type Verifier Functionality", async (t) => {
    const snowfall = new SnowFall();
    await snowfall.init(wasmBuffer);
    const wasm = snowfall.ensureInitialized();

    await t.test("should validate string concatenation rule", () => {
        // "hello" + 1 should be valid and result in a String
        const input = `"hello" + 1;`;
        const result = wasm._test_verifier(input);
        assert.strictEqual(result.errors.length, 0, "Should be a valid operation.");
    });

    await t.test("should detect type mismatch for non-string + operations", () => {
        const input = `1 + true;`; // Int + Boolean
        const result = wasm._test_verifier(input);
        assert(result.errors.length > 0, "Should detect a type mismatch.");
        assert(result.errors[0].code === "SF021", "Should be a type mismatch error.");
    });

    await t.test("should detect calls to non-existent methods", () => {
        const input = `
            (String) myString = "hello";
            myString.nonExistentMethod();
        `;
        const result = wasm._test_verifier(input);
        assert(result.errors.length > 0, "Should detect a non-existent method call.");
        assert(result.errors[0].code === "SF020", "Should be a method not found error.");
    });

    // This test requires a more advanced verifier that understands class inheritance.
    // It's a placeholder for the future.
    await t.skip("@override decorator should validate method signatures", () => {
        const input = `
            class Parent {
                function void myMethod(Int a) {}
            }
            class Child extends Parent {
                @override
                function void myMethod(Float a) {} // Mismatched signature
            }
        `;
        const result = wasm._test_verifier(input);
        assert(result.errors.length > 0, "Should detect an override signature mismatch.");
    });
});
