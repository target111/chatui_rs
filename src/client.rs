use derive_builder::Builder;
use std::time::Duration;
use ureq::{Agent, AgentBuilder};

use crate::{
    error::Error,
    types::{ChatCompletion, ChatModel, Message},
};

static API_ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Debug, Clone, Builder)]
#[builder(pattern = "mutable")]
pub struct OpenAIConfig {
    #[builder(setter(into))]
    api_key: String,
    #[builder(default = "ChatModel::GPT4o")]
    model: ChatModel,
    #[builder(default = "1.0")]
    temperature: f32,
    #[builder(default)]
    max_tokens: Option<u32>,
    #[builder(default)]
    top_p: Option<f32>,
    #[builder(default)]
    frequency_penalty: Option<f32>,
    #[builder(default)]
    presence_penalty: Option<f32>,
    #[builder(default = "10")]
    timeout: u64,
}

pub struct OpenAIClient {
    config: OpenAIConfig,
    agent: Agent,
    conversation: Vec<Message>,
}

impl OpenAIClient {
    pub fn new(config: OpenAIConfig) -> Self {
        let agent = AgentBuilder::new()
            .timeout_read(Duration::from_secs(config.timeout))
            .timeout_write(Duration::from_secs(config.timeout))
            .build();

        Self {
            config,
            agent,
            conversation: Vec::new(),
        }
    }

    pub fn send_message(self, content: &str) -> crate::Result<ChatCompletion> {
        let message = vec![Message::user(content)];
        self.send_request(&message)
    }

    pub fn chat(&mut self, content: &str) -> crate::Result<String> {
        self.conversation.push(Message::user(content));

        self.conversation.push(Message::assistant(
            self.send_request(&self.conversation)?.message().as_ref(),
        ));

        Ok(self.conversation.last().unwrap().content.clone())
    }

    fn send_request(&self, messages: &[Message]) -> crate::Result<ChatCompletion> {
        let request_body = self.build_request_body(messages);

        let response = self
            .agent
            .post(API_ENDPOINT)
            .set("Authorization", &format!("Bearer {}", self.config.api_key))
            .set("Content-Type", "application/json")
            .send_json(&request_body)
            .map_err(Error::RequestFailed)?;

        // Parse response into json value first because otherwise the Result is io::Error
        let chat_completion: ChatCompletion =
            serde_json::from_value(response.into_json::<serde_json::Value>()?)?;

        Ok(chat_completion)
    }

    fn build_request_body(&self, messages: &[Message]) -> serde_json::Value {
        let mut request_body = serde_json::json!({
            "model": self.config.model.as_ref(),
            "messages": messages,
            "temperature": self.config.temperature,
        });

        if let Some(max_tokens) = self.config.max_tokens {
            request_body["max_tokens"] = serde_json::json!(max_tokens);
        }
        if let Some(top_p) = self.config.top_p {
            request_body["top_p"] = serde_json::json!(top_p);
        }
        if let Some(frequency_penalty) = self.config.frequency_penalty {
            request_body["frequency_penalty"] = serde_json::json!(frequency_penalty);
        }
        if let Some(presence_penalty) = self.config.presence_penalty {
            request_body["presence_penalty"] = serde_json::json!(presence_penalty);
        }

        request_body
    }

    pub fn clear_conversation(&mut self) {
        self.conversation.clear();
    }
}
