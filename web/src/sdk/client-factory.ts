import init, { type MainApplicationClient, init_as_main } from "@sdk/wasm";
import wasm from "@sdk/wasm/wasm_bg.wasm";

export async function createMainApplicationClient(): Promise<MainApplicationClient> {
  await init(wasm);

  const worker_url = new URL("../worker.ts", import.meta.url).href;
  return Promise.resolve(init_as_main(worker_url));
}
