import { createMainApplicationClient } from './sdk';

async function main() {
  const client = await createMainApplicationClient();

  const result = await client.run_in_worker(9000);
  console.log({ result });
}

main().then(() => console.log("Main done")).catch(console.error);
