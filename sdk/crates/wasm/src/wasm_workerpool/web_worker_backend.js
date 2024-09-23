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
async function init() {
  onmessage = function(e) {
    console.log('Worker - Message received from main script:', e.data);
    postMessage({ request_id: e.data.request_id, result: [0] });
  }
}

function isWorker() {
  // run this in global scope of window or worker. since window.self = window, we're ok
  return typeof WorkerGlobalScope !== 'undefined' && self instanceof WorkerGlobalScope;
}

if (isWorker()) {
  init();
}