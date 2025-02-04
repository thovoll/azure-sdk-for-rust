#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Acl {
    #[serde(rename = "initiatorIqn")]
    pub initiator_iqn: String,
    #[serde(rename = "mappedLuns")]
    pub mapped_luns: Vec<String>,
}
impl Acl {
    pub fn new(initiator_iqn: String, mapped_luns: Vec<String>) -> Self {
        Self {
            initiator_iqn,
            mapped_luns,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AclMode {
    Dynamic,
    Static,
}
pub type AdditionalCapability = String;
pub type AvailabilityZone = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    pub id: String,
}
impl Disk {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPool {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    pub properties: DiskPoolProperties,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemMetadata>,
}
impl DiskPool {
    pub fn new(tracked_resource: TrackedResource, properties: DiskPoolProperties) -> Self {
        Self {
            tracked_resource,
            sku: None,
            properties,
            system_data: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolCreate {
    pub sku: Sku,
    pub properties: DiskPoolCreateProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl DiskPoolCreate {
    pub fn new(sku: Sku, properties: DiskPoolCreateProperties, location: String) -> Self {
        Self {
            sku,
            properties,
            tags: None,
            location,
            id: None,
            name: None,
            type_: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolCreateProperties {
    #[serde(rename = "availabilityZones", default, skip_serializing_if = "Vec::is_empty")]
    pub availability_zones: Vec<AvailabilityZone>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
}
impl DiskPoolCreateProperties {
    pub fn new(subnet_id: String) -> Self {
        Self {
            availability_zones: Vec::new(),
            disks: Vec::new(),
            subnet_id,
            additional_capabilities: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolListResult {
    pub value: Vec<DiskPool>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DiskPoolListResult {
    pub fn new(value: Vec<DiskPool>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolProperties {
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
    #[serde(rename = "availabilityZones")]
    pub availability_zones: Vec<AvailabilityZone>,
    pub status: OperationalStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
    #[serde(rename = "subnetId")]
    pub subnet_id: String,
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
}
impl DiskPoolProperties {
    pub fn new(
        provisioning_state: ProvisioningState,
        availability_zones: Vec<AvailabilityZone>,
        status: OperationalStatus,
        subnet_id: String,
    ) -> Self {
        Self {
            provisioning_state,
            availability_zones,
            status,
            disks: Vec::new(),
            subnet_id,
            additional_capabilities: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiskPoolTier {
    Basic,
    Standard,
    Premium,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskPoolUpdate {
    pub properties: DiskPoolUpdateProperties,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl DiskPoolUpdate {
    pub fn new(properties: DiskPoolUpdateProperties) -> Self {
        Self { properties, tags: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolUpdateProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub disks: Vec<Disk>,
}
impl DiskPoolUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolZoneInfo {
    #[serde(rename = "availabilityZones", default, skip_serializing_if = "Vec::is_empty")]
    pub availability_zones: Vec<AvailabilityZone>,
    #[serde(rename = "additionalCapabilities", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_capabilities: Vec<AdditionalCapability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl DiskPoolZoneInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskPoolZoneListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<DiskPoolZoneInfo>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl DiskPoolZoneListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDependency {
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "endpointDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_details: Vec<EndpointDetail>,
}
impl EndpointDependency {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointDetail {
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency: Option<f64>,
    #[serde(rename = "isAccessible", default, skip_serializing_if = "Option::is_none")]
    pub is_accessible: Option<bool>,
}
impl EndpointDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponse>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiLun {
    pub name: String,
    #[serde(rename = "managedDiskAzureResourceId")]
    pub managed_disk_azure_resource_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
}
impl IscsiLun {
    pub fn new(name: String, managed_disk_azure_resource_id: String) -> Self {
        Self {
            name,
            managed_disk_azure_resource_id,
            lun: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTarget {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: IscsiTargetProperties,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemMetadata>,
}
impl IscsiTarget {
    pub fn new(properties: IscsiTargetProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
            system_data: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCreate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: IscsiTargetCreateProperties,
}
impl IscsiTargetCreate {
    pub fn new(properties: IscsiTargetCreateProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetCreateProperties {
    #[serde(rename = "aclMode")]
    pub acl_mode: AclMode,
    #[serde(rename = "targetIqn", default, skip_serializing_if = "Option::is_none")]
    pub target_iqn: Option<String>,
    #[serde(rename = "staticAcls", default, skip_serializing_if = "Vec::is_empty")]
    pub static_acls: Vec<Acl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<IscsiLun>,
}
impl IscsiTargetCreateProperties {
    pub fn new(acl_mode: AclMode) -> Self {
        Self {
            acl_mode,
            target_iqn: None,
            static_acls: Vec::new(),
            luns: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetList {
    pub value: Vec<IscsiTarget>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl IscsiTargetList {
    pub fn new(value: Vec<IscsiTarget>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetProperties {
    #[serde(rename = "aclMode")]
    pub acl_mode: AclMode,
    #[serde(rename = "staticAcls", default, skip_serializing_if = "Vec::is_empty")]
    pub static_acls: Vec<Acl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<IscsiLun>,
    #[serde(rename = "targetIqn")]
    pub target_iqn: String,
    #[serde(rename = "provisioningState")]
    pub provisioning_state: ProvisioningState,
    pub status: OperationalStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl IscsiTargetProperties {
    pub fn new(acl_mode: AclMode, target_iqn: String, provisioning_state: ProvisioningState, status: OperationalStatus) -> Self {
        Self {
            acl_mode,
            static_acls: Vec::new(),
            luns: Vec::new(),
            target_iqn,
            provisioning_state,
            status,
            endpoints: Vec::new(),
            port: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IscsiTargetUpdate {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    pub properties: IscsiTargetUpdateProperties,
}
impl IscsiTargetUpdate {
    pub fn new(properties: IscsiTargetUpdateProperties) -> Self {
        Self {
            proxy_resource: ProxyResource::default(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IscsiTargetUpdateProperties {
    #[serde(rename = "staticAcls", default, skip_serializing_if = "Vec::is_empty")]
    pub static_acls: Vec<Acl>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<IscsiLun>,
}
impl IscsiTargetUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperationalStatus {
    Invalid,
    Unknown,
    Healthy,
    Unhealthy,
    Updating,
    Running,
    Stopped,
    #[serde(rename = "Stopped (deallocated)")]
    StoppedDeallocated,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OutboundEnvironmentEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<EndpointDependency>,
}
impl OutboundEnvironmentEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OutboundEnvironmentEndpointList {
    pub value: Vec<OutboundEnvironmentEndpoint>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OutboundEnvironmentEndpointList {
    pub fn new(value: Vec<OutboundEnvironmentEndpoint>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProvisioningState {
    Invalid,
    Succeeded,
    Failed,
    Canceled,
    Pending,
    Creating,
    Updating,
    Deleting,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self { name, tier: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolOperationDisplay {
    pub provider: String,
    pub resource: String,
    pub operation: String,
    pub description: String,
}
impl StoragePoolOperationDisplay {
    pub fn new(provider: String, resource: String, operation: String, description: String) -> Self {
        Self {
            provider,
            resource,
            operation,
            description,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolOperationListResult {
    pub value: Vec<StoragePoolRpOperation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl StoragePoolOperationListResult {
    pub fn new(value: Vec<StoragePoolRpOperation>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoragePoolRpOperation {
    pub name: String,
    #[serde(rename = "isDataAction")]
    pub is_data_action: bool,
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    pub display: StoragePoolOperationDisplay,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl StoragePoolRpOperation {
    pub fn new(name: String, is_data_action: bool, display: StoragePoolOperationDisplay) -> Self {
        Self {
            name,
            is_data_action,
            action_type: None,
            display,
            origin: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemMetadata {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_metadata::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_metadata::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_metadata {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    pub location: String,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            tags: None,
            location,
        }
    }
}
