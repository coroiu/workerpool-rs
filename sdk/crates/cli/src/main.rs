mod routines;
mod same_thread;

#[tokio::main]
async fn main() {
    same_thread::manual_same_thread_test().await;
}
