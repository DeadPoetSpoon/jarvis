use std::fmt::Display;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub enum Message {
    SimpleMessage(String),
    Error(String),
}

impl Default for Message {
    fn default() -> Self {
        Message::SimpleMessage("Hello !".to_string())
    }
}

impl Message {
    pub fn pkg_error<T>(err: T) -> Self
    where
        T: Display,
    {
        Message::Error(format!("Error: {err}"))
    }
    pub fn pkg_simple<T>(str: T) -> Self
    where
        T: Display,
    {
        Message::SimpleMessage(format!("From Jarvis: {str}"))
    }
}
