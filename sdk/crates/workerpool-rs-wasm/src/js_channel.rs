use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use wasm_bindgen::prelude::*;
use web_sys::{MessageEvent, Worker};

pub struct JsChannel<I, O> {
    rx: Receiver<O>,
    _phantom: std::marker::PhantomData<I>,
}

impl<I, O> JsChannel<I, O>
where
    I: Send + Serialize + DeserializeOwned + 'static,
    O: Send + Serialize + DeserializeOwned + 'static,
{
    pub fn connect_to(worker: &Worker) -> JsChannel<I, O> {
        let (tx, rx) = channel::<O>(1);
        let listener = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
            let tx = tx.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let data = event.data();
                let output = serde_wasm_bindgen::from_value(data)
                    .expect("Worker does not communicate with non-rust code");
                tx.send(output).await.unwrap();
            });
        });
        worker
            .add_event_listener_with_callback("message", listener.as_ref().unchecked_ref())
            .unwrap();

        JsChannel {
            rx,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn recv(&mut self) -> O {
        let output = self
            .rx
            .recv()
            .await
            .expect("Worker channel should not be closed");
        output
    }
}
