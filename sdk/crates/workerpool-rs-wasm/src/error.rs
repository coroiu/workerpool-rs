use wasm_bindgen::{JsError, JsValue};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    JsValue(JsValue),
    Anyhow(anyhow::Error),
}

impl Error {
    pub fn new(message: &str) -> Self {
        Self::Anyhow(anyhow::anyhow!(message.to_owned()))
    }
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(value)
    }
}

impl From<anyhow::Error> for Error {
    fn from(value: anyhow::Error) -> Self {
        Error::Anyhow(value)
    }
}

impl From<Error> for JsValue {
    fn from(value: Error) -> Self {
        match value {
            Error::JsValue(value) => value,
            Error::Anyhow(value) => JsValue::from_str(&format!("{:?}", value)),
        }
    }
}
impl From<Error> for JsError {
    fn from(value: Error) -> Self {
        match value {
            Error::JsValue(value) => JsError::new(&format!("{:?}", value)),
            Error::Anyhow(value) => JsError::new(&format!("{:?}", value)),
        }
    }
}
