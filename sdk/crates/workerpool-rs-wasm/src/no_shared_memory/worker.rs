use crate::Result;
use wasm_bindgen::prelude::*;
use web_sys::MessageEvent;

pub struct WasmWorker;

impl WasmWorker {
    pub fn setup() -> Result<()> {
        let global = get_worker_global_scope()?;

        let listener = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
            web_sys::console::log_1(&JsValue::from_str(&format!(
                "Worker received message: {:?}",
                event.data()
            )));
        });
        global.add_event_listener_with_callback("message", listener.as_ref().unchecked_ref())?;
        listener.forget();

        global.post_message(&JsValue::from_str("Hello from worker"))?;

        // Err(Error::new("WasmWorker setup failed"))
        web_sys::console::log_1(&JsValue::from_str("WasmWorker running"));
        Ok(())
    }
}

fn get_worker_global_scope() -> Result<web_sys::DedicatedWorkerGlobalScope> {
    use wasm_bindgen::JsCast;
    use web_sys::js_sys::global;
    use web_sys::DedicatedWorkerGlobalScope;

    // Get the global object (which could be Window or WorkerGlobalScope)
    let global = global();

    // Try casting it to WorkerGlobalScope
    Ok(global
        .dyn_into::<DedicatedWorkerGlobalScope>()
        .map_err(|e| {
            anyhow::anyhow!(
                "Failed to cast global to DedicatedWorkerGlobalScope: {:?}",
                e
            )
        })?)
}
