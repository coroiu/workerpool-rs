import { createMainApplicationClient } from './sdk';

async function main() {
  const client = await createMainApplicationClient();

  console.log(client);
}

main().then(() => console.log("Main done")).catch(console.error);


