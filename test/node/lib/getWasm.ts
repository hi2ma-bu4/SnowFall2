import fs from "node:fs/promises";
import path from "node:path";

const wasmPath = path.join(process.cwd(), "./dist/snowfall_core_bg.wasm");
export const wasmBuffer = await fs.readFile(wasmPath);
