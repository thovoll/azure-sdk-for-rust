#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Error>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<InnerError>>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventRoute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "endpointName")]
    pub endpoint_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}
impl EventRoute {
    pub fn new(endpoint_name: String) -> Self {
        Self {
            id: None,
            endpoint_name,
            filter: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventRouteCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EventRoute>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl EventRouteCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncomingRelationship {
    #[serde(rename = "$relationshipId", default, skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<String>,
    #[serde(rename = "$sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "$relationshipName", default, skip_serializing_if = "Option::is_none")]
    pub relationship_name: Option<String>,
    #[serde(rename = "$relationshipLink", default, skip_serializing_if = "Option::is_none")]
    pub relationship_link: Option<String>,
}
impl IncomingRelationship {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IncomingRelationshipCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<IncomingRelationship>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl IncomingRelationshipCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct InnerError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<InnerError>>,
}
impl InnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelData {
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<serde_json::Value>,
    pub id: String,
    #[serde(rename = "uploadTime", default, skip_serializing_if = "Option::is_none")]
    pub upload_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decommissioned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<serde_json::Value>,
}
impl ModelData {
    pub fn new(id: String) -> Self {
        Self {
            display_name: None,
            description: None,
            id,
            upload_time: None,
            decommissioned: None,
            model: None,
        }
    }
}
pub type NonPagedModelDataCollection = Vec<ModelData>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PagedModelDataCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ModelData>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PagedModelDataCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QueryResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<serde_json::Value>,
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl QueryResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuerySpecification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "continuationToken", default, skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}
impl QuerySpecification {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RelationshipCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<serde_json::Value>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl RelationshipCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
