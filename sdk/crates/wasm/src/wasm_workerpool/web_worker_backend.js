// TODO: We really shouldn't be importing the SDK like this, since this 
// path is set by the downstream application
import init, { init_as_worker } from "/pkg/wasm.js";

const workers = [];

export class JsTaskRequest {
  request_id;
  routine_name;
  args;
}

export class JsTaskResponse {
  request_id;
  result;
}

export function spawn_worker() {
  const worker = new Worker(import.meta.url, { type: "module" });
  workers.push(worker);
  return workers.length - 1;
}

export function execute_task(workerIndex, taskRequest, executionContext) {
  console.log(`Worker #${workerIndex} was asked to perform: `, taskRequest);
  console.log("executionContext: ", executionContext);

  const worker = workers[workerIndex];

  function responseListener(e) {
    console.log("Worker response received: ", e);
    const taskResponse = e.data;

    const response = new JsTaskResponse();
    response.request_id = taskResponse.request_id;
    response.result = taskResponse.result;
    executionContext.respond(response);

    worker.removeEventListener("message", responseListener);
  }

  worker.addEventListener("message", responseListener);
  worker.postMessage(taskRequest);
}

// Main worker entry point
async function run() {
  init().then(() => {
    const workerCount = 4;
    const client = init_as_worker();

    onmessage = function(e) {
      console.log('Worker - Message received from main script:', e.data);
      const request = e.data;
      const result = client.execute_routine(request);
      console.log('Worker - Posting message back to main script:', result);
      postMessage(result);
    }
  });
}

function isWorker() {
  // run this in global scope of window or worker. since window.self = window, we're ok
  return typeof WorkerGlobalScope !== 'undefined' && self instanceof WorkerGlobalScope;
}

if (isWorker()) {
  run();
}