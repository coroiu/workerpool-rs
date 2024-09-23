const workers = [];

export function spawn_worker() {
  const worker = new Worker(import.meta.url, { type: "module" });
  workers.push(worker);
  return workers.length - 1;
}

// Main worker entry point
async function init() {
  onmessage = function(e) {
    console.log('Worker: Message received from main script', e.data);
  }
}

function isWorker() {
  // run this in global scope of window or worker. since window.self = window, we're ok
  return typeof WorkerGlobalScope !== 'undefined' && self instanceof WorkerGlobalScope;
}

if (isWorker()) {
  init();
}