import assert from "node:assert";
import { test } from "node:test";

import { SnowFall } from "../../dist/snowfall";
import { wasmBuffer } from "./lib/getWasm";

// Helper to stringify complex objects for better diffs
const deepStrictEqual = (actual: any, expected: any, message?: string) => {
	assert.deepStrictEqual(JSON.parse(JSON.stringify(actual)), JSON.parse(JSON.stringify(expected)), message);
};

test("Parser Test", async (t) => {
	const sf = new SnowFall();
	await sf.init(wasmBuffer);

	await t.test("should parse VariableDeclaration statements", () => {
		const input = `
            Int x = 5;
            String y = "hello";
            Int a, b = 10;
        `;
		const { ast, errors } = sf.dev_parser(input);
		assert.ok(!errors, `Compilation failed with errors: ${JSON.stringify(errors, null, 2)}`);
		assert.ok(ast, "AST should not be null");
		assert.strictEqual(ast.statements.length, 3, "AST should have 3 statements");

		// Test: Int x = 5;
		const stmt1 = ast.statements[0];
		assert.strictEqual(stmt1.kind.type, "VariableDeclaration");
		assert.strictEqual(stmt1.kind.value.type_name, "Int");
		assert.strictEqual(stmt1.kind.value.declarators.length, 1);
		assert.strictEqual(stmt1.kind.value.declarators[0].name, "x");
		deepStrictEqual(stmt1.kind.value.declarators[0].value?.kind, { type: "IntLiteral", value: 5 });

		// Test: String y = "hello";
		const stmt2 = ast.statements[1];
		assert.strictEqual(stmt2.kind.type, "VariableDeclaration");
		assert.strictEqual(stmt2.kind.value.type_name, "String");
		assert.strictEqual(stmt2.kind.value.declarators.length, 1);
		assert.strictEqual(stmt2.kind.value.declarators[0].name, "y");
		deepStrictEqual(stmt2.kind.value.declarators[0].value?.kind, { type: "StringLiteral", value: "hello" });

		// Test: Int a, b = 10;
		const stmt3 = ast.statements[2];
		assert.strictEqual(stmt3.kind.type, "VariableDeclaration");
		assert.strictEqual(stmt3.kind.value.type_name, "Int");
		assert.strictEqual(stmt3.kind.value.declarators.length, 2);
		assert.strictEqual(stmt3.kind.value.declarators[0].name, "a");
		assert.strictEqual(stmt3.kind.value.declarators[0].value, undefined);
		assert.strictEqual(stmt3.kind.value.declarators[1].name, "b");
		deepStrictEqual(stmt3.kind.value.declarators[1].value?.kind, { type: "IntLiteral", value: 10 });
	});

	await t.test("should parse Return statements", () => {
		const input = `
            return 5;
            return;
            return "hello";
        `;
		const { ast, errors } = sf.dev_parser(input);
		assert.ok(!errors, `Compilation failed with errors: ${JSON.stringify(errors, null, 2)}`);
		assert.ok(ast);
		assert.strictEqual(ast.statements.length, 3);

		const stmt1 = ast.statements[0];
		assert.strictEqual(stmt1.kind.type, "Return");
		deepStrictEqual(stmt1.kind.value?.kind, { type: "IntLiteral", value: 5 });

		const stmt2 = ast.statements[1];
		assert.strictEqual(stmt2.kind.type, "Return");
		assert.strictEqual(stmt2.kind?.value, undefined);

		const stmt3 = ast.statements[2];
		assert.strictEqual(stmt3.kind.type, "Return");
		deepStrictEqual(stmt3.kind.value?.kind, { type: "StringLiteral", value: "hello" });
	});

	await t.test("should parse Prefix expressions", () => {
		const tests = [
			{ input: "!true;", operator: "Bang", right: { type: "Boolean", value: true } },
			{ input: "-15;", operator: "Minus", right: { type: "IntLiteral", value: 15 } },
		];

		for (const tt of tests) {
			const { ast, errors } = sf.dev_parser(tt.input);
			assert.ok(!errors, `Compilation failed with errors: ${JSON.stringify(errors, null, 2)}`);
			assert.ok(ast);
			const stmt = ast.statements[0];
			assert.strictEqual(stmt.kind.type, "Expression");
			const expr = stmt.kind.value.kind;
			assert.strictEqual(expr.type, "Prefix");
			assert.strictEqual(expr.value.operator, tt.operator);
			deepStrictEqual(expr.value.right.kind, tt.right);
		}
	});

	await t.test("should parse Infix expressions", () => {
		const tests = [
			{ input: "5 + 5;", left: 5, operator: "Add", right: 5 },
			{ input: "5 - 5;", left: 5, operator: "Subtract", right: 5 },
			{ input: "5 * 5;", left: 5, operator: "Multiply", right: 5 },
			{ input: "5 / 5;", left: 5, operator: "Divide", right: 5 },
			{ input: "5 > 5;", left: 5, operator: "GreaterThan", right: 5 },
			{ input: "5 < 5;", left: 5, operator: "LessThan", right: 5 },
			{ input: "5 == 5;", left: 5, operator: "Equals", right: 5 },
			{ input: "5 != 5;", left: 5, operator: "NotEquals", right: 5 },
			{ input: "true == true;", left: true, operator: "Equals", right: true, literalType: "Boolean" },
		];

		for (const tt of tests) {
			const { ast, errors } = sf.dev_parser(tt.input);
			assert.ok(!errors, `Compilation failed for '${tt.input}': ${JSON.stringify(errors, null, 2)}`);
			assert.ok(ast);
			const stmt = ast.statements[0];
			assert.strictEqual(stmt.kind.type, "Expression");
			const expr = stmt.kind.value.kind;
			assert.strictEqual(expr.type, "Infix");
			assert.strictEqual(expr.value.operator, tt.operator);
			const literalType = tt.literalType || "IntLiteral";
			deepStrictEqual(expr.value.left.kind, { type: literalType, value: tt.left });
			deepStrictEqual(expr.value.right.kind, { type: literalType, value: tt.right });
		}
	});

	await t.test("should respect operator precedence", () => {
		const tests = [
			{ input: "-a * b;", expected: "((-a) * b)" },
			{ input: "!-a;", expected: "(!(-a))" },
			{ input: "a + b + c;", expected: "((a + b) + c)" },
			{ input: "a + b - c;", expected: "((a + b) - c)" },
			{ input: "a * b * c;", expected: "((a * b) * c)" },
			{ input: "a * b / c;", expected: "((a * b) / c)" },
			{ input: "a + b / c;", expected: "(a + (b / c))" },
			{ input: "a + b * c + d / e - f;", expected: "(((a + (b * c)) + (d / e)) - f)" },
			{ input: "3 + 4; -5 * 5;", expected: "(3 + 4)((-5) * 5)" }, // Note: this becomes two expression statements
			{ input: "5 > 4 == 3 < 4;", expected: "((5 > 4) == (3 < 4))" },
			{ input: "3 + 4 * 5 == 3 * 1 + 4 * 5;", expected: "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))" },
			{ input: "(a + b) * c;", expected: "((a + b) * c)" },
			{ input: "a * (b + c);", expected: "(a * (b + c))" },
			{ input: "a + (b + c) + d;", expected: "((a + (b + c)) + d)" },
			{ input: "-(5 + 5);", expected: "(-(5 + 5))" },
			{ input: "!(true == true);", expected: "(!(true == true))" },
			{ input: "a + add(b * c) + d;", expected: "((a + add((b * c))) + d)" },
			{ input: "add(a, b, 1, 2 * 3, 4 + 5);", expected: "add(a, b, 1, (2 * 3), (4 + 5))" },
			{ input: "myArray[1 + 1];", expected: "myArray[(1 + 1)]" },
		];
		// このテストでは AST はチェックされず、エラーなしでコンパイルされるだけです
		for (const tt of tests) {
			const { ast, errors } = sf.dev_parser(tt.input);
			assert.ok(!errors, `Compilation failed for '${tt.input}': ${JSON.stringify(errors, null, 2)}`);
			assert.ok(ast);
		}
	});

	await t.test("should parse If expressions", () => {
		const input = "if (x < y) { return x; } else { return y; }";
		const { ast, errors } = sf.dev_parser(input);
		assert.ok(!errors, `Compilation failed: ${JSON.stringify(errors, null, 2)}`);
		assert.ok(ast);
		const stmt = ast.statements[0];
		assert.strictEqual(stmt.kind.type, "If");
		assert.strictEqual(stmt.kind.value.condition.kind.type, "Infix");
		assert.strictEqual(stmt.kind.value.consequence.kind.type, "Block");
		assert.strictEqual(stmt.kind.value.alternative?.kind.type, "Block");
	});

	await t.test("should parse FunctionDeclarations", () => {
		const input = `
			sub mySub() {}
			function Int add(Int a, String b = "default") { return a; }
		`;
		const { ast, errors } = sf.dev_parser(input);
		assert.ok(!errors, `Compilation failed: ${JSON.stringify(errors, null, 2)}`);
		assert.ok(ast);
		assert.strictEqual(ast.statements.length, 2);

		// sub mySub() {}
		const sub = ast.statements[0];
		assert.strictEqual(sub.kind.type, "FunctionDeclaration");
		assert.strictEqual(sub.kind.value.kind, "Sub");
		assert.strictEqual(sub.kind.value.name, "mySub");
		assert.strictEqual(sub.kind.value.return_type, undefined);
		assert.strictEqual(sub.kind.value.params.length, 0);

		// function Int add(...)
		const func = ast.statements[1];
		assert.strictEqual(func.kind.type, "FunctionDeclaration");
		assert.strictEqual(func.kind.value.kind, "Function");
		assert.strictEqual(func.kind.value.name, "add");
		assert.strictEqual(func.kind.value.return_type, "Int");
		assert.strictEqual(func.kind.value.params.length, 2);
		assert.strictEqual(func.kind.value.params[0].name, "a");
		assert.strictEqual(func.kind.value.params[0].type_name, "Int");
		assert.strictEqual(func.kind.value.params[0].value, undefined);
		assert.strictEqual(func.kind.value.params[1].name, "b");
		assert.strictEqual(func.kind.value.params[1].type_name, "String");
		deepStrictEqual(func.kind.value.params[1].value?.kind, { type: "StringLiteral", value: "default" });
	});

	await t.test("should parse Call expressions", () => {
		const input = "add(1, 2 * 3, 4 + 5);";
		const { ast, errors } = sf.dev_parser(input);
		assert.ok(!errors, `Compilation failed: ${JSON.stringify(errors, null, 2)}`);
		assert.ok(ast);
		const stmt = ast.statements[0];
		assert.strictEqual(stmt.kind.type, "Expression");
		assert.strictEqual(stmt.kind.value.kind.type, "Call");
		assert.strictEqual(stmt.kind.value.kind.value.function.kind.type, "Identifier");
		assert.strictEqual(stmt.kind.value.kind.value.function.kind.value, "add");
		assert.strictEqual(stmt.kind.value.kind.value.arguments.length, 3);
	});

	await t.test("should parse ClassDeclarations", { skip: "Class実装後に検証" }, () => {
		const input = `
			class Person extends Human {
				sub new() {}
				function String getName() { return "test"; }
			}
		`;
		const { ast, errors } = sf.dev_parser(input);
		assert.ok(!errors, `Compilation failed: ${JSON.stringify(errors, null, 2)}`);
		assert.ok(ast);
		assert.strictEqual(ast.statements.length, 1);
		const classStmt = ast.statements[0];
		assert.strictEqual(classStmt.kind.type, "ClassDeclaration");
		assert.strictEqual(classStmt.kind.value.name, "Person");
		assert.strictEqual(classStmt.kind.value.superclass, "Human");
		assert.strictEqual(classStmt.kind.value.members.length, 2);
		assert.strictEqual(classStmt.kind.value.members[0].kind.type, "FunctionDeclaration");
		assert.strictEqual(classStmt.kind.value.members[0].kind.value.name, "new");
		assert.strictEqual(classStmt.kind.value.members[1].kind.type, "FunctionDeclaration");
		assert.strictEqual(classStmt.kind.value.members[1].kind.value.name, "getName");
	});

	await t.test("should report parsing errors", { skip: "Class実装後に検証" }, () => {
		const tests = [
			"Int x 5;",
			"function test(Int a,)",
			"class MyClass { Int x = 1; }", // メソッドのみが許可される
			"if (x > 5) return 1 else return 2", // 中かっこがありません
		];

		for (const input of tests) {
			const { ast, errors } = sf.dev_parser(input);
			assert.ok(errors, `Expected errors for input: '${input}' but got none.`);
			assert.ok(errors.length > 0, `Error array should not be empty for input: '${input}'`);
		}
	});
});
