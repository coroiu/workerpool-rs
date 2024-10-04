import init, { init_as_worker } from "/wasm/wasm.js"

async function run() {
  await init("/wasm/wasm_bg.wasm");
  init_as_worker();
}

run().catch(console.error);
