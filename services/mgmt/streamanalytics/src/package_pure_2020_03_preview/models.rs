#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<ClusterSku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterProperties>,
}
impl Cluster {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterJob {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "streamingUnits", default, skip_serializing_if = "Option::is_none")]
    pub streaming_units: Option<i32>,
    #[serde(rename = "jobState", default, skip_serializing_if = "Option::is_none")]
    pub job_state: Option<JobState>,
}
impl ClusterJob {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterJobListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ClusterJob>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ClusterJobListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ClusterListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterProperties {
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<ClusterProvisioningState>,
    #[serde(rename = "capacityAllocated", default, skip_serializing_if = "Option::is_none")]
    pub capacity_allocated: Option<i32>,
    #[serde(rename = "capacityAssigned", default, skip_serializing_if = "Option::is_none")]
    pub capacity_assigned: Option<i32>,
}
impl ClusterProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ClusterProvisioningState {
    Succeeded,
    Failed,
    Canceled,
    InProgress,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ClusterSku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<cluster_sku::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl ClusterSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod cluster_sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        Default,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error::Error>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub target: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub details: Vec<ErrorDetails>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum JobState {
    Created,
    Starting,
    Running,
    Stopping,
    Stopped,
    Deleting,
    Failed,
    Degraded,
    Restarting,
    Scaling,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpoint>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PrivateEndpointListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointProperties {
    #[serde(rename = "createdDate", default, skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "manualPrivateLinkServiceConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub manual_private_link_service_connections: Vec<PrivateLinkServiceConnection>,
}
impl PrivateEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkConnectionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkServiceConnectionProperties>,
}
impl PrivateLinkServiceConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionProperties {
    #[serde(rename = "privateLinkServiceId", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_id: Option<String>,
    #[serde(rename = "groupIds", default, skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<String>,
    #[serde(rename = "requestMessage", default, skip_serializing_if = "Option::is_none")]
    pub request_message: Option<String>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkConnectionState>,
}
impl PrivateLinkServiceConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl TrackedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
