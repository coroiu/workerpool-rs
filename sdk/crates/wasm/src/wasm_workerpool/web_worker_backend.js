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

export async function execute_task(workerIndex, taskRequest) {
  const worker = workers[workerIndex];
  return new Promise((resolve, reject) => {
    // TODO: Response ID should be checked to match the request ID
    worker.onmessage = function(e) {
      resolve(e.data);
    };
    worker.onerror = function(e) {
      reject(e);
    };
    worker.postMessage(taskRequest);
  });
}

// Main worker entry point
async function init() {
  onmessage = function(e) {
    console.log('Worker - Message received from main script:', e.data);
  }
}

function isWorker() {
  // run this in global scope of window or worker. since window.self = window, we're ok
  return typeof WorkerGlobalScope !== 'undefined' && self instanceof WorkerGlobalScope;
}

if (isWorker()) {
  init();
}