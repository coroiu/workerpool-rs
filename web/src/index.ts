import init, { greet, init_as_main } from "../pkg/wasm.js";

console.log("Hello from typescript");

// Import the WASM module and the `greet` function

// Initialize the WASM module
init().then(() => {
  const output = greet("WebAssembly");
  console.log("Hello from WASM:", output);

  const workerCount = 4;
  const client = init_as_main(workerCount);

  document.querySelector("#run-test-btn")?.addEventListener("click", async () => {
    console.log("Running test...");

    document.querySelector("#spinner")?.classList.remove("hidden");
    await new Promise((resolve) => setTimeout(resolve, 1000));
    document.querySelector("#spinner")?.classList.add("hidden");
  });
});