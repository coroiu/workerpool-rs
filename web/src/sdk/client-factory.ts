import init, { type MainApplicationClient, init_as_main } from "@sdk/wasm";
import wasm from "@sdk/wasm/wasm_bg.wasm";

export async function createMainApplicationClient(
  ...args: ConstructorParameters<typeof MainApplicationClient>
): Promise<MainApplicationClient> {
  await init(wasm);

  return Promise.resolve(init_as_main(0));
}
