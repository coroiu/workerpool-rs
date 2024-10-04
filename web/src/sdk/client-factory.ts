import { MainApplicationClient } from "@sdk/wasm";

export async function createMainApplicationClient(
  ...args: ConstructorParameters<typeof MainApplicationClient>
): Promise<MainApplicationClient> {
  return Promise.resolve(new MainApplicationClient(...args));
}
