#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Account {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<Identity>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AccountProperties>,
}
impl Account {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Account>,
}
impl AccountListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<account_properties::ProvisioningState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "internalId", default, skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<SkuCapability>,
    #[serde(rename = "isMigrated", default, skip_serializing_if = "Option::is_none")]
    pub is_migrated: Option<bool>,
    #[serde(rename = "migrationToken", default, skip_serializing_if = "Option::is_none")]
    pub migration_token: Option<String>,
    #[serde(rename = "skuChangeInfo", default, skip_serializing_if = "Option::is_none")]
    pub sku_change_info: Option<SkuChangeInfo>,
    #[serde(rename = "customSubDomainName", default, skip_serializing_if = "Option::is_none")]
    pub custom_sub_domain_name: Option<String>,
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<NetworkRuleSet>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "userOwnedStorage", default, skip_serializing_if = "Vec::is_empty")]
    pub user_owned_storage: Vec<UserOwnedStorage>,
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<account_properties::PublicNetworkAccess>,
    #[serde(rename = "apiProperties", default, skip_serializing_if = "Option::is_none")]
    pub api_properties: Option<ApiProperties>,
    #[serde(rename = "dateCreated", default, skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "callRateLimit", default, skip_serializing_if = "Option::is_none")]
    pub call_rate_limit: Option<CallRateLimit>,
    #[serde(rename = "quotaLimit", default, skip_serializing_if = "Option::is_none")]
    pub quota_limit: Option<QuotaLimit>,
    #[serde(rename = "restrictOutboundNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub restrict_outbound_network_access: Option<bool>,
    #[serde(rename = "allowedFqdnList", default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_fqdn_list: Vec<String>,
    #[serde(rename = "disableLocalAuth", default, skip_serializing_if = "Option::is_none")]
    pub disable_local_auth: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore: Option<bool>,
}
impl AccountProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod account_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Accepted,
        Creating,
        Deleting,
        Moving,
        Failed,
        Succeeded,
        #[serde(rename = "ResolvingDNS")]
        ResolvingDns,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountSku {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl AccountSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountSkuListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AccountSku>,
}
impl AccountSkuListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiKeys {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key2: Option<String>,
}
impl ApiKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiProperties {
    #[serde(rename = "qnaRuntimeEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub qna_runtime_endpoint: Option<String>,
    #[serde(rename = "qnaAzureSearchEndpointKey", default, skip_serializing_if = "Option::is_none")]
    pub qna_azure_search_endpoint_key: Option<String>,
    #[serde(rename = "qnaAzureSearchEndpointId", default, skip_serializing_if = "Option::is_none")]
    pub qna_azure_search_endpoint_id: Option<String>,
    #[serde(rename = "statisticsEnabled", default, skip_serializing_if = "Option::is_none")]
    pub statistics_enabled: Option<bool>,
    #[serde(rename = "eventHubConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_connection_string: Option<String>,
    #[serde(rename = "storageAccountConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_connection_string: Option<String>,
    #[serde(rename = "aadClientId", default, skip_serializing_if = "Option::is_none")]
    pub aad_client_id: Option<String>,
    #[serde(rename = "aadTenantId", default, skip_serializing_if = "Option::is_none")]
    pub aad_tenant_id: Option<String>,
    #[serde(rename = "superUser", default, skip_serializing_if = "Option::is_none")]
    pub super_user: Option<String>,
    #[serde(rename = "websiteName", default, skip_serializing_if = "Option::is_none")]
    pub website_name: Option<String>,
}
impl ApiProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureEntityResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl AzureEntityResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CallRateLimit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    #[serde(rename = "renewalPeriod", default, skip_serializing_if = "Option::is_none")]
    pub renewal_period: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<ThrottlingRule>,
}
impl CallRateLimit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckDomainAvailabilityParameter {
    #[serde(rename = "subdomainName")]
    pub subdomain_name: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl CheckDomainAvailabilityParameter {
    pub fn new(subdomain_name: String, type_: String) -> Self {
        Self { subdomain_name, type_ }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckSkuAvailabilityParameter {
    pub skus: Vec<SkuName>,
    pub kind: Kind,
    #[serde(rename = "type")]
    pub type_: String,
}
impl CheckSkuAvailabilityParameter {
    pub fn new(skus: Vec<SkuName>, kind: Kind, type_: String) -> Self {
        Self { skus, kind, type_ }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DomainAvailability {
    #[serde(rename = "isSubdomainAvailable", default, skip_serializing_if = "Option::is_none")]
    pub is_subdomain_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "subdomainName", default, skip_serializing_if = "Option::is_none")]
    pub subdomain_name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl DomainAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Encryption {
    #[serde(rename = "keyVaultProperties", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_properties: Option<KeyVaultProperties>,
    #[serde(rename = "keySource", default, skip_serializing_if = "Option::is_none")]
    pub key_source: Option<encryption::KeySource>,
}
impl Encryption {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod encryption {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeySource {
        #[serde(rename = "Microsoft.CognitiveServices")]
        MicrosoftCognitiveServices,
        #[serde(rename = "Microsoft.KeyVault")]
        MicrosoftKeyVault,
    }
    impl Default for KeySource {
        fn default() -> Self {
            Self::MicrosoftKeyVault
        }
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
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<identity::Type>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "userAssignedIdentities", default, skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        None,
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpRule {
    pub value: String,
}
impl IpRule {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultProperties {
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "keyVersion", default, skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
    #[serde(rename = "keyVaultUri", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_uri: Option<String>,
    #[serde(rename = "identityClientId", default, skip_serializing_if = "Option::is_none")]
    pub identity_client_id: Option<String>,
}
impl KeyVaultProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type Kind = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetricName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl MetricName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSet {
    #[serde(rename = "defaultAction", default, skip_serializing_if = "Option::is_none")]
    pub default_action: Option<network_rule_set::DefaultAction>,
    #[serde(rename = "ipRules", default, skip_serializing_if = "Vec::is_empty")]
    pub ip_rules: Vec<IpRule>,
    #[serde(rename = "virtualNetworkRules", default, skip_serializing_if = "Vec::is_empty")]
    pub virtual_network_rules: Vec<VirtualNetworkRule>,
}
impl NetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_rule_set {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DefaultAction {
        Allow,
        Deny,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDataAction", default, skip_serializing_if = "Option::is_none")]
    pub is_data_action: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[serde(rename = "actionType", default, skip_serializing_if = "Option::is_none")]
    pub action_type: Option<operation::ActionType>,
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
        #[serde(rename = "user,system")]
        UserSystem,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ActionType {
        Internal,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpoint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub azure_entity_resource: AzureEntityResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}
impl PrivateEndpointConnection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateEndpointConnection>,
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpoint>,
    #[serde(rename = "privateLinkServiceConnectionState")]
    pub private_link_service_connection_state: PrivateLinkServiceConnectionState,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<PrivateEndpointConnectionProvisioningState>,
    #[serde(rename = "groupIds", default, skip_serializing_if = "Vec::is_empty")]
    pub group_ids: Vec<String>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new(private_link_service_connection_state: PrivateLinkServiceConnectionState) -> Self {
        Self {
            private_endpoint: None,
            private_link_service_connection_state,
            provisioning_state: None,
            group_ids: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointConnectionProvisioningState {
    Succeeded,
    Creating,
    Deleting,
    Failed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PrivateEndpointServiceConnectionStatus {
    Pending,
    Approved,
    Rejected,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateLinkResourceProperties>,
}
impl PrivateLinkResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PrivateLinkResource>,
}
impl PrivateLinkResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResourceProperties {
    #[serde(rename = "groupId", default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "requiredMembers", default, skip_serializing_if = "Vec::is_empty")]
    pub required_members: Vec<String>,
    #[serde(rename = "requiredZoneNames", default, skip_serializing_if = "Vec::is_empty")]
    pub required_zone_names: Vec<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkServiceConnectionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PrivateEndpointServiceConnectionStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct QuotaLimit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    #[serde(rename = "renewalPeriod", default, skip_serializing_if = "Option::is_none")]
    pub renewal_period: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<ThrottlingRule>,
}
impl QuotaLimit {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegenerateKeyParameters {
    #[serde(rename = "keyName")]
    pub key_name: regenerate_key_parameters::KeyName,
}
impl RegenerateKeyParameters {
    pub fn new(key_name: regenerate_key_parameters::KeyName) -> Self {
        Self { key_name }
    }
}
pub mod regenerate_key_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum KeyName {
        Key1,
        Key2,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestMatchPattern {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}
impl RequestMatchPattern {
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
pub struct ResourceSku {
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<ResourceSkuRestrictions>,
}
impl ResourceSku {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceSkuListResult {
    pub value: Vec<ResourceSku>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ResourceSkuListResult {
    pub fn new(value: Vec<ResourceSku>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictionInfo {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
}
impl ResourceSkuRestrictionInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceSkuRestrictions {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_sku_restrictions::Type>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[serde(rename = "restrictionInfo", default, skip_serializing_if = "Option::is_none")]
    pub restriction_info: Option<ResourceSkuRestrictionInfo>,
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<resource_sku_restrictions::ReasonCode>,
}
impl ResourceSkuRestrictions {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_sku_restrictions {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Location,
        Zone,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReasonCode {
        QuotaId,
        NotAvailableForSubscription,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<sku::Tier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
}
impl Sku {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tier: None,
            size: None,
            family: None,
            capacity: None,
        }
    }
}
pub mod sku {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Free,
        Basic,
        Standard,
        Premium,
        Enterprise,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuAvailability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<SkuName>,
    #[serde(rename = "skuAvailable", default, skip_serializing_if = "Option::is_none")]
    pub sku_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl SkuAvailability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuAvailabilityListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SkuAvailability>,
}
impl SkuAvailabilityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuCapability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuCapability {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuChangeInfo {
    #[serde(rename = "countOfDowngrades", default, skip_serializing_if = "Option::is_none")]
    pub count_of_downgrades: Option<f64>,
    #[serde(rename = "countOfUpgradesAfterDowngrades", default, skip_serializing_if = "Option::is_none")]
    pub count_of_upgrades_after_downgrades: Option<f64>,
    #[serde(rename = "lastChangeDate", default, skip_serializing_if = "Option::is_none")]
    pub last_change_date: Option<String>,
}
impl SkuChangeInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type SkuName = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ThrottlingRule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "renewalPeriod", default, skip_serializing_if = "Option::is_none")]
    pub renewal_period: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<f64>,
    #[serde(rename = "minCount", default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<f64>,
    #[serde(rename = "dynamicThrottlingEnabled", default, skip_serializing_if = "Option::is_none")]
    pub dynamic_throttling_enabled: Option<bool>,
    #[serde(rename = "matchPatterns", default, skip_serializing_if = "Vec::is_empty")]
    pub match_patterns: Vec<RequestMatchPattern>,
}
impl ThrottlingRule {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum UnitType {
    Count,
    Bytes,
    Seconds,
    Percent,
    CountPerSecond,
    BytesPerSecond,
    Milliseconds,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<UnitType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<MetricName>,
    #[serde(rename = "quotaPeriod", default, skip_serializing_if = "Option::is_none")]
    pub quota_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<f64>,
    #[serde(rename = "nextResetTime", default, skip_serializing_if = "Option::is_none")]
    pub next_reset_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<usage::Status>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Included,
        Blocked,
        InOverage,
        Unknown,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
impl UsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserAssignedIdentity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}
impl UserAssignedIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UserOwnedStorage {
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "identityClientId", default, skip_serializing_if = "Option::is_none")]
    pub identity_client_id: Option<String>,
}
impl UserOwnedStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkRule {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "ignoreMissingVnetServiceEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_vnet_service_endpoint: Option<bool>,
}
impl VirtualNetworkRule {
    pub fn new(id: String) -> Self {
        Self {
            id,
            state: None,
            ignore_missing_vnet_service_endpoint: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
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
