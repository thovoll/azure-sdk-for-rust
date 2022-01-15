#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdmCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AdmCredentialProperties>,
}
impl AdmCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdmCredentialProperties {
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "authTokenUrl", default, skip_serializing_if = "Option::is_none")]
    pub auth_token_url: Option<String>,
}
impl AdmCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApnsCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApnsCredentialProperties>,
}
impl ApnsCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApnsCredentialProperties {
    #[serde(rename = "apnsCertificate", default, skip_serializing_if = "Option::is_none")]
    pub apns_certificate: Option<String>,
    #[serde(rename = "certificateKey", default, skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
impl ApnsCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaiduCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BaiduCredentialProperties>,
}
impl BaiduCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BaiduCredentialProperties {
    #[serde(rename = "baiduApiKey", default, skip_serializing_if = "Option::is_none")]
    pub baidu_api_key: Option<String>,
    #[serde(rename = "baiduEndPoint", default, skip_serializing_if = "Option::is_none")]
    pub baidu_end_point: Option<String>,
    #[serde(rename = "baiduSecretKey", default, skip_serializing_if = "Option::is_none")]
    pub baidu_secret_key: Option<String>,
}
impl BaiduCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAvailabilityParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
    #[serde(rename = "isAvailiable", default, skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
}
impl CheckAvailabilityParameters {
    pub fn new(name: String, location: String) -> Self {
        Self {
            id: None,
            name,
            type_: None,
            location,
            tags: None,
            sku: None,
            is_availiable: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAvailabilityResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(rename = "isAvailiable", default, skip_serializing_if = "Option::is_none")]
    pub is_availiable: Option<bool>,
}
impl CheckAvailabilityResult {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            is_availiable: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GcmCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<GcmCredentialProperties>,
}
impl GcmCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct GcmCredentialProperties {
    #[serde(rename = "gcmEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub gcm_endpoint: Option<String>,
    #[serde(rename = "googleApiKey", default, skip_serializing_if = "Option::is_none")]
    pub google_api_key: Option<String>,
}
impl GcmCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MpnsCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MpnsCredentialProperties>,
}
impl MpnsCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MpnsCredentialProperties {
    #[serde(rename = "mpnsCertificate", default, skip_serializing_if = "Option::is_none")]
    pub mpns_certificate: Option<String>,
    #[serde(rename = "certificateKey", default, skip_serializing_if = "Option::is_none")]
    pub certificate_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
}
impl MpnsCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceCreateOrUpdateParameters {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
}
impl NamespaceCreateOrUpdateParameters {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NamespaceResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl NamespaceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespacePatchParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl NamespacePatchParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NamespaceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "serviceBusEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub service_bus_endpoint: Option<String>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "scaleUnit", default, skip_serializing_if = "Option::is_none")]
    pub scale_unit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    #[serde(rename = "namespaceType", default, skip_serializing_if = "Option::is_none")]
    pub namespace_type: Option<namespace_properties::NamespaceType>,
}
impl NamespaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod namespace_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum NamespaceType {
        Messaging,
        NotificationHub,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NamespaceResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
}
impl NamespaceResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubCreateOrUpdateParameters {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: NotificationHubProperties,
}
impl NotificationHubCreateOrUpdateParameters {
    pub fn new(resource: Resource, properties: NotificationHubProperties) -> Self {
        Self { resource, properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationHubListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<NotificationHubResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl NotificationHubListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NotificationHubProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "registrationTtl", default, skip_serializing_if = "Option::is_none")]
    pub registration_ttl: Option<String>,
    #[serde(rename = "authorizationRules", default, skip_serializing_if = "Vec::is_empty")]
    pub authorization_rules: Vec<SharedAccessAuthorizationRuleProperties>,
    #[serde(rename = "apnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub apns_credential: Option<ApnsCredential>,
    #[serde(rename = "wnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub wns_credential: Option<WnsCredential>,
    #[serde(rename = "gcmCredential", default, skip_serializing_if = "Option::is_none")]
    pub gcm_credential: Option<GcmCredential>,
    #[serde(rename = "mpnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub mpns_credential: Option<MpnsCredential>,
    #[serde(rename = "admCredential", default, skip_serializing_if = "Option::is_none")]
    pub adm_credential: Option<AdmCredential>,
    #[serde(rename = "baiduCredential", default, skip_serializing_if = "Option::is_none")]
    pub baidu_credential: Option<BaiduCredential>,
}
impl NotificationHubProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationHubResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<NotificationHubProperties>,
}
impl NotificationHubResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PnsCredentialsProperties {
    #[serde(rename = "apnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub apns_credential: Option<ApnsCredential>,
    #[serde(rename = "wnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub wns_credential: Option<WnsCredential>,
    #[serde(rename = "gcmCredential", default, skip_serializing_if = "Option::is_none")]
    pub gcm_credential: Option<GcmCredential>,
    #[serde(rename = "mpnsCredential", default, skip_serializing_if = "Option::is_none")]
    pub mpns_credential: Option<MpnsCredential>,
    #[serde(rename = "admCredential", default, skip_serializing_if = "Option::is_none")]
    pub adm_credential: Option<AdmCredential>,
    #[serde(rename = "baiduCredential", default, skip_serializing_if = "Option::is_none")]
    pub baidu_credential: Option<BaiduCredential>,
}
impl PnsCredentialsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PnsCredentialsResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PnsCredentialsProperties>,
}
impl PnsCredentialsResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicykeyResource {
    #[serde(rename = "policyKey", default, skip_serializing_if = "Option::is_none")]
    pub policy_key: Option<String>,
}
impl PolicykeyResource {
    pub fn new() -> Self {
        Self::default()
    }
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<Sku>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
            sku: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceListKeys {
    #[serde(rename = "primaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub primary_connection_string: Option<String>,
    #[serde(rename = "secondaryConnectionString", default, skip_serializing_if = "Option::is_none")]
    pub secondary_connection_string: Option<String>,
    #[serde(rename = "primaryKey", default, skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
    #[serde(rename = "secondaryKey", default, skip_serializing_if = "Option::is_none")]
    pub secondary_key: Option<String>,
    #[serde(rename = "keyName", default, skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
}
impl ResourceListKeys {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleCreateOrUpdateParameters {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: SharedAccessAuthorizationRuleProperties,
}
impl SharedAccessAuthorizationRuleCreateOrUpdateParameters {
    pub fn new(resource: Resource, properties: SharedAccessAuthorizationRuleProperties) -> Self {
        Self { resource, properties }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedAccessAuthorizationRuleListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SharedAccessAuthorizationRuleResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SharedAccessAuthorizationRuleListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SharedAccessAuthorizationRuleProperties {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rights: Vec<String>,
}
impl SharedAccessAuthorizationRuleProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharedAccessAuthorizationRuleResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SharedAccessAuthorizationRuleProperties>,
}
impl SharedAccessAuthorizationRuleResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sku {
    pub name: sku::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i64>,
}
impl Sku {
    pub fn new(name: sku::Name) -> Self {
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
    pub enum Name {
        Free,
        Basic,
        Standard,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SubResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WnsCredential {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WnsCredentialProperties>,
}
impl WnsCredential {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WnsCredentialProperties {
    #[serde(rename = "packageSid", default, skip_serializing_if = "Option::is_none")]
    pub package_sid: Option<String>,
    #[serde(rename = "secretKey", default, skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
    #[serde(rename = "windowsLiveEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub windows_live_endpoint: Option<String>,
}
impl WnsCredentialProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
