import assert from "node:assert";
import { test } from "node:test";

import { KeywordToken, LiteralToken, SnowFall, type DelimiterToken, type OperatorToken } from "../../dist/snowfall";
import { wasmBuffer } from "./lib/getWasm";

test("Lexer Test", async (t) => {
	const sf = new SnowFall();
	await sf.init(wasmBuffer);

	await t.test("should tokenize Operators", () => {
		const data = {
			"=": "Assign",
			"==": "Equal",
			"===": "StrictEqual",
			"+": "Plus",
			"-": "Minus",
			"*": "Asterisk",
			"**": "Power",
			"/": "Slash",
			"%": "Percent",
			"!": "Bang",
			"!=": "NotEqual",
			"!==": "StrictNotEqual",
			"<": "LessThan",
			"<=": "LessThanOrEqual",
			">": "GreaterThan",
			">=": "GreaterThanOrEqual",
		};
		const input = Object.keys(data).join(" ");
		const expectedTokens = Object.values(data).map((type) => ({ type }));
		const tokens = sf.dev_lexer(input);

		if (tokens.length !== expectedTokens.length) {
			assert.deepStrictEqual(tokens, expectedTokens, `Token length mismatch: expected ${expectedTokens.length}, got ${tokens.length}`);
		}

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(tokens[i].kind.type, "Operator");
		}

		const filteredTokens: OperatorToken[] = tokens.map((t) => (t.kind as any).value);

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(filteredTokens[i].type, expectedTokens[i].type, `Token [${Object.keys(data)[i]}] type mismatch`);
		}
	});

	await t.test("should tokenize Logical Operators", () => {
		const data = {
			"&&": "LogicalAnd",
			"||": "LogicalOr",
		};
		const input = Object.keys(data).join(" ");
		const expectedTokens = Object.values(data).map((type) => ({ type }));
		const tokens = sf.dev_lexer(input);

		if (tokens.length !== expectedTokens.length) {
			assert.deepStrictEqual(tokens, expectedTokens, `Token length mismatch: expected ${expectedTokens.length}, got ${tokens.length}`);
		}

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(tokens[i].kind.type, "Operator");
		}

		const filteredTokens: OperatorToken[] = tokens.map((t) => (t.kind as any).value);

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(filteredTokens[i].type, expectedTokens[i].type, `Token [${Object.keys(data)[i]}] type mismatch`);
		}
	});

	await t.test("should tokenize Bitwise Operators", () => {
		const data = {
			"&": "BitwiseAnd",
			"|": "BitwiseOr",
			"^": "BitwiseXor",
			"~": "BitwiseNot",
			"<<": "BitwiseLeftShift",
			"<<<": "BitwiseUnsignedLeftShift",
			">>": "BitwiseRightShift",
			">>>": "BitwiseUnsignedRightShift",
		};
		const input = Object.keys(data).join(" ");
		const expectedTokens = Object.values(data).map((type) => ({ type }));
		const tokens = sf.dev_lexer(input);

		if (tokens.length !== expectedTokens.length) {
			assert.deepStrictEqual(tokens, expectedTokens, `Token length mismatch: expected ${expectedTokens.length}, got ${tokens.length}`);
		}

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(tokens[i].kind.type, "Operator");
		}

		const filteredTokens: OperatorToken[] = tokens.map((t) => (t.kind as any).value);

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(filteredTokens[i].type, expectedTokens[i].type, `Token [${Object.keys(data)[i]}] type mismatch`);
		}
	});

	await t.test("should tokenize Delimiters", () => {
		const data = {
			".": "Dot",
			",": "Comma",
			":": "Colon",
			";": "Semicolon",
			"(": "LParen",
			")": "RParen",
			"{": "LBrace",
			"}": "RBrace",
			"[": "LBracket",
			"]": "RBracket",
		};
		const input = Object.keys(data).join(" ");
		const expectedTokens = Object.values(data).map((type) => ({ type }));
		const tokens = sf.dev_lexer(input);

		if (tokens.length !== expectedTokens.length) {
			assert.deepStrictEqual(tokens, expectedTokens, `Token length mismatch: expected ${expectedTokens.length}, got ${tokens.length}`);
		}

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(tokens[i].kind.type, "Delimiter");
		}

		const filteredTokens: DelimiterToken[] = tokens.map((t) => (t.kind as any).value);

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(filteredTokens[i].type, expectedTokens[i].type, `Token [${Object.keys(data)[i]}] type mismatch`);
		}
	});

	await t.test("should tokenize Keywords", () => {
		const data = {
			function: "Function",
			sub: "Sub",
			class: "Class",
			extends: "Extends",
			constructor: "Constructor",
			new: "New",
			if: "If",
			else: "Else",
			for: "For",
			while: "While",
			in: "In",
			of: "Of",
			switch: "Switch",
			case: "Case",
			default: "Default",
			break: "Break",
			continue: "Continue",
			return: "Return",
			true: "True",
			false: "False",
			null: "Null",
			and: "And",
			or: "Or",
		};
		const input = Object.keys(data).join(" ");
		const expectedTokens = Object.values(data).map((type) => ({ type }));
		const tokens = sf.dev_lexer(input);

		if (tokens.length !== expectedTokens.length) {
			assert.deepStrictEqual(tokens, expectedTokens, `Token length mismatch: expected ${expectedTokens.length}, got ${tokens.length}`);
		}

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(tokens[i].kind.type, "Keyword");
		}

		const filteredTokens: KeywordToken[] = tokens.map((t) => (t.kind as any).value);

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(filteredTokens[i].type, expectedTokens[i].type, `Token [${Object.keys(data)[i]}] type mismatch`);
		}
	});

	await t.test("should tokenize Number Literals", () => {
		const data = {
			"0": "Int",
			"10": "Int",
			"99": "Int",
			"1_000": "Int",
			"0.0": "Float",
			"10.01": "Float",
			"10.": "Float",
			// ".01": "Float", // TODO: 対応させる
			"9_999.999_9": "Float",
			"1__0": "Int",
			"0x0": "Int",
			"0x10": "Int",
			"0xFF": "Int",
			"0xee": "Int",
			"0XDD": "Int",
			"0x1_000": "Int",
			"0x0.0": "Float",
			"0x10.01": "Float",
			"0x10.": "Float",
			"0x1_000.000_1": "Float",
			"0b0": "Int",
			"0b10": "Int",
			"0b11": "Int",
			"0B11": "Int",
			"0b1_000": "Int",
			"0b0.0": "Float",
			"0b10.01": "Float",
			"0b10.": "Float",
			"0b1_000.000_1": "Float",
		};
		const input = Object.keys(data).join(" ");
		const expectedTokens = Object.values(data).map((type) => ({ type }));
		const tokens = sf.dev_lexer(input);

		if (tokens.length !== expectedTokens.length) {
			assert.deepStrictEqual(tokens, expectedTokens, `Token length mismatch: expected ${expectedTokens.length}, got ${tokens.length}`);
		}

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(tokens[i].kind.type, "Literal");
		}

		const filteredTokens: LiteralToken[] = tokens.map((t) => (t.kind as any).value);

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(filteredTokens[i].type, expectedTokens[i].type, `Token [${Object.keys(data)[i]}] type mismatch: ${JSON.stringify(tokens[i])}`);
		}
	});

	await t.test("should tokenize Illegal Number Literals", () => {
		const data = [
			//
			"1_",
			"1_.0",
			"1._0",
			"0xZ",
			"0x0.Z",
			"0x.01",
			"0b2",
			"0b0.2",
			"0b.01",
		];

		for (const input of data) {
			assert.throws(
				() => {
					sf.dev_lexer(input);
				},
				new RegExp("Lexer error"),
				`Expected lexer to throw for input: ${input}`
			);
		}
	});

	await t.test("should tokenize String Literals", () => {
		const data = {
			'""': "String",
			'"a"': "String",
			'"abcdefghijklmnopqrstuvwxyz"': "String",
			'"あいうえお"': "String",
			'"🍣🍺"': "String",
			"''": "String",
			"'apple'": "String",
			'"foo\'bar"': "String",
			'"hoge\\"fuga"': "String",
			'"abc\ndef"': "String",
		};
		const input = Object.keys(data).join(" ");
		const expectedTokens = Object.values(data).map((type) => ({ type }));
		const tokens = sf.dev_lexer(input);

		if (tokens.length !== expectedTokens.length) {
			assert.deepStrictEqual(tokens, expectedTokens, `Token length mismatch: expected ${expectedTokens.length}, got ${tokens.length}`);
		}

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(tokens[i].kind.type, "Literal");
		}

		const filteredTokens: LiteralToken[] = tokens.map((t) => (t.kind as any).value);

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(filteredTokens[i].type, expectedTokens[i].type, `Token [${Object.keys(data)[i]}] type mismatch: ${JSON.stringify(tokens[i])}`);
		}
	});

	await t.test("should tokenize Illegal String Literals", () => {
		const data = [
			//
			'"""',
			"'''",
			'"\\',
		];

		for (const input of data) {
			assert.throws(
				() => {
					sf.dev_lexer(input);
				},
				new RegExp("Lexer error"),
				`Expected lexer to throw for input: ${input}`
			);
		}
	});

	await t.test("should tokenize Identifier", () => {
		const data = {
			apple: "Identifier",
			banana: "Identifier",
			elif: "Identifier",
			_value: "Identifier",
		};
		const input = Object.keys(data).join(" ");
		const expectedTokens = Object.values(data).map((type) => ({ type }));
		const tokens = sf.dev_lexer(input);

		if (tokens.length !== expectedTokens.length) {
			assert.deepStrictEqual(tokens, expectedTokens, `Token length mismatch: expected ${expectedTokens.length}, got ${tokens.length}`);
		}

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(tokens[i].kind.type, "Identifier");
		}

		const filteredTokens = tokens.map((t) => t.kind);

		for (let i = 0; i < tokens.length; i++) {
			assert.strictEqual(filteredTokens[i].type, expectedTokens[i].type, `Token [${Object.keys(data)[i]}] type mismatch`);
		}
	});

	await t.test("should tokenize Illegal Identifier", () => {
		const data = [
			//
			"あいうえお",
			"🍣🍺",
		];

		for (const input of data) {
			assert.throws(
				() => {
					sf.dev_lexer(input);
				},
				new RegExp("Lexer error"),
				`Expected lexer to throw for input: ${input}`
			);
		}
	});
});
