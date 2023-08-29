use azure_core::auth::TokenCredential;
use azure_identity::{DefaultAzureCredential};

pub fn authenticate() {
    let credential = DefaultAzureCredential::default();
    credential.get_token(resource)
}
