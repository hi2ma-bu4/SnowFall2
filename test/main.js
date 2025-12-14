import { SnowFallSystem } from "../dist/snowfall.js";

(async () => {
	const sys = new SnowFallSystem({ debug: true });
	await sys.init();

	const code = `
Int a = 10;
String s = "Snow";
String res = s * 3;
if (a === 10) {
    res = res + "Fall";
}
res; // 最後の値が返る
`;

	console.log(await sys.run(code));
})();
