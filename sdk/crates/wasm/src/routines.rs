use chrono::{prelude::*, Duration};

use workerpool::global::*;
use workerpool_macro::global_routine;

struct SleepThenAddInput {
    pub seconds: u8,
    pub a: u8,
    pub b: u8,
}

impl From<Vec<u8>> for SleepThenAddInput {
    fn from(data: Vec<u8>) -> Self {
        let seconds = data[0] as u8;
        let a = data[1] as u8;
        let b = data[2] as u8;
        SleepThenAddInput { seconds, a, b }
    }
}

// TODO: global not working because ctor does not support WASM
// #[global_routine]
pub fn sleep_then_add(input: Vec<u8>) -> Result<Vec<u8>, ()> {
    let SleepThenAddInput { seconds, a, b } = SleepThenAddInput::from(input);
    sleep_blocking(Duration::seconds(seconds.into()));
    Ok(vec![a + b])
}

fn sleep_blocking(duration: Duration) {
    let start = Utc::now();
    let end = start + duration;
    while Utc::now() < end {}
}
