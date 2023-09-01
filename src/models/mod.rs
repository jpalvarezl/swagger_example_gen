use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SinglePrompt {
    pub prompt: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ChatPrompt {
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub enum ChatRole {
    System,
    User,
    Tool,
    Assistant,
}
