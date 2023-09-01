use anyhow::Result;
use dotenv::dotenv;
use http_client::{AuthInfo, OpenAIClient};
use models::SinglePrompt;
use std::env;

mod http_client;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect(".env file could not be read.");
    let client = build_client();

    let body = SinglePrompt {
        prompt: vec![String::from("Write a haiku about onions")],
    };

    let response_test = client
        .post(
            "openai/deployments/gpt-35-turbo/completions?api-version=2023-08-01-preview",
            serde_json::to_string(&body).expect("Serialization problem"),
        )
        .await;

    print!("Result: {}", response_test?);
    Ok(())
}

fn build_client() -> OpenAIClient {
    let endpoint = env::var("AZURE_OPENAI_ENDPOINT")
        .expect("Undefined environment variable: AZURE_OPENAI_ENDPOINT");
    let key =
        env::var("AZURE_OPENAI_KEY").expect("Undefined environment variable: AZURE_OPENAI_KEY");

    OpenAIClient::new(AuthInfo { endpoint, key })
}
