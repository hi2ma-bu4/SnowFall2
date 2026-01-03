import { test } from "node:test";

import { SnowFall } from "../../dist/snowfall";
import { wasmBuffer } from "./lib/getWasm";

test("Parser Test", async (t) => {
	const sf = new SnowFall();
	await sf.init(wasmBuffer);

	await t.test("", () => {});
});
