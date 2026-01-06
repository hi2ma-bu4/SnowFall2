import { strict as assert } from "node:assert";
import { test } from "node:test";
import { SnowFall, VariableDeclaration } from "../../dist/snowfall";
import { wasmBuffer } from "./lib/getWasm";

// Helper to stringify complex objects for better diffs
const deepStrictEqual = (actual: any, expected: any, message?: string) => {
	assert.deepStrictEqual(JSON.parse(JSON.stringify(actual)), JSON.parse(JSON.stringify(expected)), message);
};

// 「span」プロパティを再帰的に削除するヘルパー関数
const stripMetadata = (obj: any): any => {
	if (obj === null || typeof obj !== "object") {
		return obj;
	}
	if (Array.isArray(obj)) {
		return obj.map(stripMetadata);
	}
	const newObj: any = {};
	for (const key in obj) {
		if (key !== "span") {
			newObj[key] = stripMetadata(obj[key]);
		}
	}
	return newObj;
};

test("Normalizer Test", async (t) => {
	const sf = new SnowFall();
	await sf.init(wasmBuffer);

	await t.test("should remove parentheses", () => {
		const input = "((1));";
		const { ast, errors } = sf.dev_normalize(input);
		assert.ok(!errors, `Compilation failed with errors: ${JSON.stringify(errors, null, 2)}`);

		const expectedAst = {
			statements: [
				{
					kind: {
						type: "Expression",
						value: {
							kind: {
								type: "IntLiteral",
								value: 1,
							},
						},
					},
				},
			],
		};
		deepStrictEqual(stripMetadata(ast), expectedAst);
	});

	await t.test("should eliminate dead code with if(true) and if(false)", () => {
		const input = `
      if (true) {
        Int a = 1;
      } else {
        Int a = 2;
      }
      if (false) {
        Int b = 3;
      }
    `;
		const { ast, errors } = sf.dev_normalize(input);
		assert.ok(!errors, `Compilation failed with errors: ${JSON.stringify(errors, null, 2)}`);

		const expectedAst = {
			statements: [
				{
					kind: {
						type: "Block",
						value: [
							{
								kind: {
									type: "VariableDeclaration",
									value: {
										type_name: "Int",
										declarators: [
											{
												name: "a",
												value: {
													kind: {
														type: "IntLiteral",
														value: 1,
													},
												},
											},
										],
									},
								},
							},
						],
					},
				},
			],
		};
		deepStrictEqual(stripMetadata(ast), expectedAst);
	});

	await t.test("should perform constant folding", () => {
		const input = "Int a = 1 + 2 * 3;";
		const { ast, errors } = sf.dev_normalize(input);
		assert.ok(!errors, `Compilation failed with errors: ${JSON.stringify(errors, null, 2)}`);

		const expectedAst = {
			statements: [
				{
					kind: {
						type: "VariableDeclaration",
						value: {
							type_name: "Int",
							declarators: [
								{
									name: "a",
									value: {
										kind: {
											type: "IntLiteral",
											value: 7,
										},
									},
								},
							],
						},
					},
				},
			],
		};
		deepStrictEqual(stripMetadata(ast), expectedAst);
	});

	await t.test("should sort commutative operations", () => {
		const input = "Int val = c + a + b;";
		const { ast, errors } = sf.dev_normalize(input);
		assert.ok(!errors, `Compilation failed with errors: ${JSON.stringify(errors, null, 2)}`);

		const expectedAst = {
			statements: [
				{
					kind: {
						type: "VariableDeclaration",
						value: {
							type_name: "Int",
							declarators: [
								{
									name: "val",
									value: {
										kind: {
											type: "Infix",
											value: {
												left: {
													kind: {
														type: "Infix",
														value: {
															left: {
																kind: {
																	type: "Identifier",
																	value: "a",
																},
															},
															operator: "Add",
															right: {
																kind: {
																	type: "Identifier",
																	value: "b",
																},
															},
														},
													},
												},
												operator: "Add",
												right: {
													kind: {
														type: "Identifier",
														value: "c",
													},
												},
											},
										},
									},
								},
							],
						},
					},
				},
			],
		};
		deepStrictEqual(stripMetadata(ast), expectedAst);
	});

	await t.test("should fold prefix operators", () => {
		const input = `
      Int a = -10;
      Int b = +20;
    `;
		const { ast, errors } = sf.dev_normalize(input);
		assert.ok(!errors, `Compilation failed with errors: ${JSON.stringify(errors, null, 2)}`);

		const expectedAst = {
			statements: [
				{
					kind: {
						type: "VariableDeclaration",
						value: {
							type_name: "Int",
							declarators: [
								{
									name: "a",
									value: {
										kind: {
											type: "IntLiteral",
											value: -10,
										},
									},
								},
							],
						},
					},
				},
				{
					kind: {
						type: "VariableDeclaration",
						value: {
							type_name: "Int",
							declarators: [
								{
									name: "b",
									value: {
										kind: {
											type: "IntLiteral",
											value: 20,
										},
									},
								},
							],
						},
					},
				},
			],
		};
		deepStrictEqual(stripMetadata(ast), expectedAst);
	});

	await t.test("should fold floating point numbers", () => {
		const input = `
      Float a = 1.5 + 2.5;
      Float b = 5.0 - 1.5;
      Float c = 2.0 * 3.0;
      Float d = 10.0 / 4.0;
      Float e = 1 + 2.5;
    `;
		const { ast, errors } = sf.dev_normalize(input);
		assert.ok(!errors, `Compilation failed with errors: ${JSON.stringify(errors, null, 2)}`);
		assert.ok(ast, "Compilation failed");

		const declarators = ast.statements.map((s) => (s.kind as { type: "VariableDeclaration"; value: VariableDeclaration }).value.declarators[0]);
		deepStrictEqual(stripMetadata(declarators[0].value?.kind), { type: "FloatLiteral", value: 4.0 });
		deepStrictEqual(stripMetadata(declarators[1].value?.kind), { type: "FloatLiteral", value: 3.5 });
		deepStrictEqual(stripMetadata(declarators[2].value?.kind), { type: "FloatLiteral", value: 6.0 });
		deepStrictEqual(stripMetadata(declarators[3].value?.kind), { type: "FloatLiteral", value: 2.5 });
		deepStrictEqual(stripMetadata(declarators[4].value?.kind), { type: "FloatLiteral", value: 3.5 });
	});
});
