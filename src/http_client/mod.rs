use anyhow::Result;
use azure_core::{
    headers::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
    new_http_client, HttpClient, Method, Request, Url,
};
use std::sync::Arc;

pub struct OpenAIClient {
    http_client: Arc<dyn HttpClient>,
    auth_info: AuthInfo,
}

pub struct AuthInfo {
    pub endpoint: String,
    pub key: String,
}

impl OpenAIClient {
    pub fn new(auth_info: AuthInfo) -> Self {
        Self {
            http_client: new_http_client(),
            auth_info,
        }
    }

    pub async fn post(&self, path: &str, body: String) -> Result<String> {
        let uri = format!("{}/{}", &self.auth_info.endpoint, path);
        let mut request = self.new_request(&uri, Method::Post);
        request.set_body(body);

        let response = (&self.http_client.as_ref())
            .execute_request(&request)
            .await?;

        return Ok(response.into_body().collect_string().await?);
    }

    fn new_request(&self, url: &str, method: Method) -> Request {
        let request = {
            let mut request = Request::new(
                Url::parse(url).expect(&format!("Failed to parse URL: {}", url)),
                method,
            );

            request.insert_header(ACCEPT, "application/json");
            request.insert_header(AUTHORIZATION, format!("Bearer {}", &self.auth_info.key));
            request.insert_header(USER_AGENT, "rust_example_gen_tool");
            request.insert_header(CONTENT_TYPE, "application/json");

            request
        };

        request
    }
}
