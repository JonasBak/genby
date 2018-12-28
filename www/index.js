import * as wasm from "genby";

console.log(wasm.export_height());
wasm.create();
console.log(wasm.export_height());
wasm.tick(0.15);
