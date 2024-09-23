import init, { greet, init_as_main } from "../pkg/wasm.js";

console.log("Hello from typescript");

function spawn_worker() {

}

// Import the WASM module and the `greet` function

// Initialize the WASM module
init().then(() => {
  const workerCount = 4;
  const client = init_as_main(workerCount);

  const output = document.querySelector("#output")
  function log(...args: any[]) {
    output!.textContent += Array.from(args).join(" ") + "\n";
    console.log(...args);
  }

  document.querySelector("#run-test-direct")?.addEventListener("click", async () => {
    log("Running test: Direct...");

    document.querySelector("#spinner")?.classList.remove("hidden");
    client.run_direct();
    document.querySelector("#spinner")?.classList.add("hidden");

    log("Test completed");
  });

  document.querySelector("#run-test-same-thread")?.addEventListener("click", async () => {
    log("Running test: Same thread...");

    document.querySelector("#spinner")?.classList.remove("hidden");
    await client.run_same_thread();
    document.querySelector("#spinner")?.classList.add("hidden");

    log("Test completed");
  });

  document.querySelector("#run-test-web-worker")?.addEventListener("click", async () => {
    log("Running test: Web worker...");

    document.querySelector("#spinner")?.classList.remove("hidden");
    await client.run_in_worker();
    document.querySelector("#spinner")?.classList.add("hidden");

    log("Test completed");
  });
});