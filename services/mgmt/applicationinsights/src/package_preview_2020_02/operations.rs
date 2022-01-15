#![doc = "generated by AutoRust"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use super::models;
#[derive(Clone)]
pub struct Client {
    endpoint: String,
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    scopes: Vec<String>,
    pipeline: azure_core::Pipeline,
}
#[derive(Clone)]
pub struct ClientBuilder {
    credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
    endpoint: Option<String>,
    scopes: Option<Vec<String>>,
}
pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
impl ClientBuilder {
    pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
        Self {
            credential,
            endpoint: None,
            scopes: None,
        }
    }
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }
    pub fn scopes(mut self, scopes: &[&str]) -> Self {
        self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
        self
    }
    pub fn build(self) -> Client {
        let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
        let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
        Client::new(endpoint, self.credential, scopes)
    }
}
impl Client {
    pub(crate) fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }
    pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
        self.credential.as_ref()
    }
    pub(crate) fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(String::as_str).collect()
    }
    pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> Result<azure_core::Response, azure_core::Error> {
        let mut context = azure_core::Context::default();
        let mut request = request.into();
        self.pipeline.send(&mut context, &mut request).await
    }
    pub fn new(
        endpoint: impl Into<String>,
        credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
        scopes: Vec<String>,
    ) -> Self {
        let endpoint = endpoint.into();
        let pipeline = azure_core::Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            azure_core::ClientOptions::default(),
            Vec::new(),
            Vec::new(),
        );
        Self {
            endpoint,
            credential,
            scopes,
            pipeline,
        }
    }
    pub fn operations(&self) -> operations::Client {
        operations::Client(self.clone())
    }
}
#[non_exhaustive]
#[derive(Debug, thiserror :: Error)]
#[allow(non_camel_case_types)]
pub enum Error {
    #[error(transparent)]
    Operations_List(#[from] operations::list::Error),
    #[error(transparent)]
    GetTestResultFile(#[from] get_test_result_file::Error),
}
pub mod operations {
    use super::models;
    pub struct Client(pub(crate) super::Client);
    impl Client {
        #[doc = "List available operations."]
        pub fn list(&self) -> list::Builder {
            list::Builder { client: self.0.clone() }
        }
    }
    pub mod list {
        use super::models;
        #[derive(Debug, thiserror :: Error)]
        pub enum Error {
            #[error("HTTP status code {}", status_code)]
            DefaultResponse { status_code: http::StatusCode },
            #[error("Failed to parse request URL")]
            ParseUrl(#[source] url::ParseError),
            #[error("Failed to build request")]
            BuildRequest(#[source] http::Error),
            #[error("Failed to serialize request body")]
            Serialize(#[source] serde_json::Error),
            #[error("Failed to get access token")]
            GetToken(#[source] azure_core::Error),
            #[error("Failed to execute request")]
            SendRequest(#[source] azure_core::Error),
            #[error("Failed to get response bytes")]
            ResponseBytes(#[source] azure_core::StreamError),
            #[error("Failed to deserialize response, body: {1:?}")]
            Deserialize(#[source] serde_json::Error, bytes::Bytes),
        }
        #[derive(Clone)]
        pub struct Builder {
            pub(crate) client: super::super::Client,
        }
        impl Builder {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::OperationsListResult, Error>> {
                Box::pin(async move {
                    let url_str = &format!("{}/providers/microsoft.insights/operations", self.client.endpoint(),);
                    let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                    let mut req_builder = http::request::Builder::new();
                    req_builder = req_builder.method(http::Method::GET);
                    let credential = self.client.token_credential();
                    let token_response = credential
                        .get_token(&self.client.scopes().join(" "))
                        .await
                        .map_err(Error::GetToken)?;
                    req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                    url.query_pairs_mut().append_pair("api-version", "2020-02-10-preview");
                    let req_body = azure_core::EMPTY_BODY;
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                    let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        http::StatusCode::OK => {
                            let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                            let rsp_value: models::OperationsListResult =
                                serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                            Ok(rsp_value)
                        }
                        status_code => Err(Error::DefaultResponse { status_code }),
                    }
                })
            }
        }
    }
}
impl Client {
    pub fn get_test_result_file(
        &self,
        resource_group_name: impl Into<String>,
        subscription_id: impl Into<String>,
        web_test_name: impl Into<String>,
        geo_location_id: impl Into<String>,
        time_stamp: i64,
        download_as: impl Into<String>,
    ) -> get_test_result_file::Builder {
        get_test_result_file::Builder {
            client: self.clone(),
            resource_group_name: resource_group_name.into(),
            subscription_id: subscription_id.into(),
            web_test_name: web_test_name.into(),
            geo_location_id: geo_location_id.into(),
            time_stamp,
            download_as: download_as.into(),
            test_successful_criteria: None,
            continuation_token: None,
        }
    }
}
pub mod get_test_result_file {
    use super::models;
    #[derive(Debug, thiserror :: Error)]
    pub enum Error {
        #[error("HTTP status code {}", status_code)]
        DefaultResponse {
            status_code: http::StatusCode,
            value: models::ErrorResponse,
        },
        #[error("Failed to parse request URL")]
        ParseUrl(#[source] url::ParseError),
        #[error("Failed to build request")]
        BuildRequest(#[source] http::Error),
        #[error("Failed to serialize request body")]
        Serialize(#[source] serde_json::Error),
        #[error("Failed to get access token")]
        GetToken(#[source] azure_core::Error),
        #[error("Failed to execute request")]
        SendRequest(#[source] azure_core::Error),
        #[error("Failed to get response bytes")]
        ResponseBytes(#[source] azure_core::StreamError),
        #[error("Failed to deserialize response, body: {1:?}")]
        Deserialize(#[source] serde_json::Error, bytes::Bytes),
    }
    #[derive(Clone)]
    pub struct Builder {
        pub(crate) client: super::Client,
        pub(crate) resource_group_name: String,
        pub(crate) subscription_id: String,
        pub(crate) web_test_name: String,
        pub(crate) geo_location_id: String,
        pub(crate) time_stamp: i64,
        pub(crate) download_as: String,
        pub(crate) test_successful_criteria: Option<bool>,
        pub(crate) continuation_token: Option<String>,
    }
    impl Builder {
        pub fn test_successful_criteria(mut self, test_successful_criteria: bool) -> Self {
            self.test_successful_criteria = Some(test_successful_criteria);
            self
        }
        pub fn continuation_token(mut self, continuation_token: impl Into<String>) -> Self {
            self.continuation_token = Some(continuation_token.into());
            self
        }
        pub fn into_future(self) -> futures::future::BoxFuture<'static, std::result::Result<models::TestResultFileResponse, Error>> {
            Box::pin(async move {
                let url_str = &format!(
                    "{}/subscriptions/{}/resourcegroups/{}/providers/microsoft.insights/webtests/{}/getTestResultFile",
                    self.client.endpoint(),
                    &self.subscription_id,
                    &self.resource_group_name,
                    &self.web_test_name
                );
                let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                let mut req_builder = http::request::Builder::new();
                req_builder = req_builder.method(http::Method::POST);
                let credential = self.client.token_credential();
                let token_response = credential
                    .get_token(&self.client.scopes().join(" "))
                    .await
                    .map_err(Error::GetToken)?;
                req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
                url.query_pairs_mut().append_pair("api-version", "2020-02-10-preview");
                let geo_location_id = &self.geo_location_id;
                url.query_pairs_mut().append_pair("geoLocationId", geo_location_id);
                let time_stamp = &self.time_stamp;
                url.query_pairs_mut().append_pair("timeStamp", &time_stamp.to_string());
                let download_as = &self.download_as;
                url.query_pairs_mut().append_pair("downloadAs", download_as);
                if let Some(test_successful_criteria) = &self.test_successful_criteria {
                    url.query_pairs_mut()
                        .append_pair("testSuccessfulCriteria", &test_successful_criteria.to_string());
                }
                if let Some(continuation_token) = &self.continuation_token {
                    url.query_pairs_mut().append_pair("continuationToken", continuation_token);
                }
                let req_body = azure_core::EMPTY_BODY;
                req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
                req_builder = req_builder.uri(url.as_str());
                let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                match rsp_status {
                    http::StatusCode::OK => {
                        let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                        let rsp_value: models::TestResultFileResponse =
                            serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                        Ok(rsp_value)
                    }
                    status_code => {
                        let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                        let rsp_value: models::ErrorResponse =
                            serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
                        Err(Error::DefaultResponse {
                            status_code,
                            value: rsp_value,
                        })
                    }
                }
            })
        }
    }
}
