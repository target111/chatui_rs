use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Default)]
pub enum ChatModel {
    #[default]
    GPT4o,
    GPT4oMini,
    GPT4Turbo,
}

impl std::fmt::Display for ChatModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for ChatModel {
    fn as_ref(&self) -> &str {
        match self {
            ChatModel::GPT4o => "gpt-4o",
            ChatModel::GPT4oMini => "gpt-4o-mini",
            ChatModel::GPT4Turbo => "gpt-4-turbo",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.content)
    }
}

impl AsRef<str> for Message {
    fn as_ref(&self) -> &str {
        &self.content
    }
}

impl From<Message> for String {
    fn from(value: Message) -> Self {
        value.content
    }
}

impl Message {
    pub fn system<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::System,
            content: content.into(),
        }
    }

    pub fn user<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::User,
            content: content.into(),
        }
    }

    pub fn assistant<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::Assistant,
            content: content.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletion {
    pub model: String,
    pub usage: TokenUsage,
    #[serde(rename = "choices")]
    pub message_choices: Vec<MessageChoice>,
}

impl ChatCompletion {
    pub fn message(&self) -> &Message {
        &self.message_choices.first().unwrap().message
    }
}

#[derive(Deserialize, Debug)]
pub struct MessageChoice {
    pub message: Message,
    pub finish_reason: String,
    pub index: u32,
}

#[derive(Deserialize, Debug)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
