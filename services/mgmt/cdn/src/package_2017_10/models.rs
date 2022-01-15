#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheExpirationActionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: cache_expiration_action_parameters::OdataType,
    #[serde(rename = "cacheBehavior")]
    pub cache_behavior: cache_expiration_action_parameters::CacheBehavior,
    #[serde(rename = "cacheType")]
    pub cache_type: cache_expiration_action_parameters::CacheType,
    #[serde(rename = "cacheDuration", default, skip_serializing_if = "Option::is_none")]
    pub cache_duration: Option<String>,
}
impl CacheExpirationActionParameters {
    pub fn new(
        odata_type: cache_expiration_action_parameters::OdataType,
        cache_behavior: cache_expiration_action_parameters::CacheBehavior,
        cache_type: cache_expiration_action_parameters::CacheType,
    ) -> Self {
        Self {
            odata_type,
            cache_behavior,
            cache_type,
            cache_duration: None,
        }
    }
}
pub mod cache_expiration_action_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "Microsoft.Azure.Cdn.Models.DeliveryRuleCacheExpirationActionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleCacheExpirationActionParameters,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CacheBehavior {
        BypassCache,
        Override,
        SetIfMissing,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CacheType {
        All,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnCertificateSourceParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: cdn_certificate_source_parameters::OdataType,
    #[serde(rename = "certificateType")]
    pub certificate_type: cdn_certificate_source_parameters::CertificateType,
}
impl CdnCertificateSourceParameters {
    pub fn new(
        odata_type: cdn_certificate_source_parameters::OdataType,
        certificate_type: cdn_certificate_source_parameters::CertificateType,
    ) -> Self {
        Self {
            odata_type,
            certificate_type,
        }
    }
}
pub mod cdn_certificate_source_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.CdnCertificateSourceParameters")]
        MicrosoftAzureCdnModelsCdnCertificateSourceParameters,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CertificateType {
        Shared,
        Dedicated,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CdnManagedHttpsParameters {
    #[serde(flatten)]
    pub custom_domain_https_parameters: CustomDomainHttpsParameters,
    #[serde(rename = "certificateSourceParameters")]
    pub certificate_source_parameters: CdnCertificateSourceParameters,
}
impl CdnManagedHttpsParameters {
    pub fn new(
        custom_domain_https_parameters: CustomDomainHttpsParameters,
        certificate_source_parameters: CdnCertificateSourceParameters,
    ) -> Self {
        Self {
            custom_domain_https_parameters,
            certificate_source_parameters,
        }
    }
}
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomDomain {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomDomainProperties>,
}
impl CustomDomain {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomDomainHttpsParameters {
    #[serde(rename = "certificateSource")]
    pub certificate_source: custom_domain_https_parameters::CertificateSource,
    #[serde(rename = "protocolType")]
    pub protocol_type: custom_domain_https_parameters::ProtocolType,
}
impl CustomDomainHttpsParameters {
    pub fn new(
        certificate_source: custom_domain_https_parameters::CertificateSource,
        protocol_type: custom_domain_https_parameters::ProtocolType,
    ) -> Self {
        Self {
            certificate_source,
            protocol_type,
        }
    }
}
pub mod custom_domain_https_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CertificateSource {
        AzureKeyVault,
        Cdn,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProtocolType {
        ServerNameIndication,
        #[serde(rename = "IPBased")]
        IpBased,
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
    #[serde(rename = "customHttpsProvisioningSubstate", default, skip_serializing_if = "Option::is_none")]
    pub custom_https_provisioning_substate: Option<custom_domain_properties::CustomHttpsProvisioningSubstate>,
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
            custom_https_provisioning_substate: None,
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
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CustomHttpsProvisioningSubstate {
        SubmittingDomainControlValidationRequest,
        PendingDomainControlValidationREquestApproval,
        DomainControlValidationRequestApproved,
        DomainControlValidationRequestRejected,
        DomainControlValidationRequestTimedOut,
        IssuingCertificate,
        DeployingCertificate,
        CertificateDeployed,
        DeletingCertificate,
        CertificateDeleted,
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
pub struct DeliveryRule {
    pub order: i64,
    pub actions: Vec<DeliveryRuleAction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<DeliveryRuleCondition>,
}
impl DeliveryRule {
    pub fn new(order: i64, actions: Vec<DeliveryRuleAction>) -> Self {
        Self {
            order,
            actions,
            conditions: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleAction {
    pub name: delivery_rule_action::Name,
}
impl DeliveryRuleAction {
    pub fn new(name: delivery_rule_action::Name) -> Self {
        Self { name }
    }
}
pub mod delivery_rule_action {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        CacheExpiration,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleCacheExpirationAction {
    #[serde(flatten)]
    pub delivery_rule_action: DeliveryRuleAction,
    pub parameters: CacheExpirationActionParameters,
}
impl DeliveryRuleCacheExpirationAction {
    pub fn new(delivery_rule_action: DeliveryRuleAction, parameters: CacheExpirationActionParameters) -> Self {
        Self {
            delivery_rule_action,
            parameters,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleCondition {
    pub name: delivery_rule_condition::Name,
}
impl DeliveryRuleCondition {
    pub fn new(name: delivery_rule_condition::Name) -> Self {
        Self { name }
    }
}
pub mod delivery_rule_condition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        UrlPath,
        UrlFileExtension,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleUrlFileExtensionCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    pub parameters: UrlFileExtensionConditionParameters,
}
impl DeliveryRuleUrlFileExtensionCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: UrlFileExtensionConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeliveryRuleUrlPathCondition {
    #[serde(flatten)]
    pub delivery_rule_condition: DeliveryRuleCondition,
    pub parameters: UrlPathConditionParameters,
}
impl DeliveryRuleUrlPathCondition {
    pub fn new(delivery_rule_condition: DeliveryRuleCondition, parameters: UrlPathConditionParameters) -> Self {
        Self {
            delivery_rule_condition,
            parameters,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EdgeNode {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EdgeNodeProperties>,
}
impl EdgeNode {
    pub fn new() -> Self {
        Self::default()
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
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<EndpointProperties>,
}
impl Endpoint {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
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
    #[serde(rename = "probePath", default, skip_serializing_if = "Option::is_none")]
    pub probe_path: Option<String>,
    #[serde(rename = "geoFilters", default, skip_serializing_if = "Vec::is_empty")]
    pub geo_filters: Vec<GeoFilter>,
    #[serde(rename = "deliveryPolicy", default, skip_serializing_if = "Option::is_none")]
    pub delivery_policy: Option<endpoint_properties_update_parameters::DeliveryPolicy>,
}
impl EndpointPropertiesUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod endpoint_properties_update_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct DeliveryPolicy {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        pub rules: Vec<DeliveryRule>,
    }
    impl DeliveryPolicy {
        pub fn new(rules: Vec<DeliveryRule>) -> Self {
            Self { description: None, rules }
        }
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
pub struct KeyVaultCertificateSourceParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: key_vault_certificate_source_parameters::OdataType,
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
    #[serde(rename = "resourceGroupName")]
    pub resource_group_name: String,
    #[serde(rename = "vaultName")]
    pub vault_name: String,
    #[serde(rename = "secretName")]
    pub secret_name: String,
    #[serde(rename = "secretVersion")]
    pub secret_version: String,
    #[serde(rename = "updateRule")]
    pub update_rule: key_vault_certificate_source_parameters::UpdateRule,
    #[serde(rename = "deleteRule")]
    pub delete_rule: key_vault_certificate_source_parameters::DeleteRule,
}
impl KeyVaultCertificateSourceParameters {
    pub fn new(
        odata_type: key_vault_certificate_source_parameters::OdataType,
        subscription_id: String,
        resource_group_name: String,
        vault_name: String,
        secret_name: String,
        secret_version: String,
        update_rule: key_vault_certificate_source_parameters::UpdateRule,
        delete_rule: key_vault_certificate_source_parameters::DeleteRule,
    ) -> Self {
        Self {
            odata_type,
            subscription_id,
            resource_group_name,
            vault_name,
            secret_name,
            secret_version,
            update_rule,
            delete_rule,
        }
    }
}
pub mod key_vault_certificate_source_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "#Microsoft.Azure.Cdn.Models.KeyVaultCertificateSourceParameters")]
        MicrosoftAzureCdnModelsKeyVaultCertificateSourceParameters,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum UpdateRule {
        NoAction,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DeleteRule {
        NoAction,
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
pub struct OperationsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationsListResult {
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
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<OriginProperties>,
}
impl Origin {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
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
    pub tracked_resource: TrackedResource,
    pub sku: Sku,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ProfileProperties>,
}
impl Profile {
    pub fn new(tracked_resource: TrackedResource, sku: Sku) -> Self {
        Self {
            tracked_resource,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProfileUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ProfileUpdateParameters {
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
        #[serde(rename = "Premium_ChinaCdn")]
        PremiumChinaCdn,
        #[serde(rename = "Standard_Microsoft")]
        StandardMicrosoft,
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
pub struct SupportedOptimizationTypesListResult {
    #[serde(rename = "supportedOptimizationTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub supported_optimization_types: Vec<OptimizationType>,
}
impl SupportedOptimizationTypesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TrackedResource {
    pub fn new(location: String) -> Self {
        Self {
            resource: Resource::default(),
            location,
            tags: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlFileExtensionConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: url_file_extension_condition_parameters::OdataType,
    pub extensions: Vec<String>,
}
impl UrlFileExtensionConditionParameters {
    pub fn new(odata_type: url_file_extension_condition_parameters::OdataType, extensions: Vec<String>) -> Self {
        Self { odata_type, extensions }
    }
}
pub mod url_file_extension_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "Microsoft.Azure.Cdn.Models.DeliveryRuleUrlFileExtensionConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleUrlFileExtensionConditionParameters,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlPathConditionParameters {
    #[serde(rename = "@odata.type")]
    pub odata_type: url_path_condition_parameters::OdataType,
    pub path: String,
    #[serde(rename = "matchType")]
    pub match_type: url_path_condition_parameters::MatchType,
}
impl UrlPathConditionParameters {
    pub fn new(
        odata_type: url_path_condition_parameters::OdataType,
        path: String,
        match_type: url_path_condition_parameters::MatchType,
    ) -> Self {
        Self {
            odata_type,
            path,
            match_type,
        }
    }
}
pub mod url_path_condition_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OdataType {
        #[serde(rename = "Microsoft.Azure.Cdn.Models.DeliveryRuleUrlPathConditionParameters")]
        MicrosoftAzureCdnModelsDeliveryRuleUrlPathConditionParameters,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MatchType {
        Literal,
        Wildcard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserManagedHttpsParameters {
    #[serde(flatten)]
    pub custom_domain_https_parameters: CustomDomainHttpsParameters,
    #[serde(rename = "certificateSourceParameters")]
    pub certificate_source_parameters: KeyVaultCertificateSourceParameters,
}
impl UserManagedHttpsParameters {
    pub fn new(
        custom_domain_https_parameters: CustomDomainHttpsParameters,
        certificate_source_parameters: KeyVaultCertificateSourceParameters,
    ) -> Self {
        Self {
            custom_domain_https_parameters,
            certificate_source_parameters,
        }
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateProbeInput {
    #[serde(rename = "probeURL")]
    pub probe_url: String,
}
impl ValidateProbeInput {
    pub fn new(probe_url: String) -> Self {
        Self { probe_url }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ValidateProbeOutput {
    #[serde(rename = "isValid", default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[serde(rename = "errorCode", default, skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ValidateProbeOutput {
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
