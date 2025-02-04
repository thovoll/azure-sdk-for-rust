#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityInput {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: ResourceType,
}
impl CheckNameAvailabilityInput {
    pub fn new(name: String, type_: ResourceType) -> Self {
        Self { name, type_ }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityOutput {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomain {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomDomainProperties>,
}
impl CustomDomain {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomDomain>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl CustomDomainListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomainParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomDomainPropertiesParameters>,
}
impl CustomDomainParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainProperties {
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<custom_domain_properties::ResourceState>,
    #[serde(rename = "customHttpsProvisioningState", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_provisioning_state: Option<custom_domain_properties::CustomHttpsProvisioningState>,
    #[serde(rename = "validationData", default, skip_serializing_if = "Option::is_none")]
    pub validation_data: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl CustomDomainProperties {
    pub fn new(host_name: String) -> Self {
        Self {
            host_name,
            resource_state: None,
            custom_https_provisioning_state: None,
            validation_data: None,
            provisioning_state: None,
        }
    }
}
pub mod custom_domain_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomHttpsProvisioningState {
        Enabling,
        Enabled,
        Disabling,
        Disabled,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainPropertiesParameters {
    #[serde(rename = "hostName")]
    pub host_name: String,
}
impl CustomDomainPropertiesParameters {
    pub fn new(host_name: String) -> Self {
        Self { host_name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedOrigin {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DeepCreatedOriginProperties>,
}
impl DeepCreatedOrigin {
    pub fn new(name: String) -> Self {
        Self { name, properties: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeepCreatedOriginProperties {
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
}
impl DeepCreatedOriginProperties {
    pub fn new(host_name: String) -> Self {
        Self {
            host_name,
            http_port: None,
            https_port: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeNode {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EdgeNodeProperties>,
}
impl EdgeNode {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeNodeProperties {
    #[serde(rename = "ipAddressGroups")]
    pub ip_address_groups: Vec<IpAddressGroup>,
}
impl EdgeNodeProperties {
    pub fn new(ip_address_groups: Vec<IpAddressGroup>) -> Self {
        Self { ip_address_groups }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgenodeResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<EdgeNode>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl EdgenodeResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Endpoint {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointProperties>,
}
impl Endpoint {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Endpoint>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl EndpointListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointProperties {
    #[serde(flatten)]
    pub endpoint_properties_update_parameters: EndpointPropertiesUpdateParameters,
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    pub origins: Vec<DeepCreatedOrigin>,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<endpoint_properties::ResourceState>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl EndpointProperties {
    pub fn new(origins: Vec<DeepCreatedOrigin>) -> Self {
        Self {
            endpoint_properties_update_parameters: EndpointPropertiesUpdateParameters::default(),
            host_name: None,
            origins,
            resource_state: None,
            provisioning_state: None,
        }
    }
}
pub mod endpoint_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Deleting,
        Running,
        Starting,
        Stopped,
        Stopping,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointPropertiesUpdateParameters {
    #[serde(rename = "originHostHeader", default, skip_serializing_if = "Option::is_none")]
    pub origin_host_header: Option<String>,
    #[serde(rename = "originPath", default, skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
    #[serde(rename = "contentTypesToCompress", default, skip_serializing_if = "Vec::is_empty")]
    pub content_types_to_compress: Vec<String>,
    #[serde(rename = "isCompressionEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_compression_enabled: Option<bool>,
    #[serde(rename = "isHttpAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_http_allowed: Option<bool>,
    #[serde(rename = "isHttpsAllowed", default, skip_serializing_if = "Option::is_none")]
    pub is_https_allowed: Option<bool>,
    #[serde(rename = "queryStringCachingBehavior", default, skip_serializing_if = "Option::is_none")]
    pub query_string_caching_behavior: Option<QueryStringCachingBehavior>,
    #[serde(rename = "optimizationType", default, skip_serializing_if = "Option::is_none")]
    pub optimization_type: Option<OptimizationType>,
    #[serde(rename = "geoFilters", default, skip_serializing_if = "Vec::is_empty")]
    pub geo_filters: Vec<GeoFilter>,
}
impl EndpointPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EndpointUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointPropertiesUpdateParameters>,
}
impl EndpointUpdateParameters {
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
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoFilter {
    #[serde(rename = "relativePath")]
    pub relative_path: String,
    pub action: geo_filter::Action,
    #[serde(rename = "countryCodes")]
    pub country_codes: Vec<String>,
}
impl GeoFilter {
    pub fn new(relative_path: String, action: geo_filter::Action, country_codes: Vec<String>) -> Self {
        Self {
            relative_path,
            action,
            country_codes,
        }
    }
}
pub mod geo_filter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Action {
        Block,
        Allow,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct IpAddressGroup {
    #[serde(rename = "deliveryRegion", default, skip_serializing_if = "Option::is_none")]
    pub delivery_region: Option<String>,
    #[serde(rename = "ipv4Addresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ipv4_addresses: Vec<CidrIpAddress>,
    #[serde(rename = "ipv6Addresses", default, skip_serializing_if = "Vec::is_empty")]
    pub ipv6_addresses: Vec<CidrIpAddress>,
}
impl IpAddressGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LoadParameters {
    #[serde(rename = "contentPaths")]
    pub content_paths: Vec<String>,
}
impl LoadParameters {
    pub fn new(content_paths: Vec<String>) -> Self {
        Self { content_paths }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OptimizationType {
    GeneralWebDelivery,
    GeneralMediaStreaming,
    VideoOnDemandMediaStreaming,
    LargeFileDownload,
    DynamicSiteAcceleration,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Origin {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OriginProperties>,
}
impl Origin {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Origin>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OriginListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OriginProperties {
    #[serde(rename = "hostName")]
    pub host_name: String,
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<origin_properties::ResourceState>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl OriginProperties {
    pub fn new(host_name: String) -> Self {
        Self {
            host_name,
            http_port: None,
            https_port: None,
            resource_state: None,
            provisioning_state: None,
        }
    }
}
pub mod origin_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginPropertiesParameters {
    #[serde(rename = "hostName", default, skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "httpPort", default, skip_serializing_if = "Option::is_none")]
    pub http_port: Option<i64>,
    #[serde(rename = "httpsPort", default, skip_serializing_if = "Option::is_none")]
    pub https_port: Option<i64>,
}
impl OriginPropertiesParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OriginUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OriginPropertiesParameters>,
}
impl OriginUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    #[serde(flatten)]
    pub resource: Resource,
    pub sku: Sku,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileProperties>,
}
impl Profile {
    pub fn new(resource: Resource, sku: Sku) -> Self {
        Self {
            resource,
            sku,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Profile>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ProfileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileProperties {
    #[serde(rename = "resourceState", default, skip_serializing_if = "Option::is_none")]
    pub resource_state: Option<profile_properties::ResourceState>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ProfileProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod profile_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceState {
        Creating,
        Active,
        Deleting,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileUpdateParameters {
    pub tags: serde_json::Value,
}
impl ProfileUpdateParameters {
    pub fn new(tags: serde_json::Value) -> Self {
        Self { tags }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PurgeParameters {
    #[serde(rename = "contentPaths")]
    pub content_paths: Vec<String>,
}
impl PurgeParameters {
    pub fn new(content_paths: Vec<String>) -> Self {
        Self { content_paths }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum QueryStringCachingBehavior {
    IgnoreQueryString,
    BypassCaching,
    UseQueryString,
    NotSet,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ResourceType {
    #[serde(rename = "Microsoft.Cdn/Profiles/Endpoints")]
    MicrosoftCdnProfilesEndpoints,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUsage {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}
impl ResourceUsage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceUsageListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ResourceUsage>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ResourceUsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Sku {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<sku::Name>,
}
impl Sku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_Verizon")]
        StandardVerizon,
        #[serde(rename = "Premium_Verizon")]
        PremiumVerizon,
        #[serde(rename = "Custom_Verizon")]
        CustomVerizon,
        #[serde(rename = "Standard_Akamai")]
        StandardAkamai,
        #[serde(rename = "Standard_ChinaCdn")]
        StandardChinaCdn,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SsoUri {
    #[serde(rename = "ssoUriValue", default, skip_serializing_if = "Option::is_none")]
    pub sso_uri_value: Option<String>,
}
impl SsoUri {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SupportedOptimizationTypesResult {
    #[serde(rename = "supportedOptimizationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_optimization_types: Vec<OptimizationType>,
}
impl SupportedOptimizationTypesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateCustomDomainInput {
    #[serde(rename = "hostName")]
    pub host_name: String,
}
impl ValidateCustomDomainInput {
    pub fn new(host_name: String) -> Self {
        Self { host_name }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateCustomDomainOutput {
    #[serde(rename = "customDomainValidated", default, skip_serializing_if = "Option::is_none")]
    pub custom_domain_validated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ValidateCustomDomainOutput {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CidrIpAddress {
    #[serde(rename = "baseIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub base_ip_address: Option<String>,
    #[serde(rename = "prefixLength", default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i64>,
}
impl CidrIpAddress {
    pub fn new() -> Self {
        Self::default()
    }
}
