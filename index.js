import init, { wasm_main } from "./pkg/amazes.js";

console.log("hi")
async function run() {
    await init();
    wasm_main();
}

run();