import init, { greet } from "../pkg/rust.js";

console.log("Hello from typescript");

// Import the WASM module and the `greet` function

// Initialize the WASM module
init().then(() => {
  const output = greet("WebAssembly");
  console.log("Hello from WASM:", output);
});