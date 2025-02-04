#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiConnectionDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<api_connection_definition::Properties>,
}
impl ApiConnectionDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod api_connection_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub statuses: Vec<ConnectionStatusDefinition>,
        #[serde(rename = "parameterValues", default, skip_serializing_if = "Option::is_none")]
        pub parameter_values: Option<serde_json::Value>,
        #[serde(rename = "customParameterValues", default, skip_serializing_if = "Option::is_none")]
        pub custom_parameter_values: Option<serde_json::Value>,
        #[serde(rename = "nonSecretParameterValues", default, skip_serializing_if = "Option::is_none")]
        pub non_secret_parameter_values: Option<serde_json::Value>,
        #[serde(rename = "createdTime", default, skip_serializing_if = "Option::is_none")]
        pub created_time: Option<String>,
        #[serde(rename = "changedTime", default, skip_serializing_if = "Option::is_none")]
        pub changed_time: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub api: Option<ApiReference>,
        #[serde(rename = "testLinks", default, skip_serializing_if = "Vec::is_empty")]
        pub test_links: Vec<ApiConnectionTestLink>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiConnectionDefinitionCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ApiConnectionDefinition>,
}
impl ApiConnectionDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiConnectionTestLink {
    #[serde(rename = "requestUri", default, skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}
impl ApiConnectionTestLink {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOAuthSettings {
    #[serde(rename = "identityProvider", default, skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<String>,
    #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "clientSecret", default, skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
    #[serde(rename = "redirectUrl", default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(rename = "customParameters", default, skip_serializing_if = "Option::is_none")]
    pub custom_parameters: Option<serde_json::Value>,
}
impl ApiOAuthSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiOAuthSettingsParameter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(rename = "uiDefinition", default, skip_serializing_if = "Option::is_none")]
    pub ui_definition: Option<serde_json::Value>,
}
impl ApiOAuthSettingsParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swagger: Option<serde_json::Value>,
    #[serde(rename = "brandColor", default, skip_serializing_if = "Option::is_none")]
    pub brand_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "iconUri", default, skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ApiReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceBackendService {
    #[serde(rename = "serviceUrl", default, skip_serializing_if = "Option::is_none")]
    pub service_url: Option<String>,
}
impl ApiResourceBackendService {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceDefinitions {
    #[serde(rename = "originalSwaggerUrl", default, skip_serializing_if = "Option::is_none")]
    pub original_swagger_url: Option<String>,
    #[serde(rename = "modifiedSwaggerUrl", default, skip_serializing_if = "Option::is_none")]
    pub modified_swagger_url: Option<String>,
}
impl ApiResourceDefinitions {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceGeneralInformation {
    #[serde(rename = "iconUrl", default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "termsOfUseUrl", default, skip_serializing_if = "Option::is_none")]
    pub terms_of_use_url: Option<String>,
    #[serde(rename = "releaseTag", default, skip_serializing_if = "Option::is_none")]
    pub release_tag: Option<String>,
}
impl ApiResourceGeneralInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "brandColor", default, skip_serializing_if = "Option::is_none")]
    pub brand_color: Option<String>,
    #[serde(rename = "hideKey", default, skip_serializing_if = "Option::is_none")]
    pub hide_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsDictionary>,
    #[serde(rename = "apiType", default, skip_serializing_if = "Option::is_none")]
    pub api_type: Option<ApiType>,
    #[serde(rename = "wsdlService", default, skip_serializing_if = "Option::is_none")]
    pub wsdl_service: Option<WsdlService>,
    #[serde(rename = "wsdlImportMethod", default, skip_serializing_if = "Option::is_none")]
    pub wsdl_import_method: Option<WsdlImportMethod>,
    #[serde(rename = "connectionType", default, skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
}
impl ApiResourceMetadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourcePolicies {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "contentLink", default, skip_serializing_if = "Option::is_none")]
    pub content_link: Option<String>,
}
impl ApiResourcePolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApiResourceProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "connectionParameters", default, skip_serializing_if = "Option::is_none")]
    pub connection_parameters: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ApiResourceMetadata>,
    #[serde(rename = "runtimeUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub runtime_urls: Vec<String>,
    #[serde(rename = "generalInformation", default, skip_serializing_if = "Option::is_none")]
    pub general_information: Option<ApiResourceGeneralInformation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swagger: Option<serde_json::Value>,
    #[serde(rename = "backendService", default, skip_serializing_if = "Option::is_none")]
    pub backend_service: Option<ApiResourceBackendService>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<ApiResourcePolicies>,
    #[serde(rename = "apiDefinitionUrl", default, skip_serializing_if = "Option::is_none")]
    pub api_definition_url: Option<String>,
    #[serde(rename = "apiDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub api_definitions: Option<ApiResourceDefinitions>,
}
impl ApiResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiType {
    NotSpecified,
    Rest,
    Soap,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfirmConsentCodeDefinition {
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
impl ConfirmConsentCodeDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionError {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<connection_error::Properties>,
}
impl ConnectionError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_error {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionGatewayDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<connection_gateway_definition::Properties>,
}
impl ConnectionGatewayDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_gateway_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "connectionGatewayInstallation", default, skip_serializing_if = "Option::is_none")]
        pub connection_gateway_installation: Option<ConnectionGatewayReference>,
        #[serde(rename = "contactInformation", default, skip_serializing_if = "Vec::is_empty")]
        pub contact_information: Vec<String>,
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
        pub machine_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<serde_json::Value>,
        #[serde(rename = "backendUri", default, skip_serializing_if = "Option::is_none")]
        pub backend_uri: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionGatewayDefinitionCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectionGatewayDefinition>,
}
impl ConnectionGatewayDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionGatewayInstallationDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<connection_gateway_installation_definition::Properties>,
}
impl ConnectionGatewayInstallationDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_gateway_installation_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "connectionGateway", default, skip_serializing_if = "Option::is_none")]
        pub connection_gateway: Option<ConnectionGatewayReference>,
        #[serde(rename = "contactInformation", default, skip_serializing_if = "Vec::is_empty")]
        pub contact_information: Vec<String>,
        #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
        pub display_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(rename = "machineName", default, skip_serializing_if = "Option::is_none")]
        pub machine_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<serde_json::Value>,
        #[serde(rename = "backendUri", default, skip_serializing_if = "Option::is_none")]
        pub backend_uri: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionGatewayInstallationDefinitionCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConnectionGatewayInstallationDefinition>,
}
impl ConnectionGatewayInstallationDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionGatewayReference {
    #[serde(flatten)]
    pub resource_reference: ResourceReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ConnectionGatewayReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionParameter {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<connection_parameter::Type>,
    #[serde(rename = "oAuthSettings", default, skip_serializing_if = "Option::is_none")]
    pub o_auth_settings: Option<ApiOAuthSettings>,
}
impl ConnectionParameter {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod connection_parameter {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "string")]
        String,
        #[serde(rename = "securestring")]
        Securestring,
        #[serde(rename = "secureobject")]
        Secureobject,
        #[serde(rename = "int")]
        Int,
        #[serde(rename = "bool")]
        Bool,
        #[serde(rename = "object")]
        Object,
        #[serde(rename = "array")]
        Array,
        #[serde(rename = "oauthSetting")]
        OauthSetting,
        #[serde(rename = "connection")]
        Connection,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConnectionStatusDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ConnectionError>,
}
impl ConnectionStatusDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsentLinkCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConsentLinkDefinition>,
}
impl ConsentLinkCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsentLinkDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "firstPartyLoginUri", default, skip_serializing_if = "Option::is_none")]
    pub first_party_login_uri: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<consent_link_definition::Status>,
}
impl ConsentLinkDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod consent_link_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Unauthenticated,
        Authenticated,
        Error,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConsentLinkParameterDefinition {
    #[serde(rename = "parameterName", default, skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(rename = "redirectUrl", default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ConsentLinkParameterDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomApiDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CustomApiPropertiesDefinition>,
}
impl CustomApiDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomApiDefinitionCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CustomApiDefinition>,
}
impl CustomApiDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomApiPropertiesDefinition {
    #[serde(rename = "connectionParameters", default, skip_serializing_if = "Option::is_none")]
    pub connection_parameters: Option<serde_json::Value>,
    #[serde(rename = "runtimeUrls", default, skip_serializing_if = "Vec::is_empty")]
    pub runtime_urls: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swagger: Option<serde_json::Value>,
    #[serde(rename = "brandColor", default, skip_serializing_if = "Option::is_none")]
    pub brand_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "iconUri", default, skip_serializing_if = "Option::is_none")]
    pub icon_uri: Option<String>,
    #[serde(rename = "backendService", default, skip_serializing_if = "Option::is_none")]
    pub backend_service: Option<ApiResourceBackendService>,
    #[serde(rename = "apiDefinitions", default, skip_serializing_if = "Option::is_none")]
    pub api_definitions: Option<ApiResourceDefinitions>,
    #[serde(rename = "apiType", default, skip_serializing_if = "Option::is_none")]
    pub api_type: Option<ApiType>,
    #[serde(rename = "wsdlDefinition", default, skip_serializing_if = "Option::is_none")]
    pub wsdl_definition: Option<WsdlDefinition>,
}
impl CustomApiPropertiesDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CustomApiReference {
    #[serde(flatten)]
    pub api_reference: ApiReference,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl CustomApiReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListConnectionKeysDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<list_connection_keys_definition::Properties>,
}
impl ListConnectionKeysDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod list_connection_keys_definition {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Properties {
        #[serde(rename = "validityTimeSpan", default, skip_serializing_if = "Option::is_none")]
        pub validity_time_span: Option<String>,
    }
    impl Properties {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListConsentLinksDefinition {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ConsentLinkParameterDefinition>,
}
impl ListConsentLinksDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedApiDefinition {
    #[serde(flatten)]
    pub resource_definition: ResourceDefinition,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApiResourceProperties>,
}
impl ManagedApiDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ManagedApiDefinitionCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ManagedApiDefinition>,
}
impl ManagedApiDefinitionCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagsDictionary>,
}
impl ResourceDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsDictionary {}
impl TagsDictionary {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WsdlDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<WsdlService>,
    #[serde(rename = "importMethod", default, skip_serializing_if = "Option::is_none")]
    pub import_method: Option<WsdlImportMethod>,
}
impl WsdlDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WsdlImportMethod {
    NotSpecified,
    SoapToRest,
    SoapPassThrough,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WsdlService {
    #[serde(rename = "qualifiedName")]
    pub qualified_name: String,
    #[serde(rename = "endpointQualifiedNames", default, skip_serializing_if = "Vec::is_empty")]
    pub endpoint_qualified_names: Vec<String>,
}
impl WsdlService {
    pub fn new(qualified_name: String) -> Self {
        Self {
            qualified_name,
            endpoint_qualified_names: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WsdlServiceCollection {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<WsdlService>,
}
impl WsdlServiceCollection {
    pub fn new() -> Self {
        Self::default()
    }
}
