pub mod client;
pub mod error;
pub mod types;

pub use client::OpenAIClient;
pub use error::Error;
pub use types::{ChatModel, Message, Role};

pub type Result<T> = std::result::Result<T, Error>;
