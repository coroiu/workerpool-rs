import init, { init_as_worker } from "/wasm/wasm.js"

async function run() {
  await init("/wasm/wasm_bg.wasm");

  try {
    const result = init_as_worker();
    console.log(result);
  } catch (e) {
    console.error(e);
  }
}

run().catch(console.error);
