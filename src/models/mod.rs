use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SinglePrompt {
    pub prompt: Vec<String>
}
