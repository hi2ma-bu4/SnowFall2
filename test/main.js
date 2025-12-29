import { SnowFall } from "../dist/snowfall.js";

const sf = new SnowFall(true);

await sf.init();

window.sf = sf;
