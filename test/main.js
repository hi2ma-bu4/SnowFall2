import { SnowFall } from "../dist/snowfall.js";

const sf = new SnowFall();

await sf.init();

const sfc = sf.getCompiler();

window.sf = sf;
window.sfc = sfc;
