use reqwest::{Client, Method};
use anyhow::Result;

pub struct HttpClient  {
    http_client: Client,
    auth_info: AuthInfo
}

pub struct AuthInfo {
    pub endpoint: String,
    pub key: String,
}

impl HttpClient {

    pub fn new(auth_info: AuthInfo) -> Self {
        Self {
            http_client: Client::new(),
            auth_info
        }
    }

    pub async fn post(&self, path: &str, body: &str) -> Result<String> {
        let uri = format!("{}/{}", &self.auth_info.endpoint, path);
        let request = self.new_request(&uri, Method::POST)
            .body(String::from(body));

        let response = request.send().await?;

        return Ok(response.text().await?);
    }

    fn new_request(&self, url: &str, method: Method) -> reqwest::RequestBuilder {
        (&self.http_client).request(method , url)
            .header("Accept", "application/json")
            .header("Authorization", &self.auth_info.key)
            .header("User-Agent", "rust_example_gen_tool")
            .header("Content-Type", "application/json")

    }
}
