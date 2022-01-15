#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<certificate::Properties>,
}
impl Certificate {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
pub mod certificate {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
        pub friendly_name: Option<String>,
        #[serde(rename = "subjectName", default, skip_serializing_if = "Option::is_none")]
        pub subject_name: Option<String>,
        #[serde(rename = "hostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub host_names: Vec<String>,
        #[serde(rename = "pfxBlob", default, skip_serializing_if = "Option::is_none")]
        pub pfx_blob: Option<String>,
        #[serde(rename = "siteName", default, skip_serializing_if = "Option::is_none")]
        pub site_name: Option<String>,
        #[serde(rename = "selfLink", default, skip_serializing_if = "Option::is_none")]
        pub self_link: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub issuer: Option<String>,
        #[serde(rename = "issueDate", default, skip_serializing_if = "Option::is_none")]
        pub issue_date: Option<String>,
        #[serde(rename = "expirationDate", default, skip_serializing_if = "Option::is_none")]
        pub expiration_date: Option<String>,
        pub password: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub thumbprint: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub valid: Option<bool>,
        #[serde(rename = "cerBlob", default, skip_serializing_if = "Option::is_none")]
        pub cer_blob: Option<String>,
        #[serde(rename = "publicKeyHash", default, skip_serializing_if = "Option::is_none")]
        pub public_key_hash: Option<String>,
        #[serde(rename = "hostingEnvironmentProfile", default, skip_serializing_if = "Option::is_none")]
        pub hosting_environment_profile: Option<HostingEnvironmentProfile>,
        #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_id: Option<String>,
        #[serde(rename = "keyVaultSecretName", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_secret_name: Option<String>,
        #[serde(rename = "keyVaultSecretStatus", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_secret_status: Option<properties::KeyVaultSecretStatus>,
        #[serde(rename = "serverFarmId", default, skip_serializing_if = "Option::is_none")]
        pub server_farm_id: Option<String>,
    }
    impl Properties {
        pub fn new(password: String) -> Self {
            Self {
                friendly_name: None,
                subject_name: None,
                host_names: Vec::new(),
                pfx_blob: None,
                site_name: None,
                self_link: None,
                issuer: None,
                issue_date: None,
                expiration_date: None,
                password,
                thumbprint: None,
                valid: None,
                cer_blob: None,
                public_key_hash: None,
                hosting_environment_profile: None,
                key_vault_id: None,
                key_vault_secret_name: None,
                key_vault_secret_status: None,
                server_farm_id: None,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum KeyVaultSecretStatus {
            Initialized,
            WaitingOnCertificateOrder,
            Succeeded,
            CertificateOrderFailed,
            OperationNotPermittedOnKeyVault,
            AzureServiceUnauthorizedToAccessKeyVault,
            KeyVaultDoesNotExist,
            KeyVaultSecretDoesNotExist,
            UnknownError,
            ExternalPrivateKey,
            Unknown,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateCollection {
    pub value: Vec<Certificate>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl CertificateCollection {
    pub fn new(value: Vec<Certificate>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificatePatchResource {
    #[serde(flatten)]
    pub proxy_only_resource: ProxyOnlyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<certificate_patch_resource::Properties>,
}
impl CertificatePatchResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod certificate_patch_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
        pub friendly_name: Option<String>,
        #[serde(rename = "subjectName", default, skip_serializing_if = "Option::is_none")]
        pub subject_name: Option<String>,
        #[serde(rename = "hostNames", default, skip_serializing_if = "Vec::is_empty")]
        pub host_names: Vec<String>,
        #[serde(rename = "pfxBlob", default, skip_serializing_if = "Option::is_none")]
        pub pfx_blob: Option<String>,
        #[serde(rename = "siteName", default, skip_serializing_if = "Option::is_none")]
        pub site_name: Option<String>,
        #[serde(rename = "selfLink", default, skip_serializing_if = "Option::is_none")]
        pub self_link: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub issuer: Option<String>,
        #[serde(rename = "issueDate", default, skip_serializing_if = "Option::is_none")]
        pub issue_date: Option<String>,
        #[serde(rename = "expirationDate", default, skip_serializing_if = "Option::is_none")]
        pub expiration_date: Option<String>,
        pub password: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub thumbprint: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub valid: Option<bool>,
        #[serde(rename = "cerBlob", default, skip_serializing_if = "Option::is_none")]
        pub cer_blob: Option<String>,
        #[serde(rename = "publicKeyHash", default, skip_serializing_if = "Option::is_none")]
        pub public_key_hash: Option<String>,
        #[serde(rename = "hostingEnvironmentProfile", default, skip_serializing_if = "Option::is_none")]
        pub hosting_environment_profile: Option<HostingEnvironmentProfile>,
        #[serde(rename = "keyVaultId", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_id: Option<String>,
        #[serde(rename = "keyVaultSecretName", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_secret_name: Option<String>,
        #[serde(rename = "keyVaultSecretStatus", default, skip_serializing_if = "Option::is_none")]
        pub key_vault_secret_status: Option<properties::KeyVaultSecretStatus>,
        #[serde(rename = "serverFarmId", default, skip_serializing_if = "Option::is_none")]
        pub server_farm_id: Option<String>,
    }
    impl Properties {
        pub fn new(password: String) -> Self {
            Self {
                friendly_name: None,
                subject_name: None,
                host_names: Vec::new(),
                pfx_blob: None,
                site_name: None,
                self_link: None,
                issuer: None,
                issue_date: None,
                expiration_date: None,
                password,
                thumbprint: None,
                valid: None,
                cer_blob: None,
                public_key_hash: None,
                hosting_environment_profile: None,
                key_vault_id: None,
                key_vault_secret_name: None,
                key_vault_secret_status: None,
                server_farm_id: None,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum KeyVaultSecretStatus {
            Initialized,
            WaitingOnCertificateOrder,
            Succeeded,
            CertificateOrderFailed,
            OperationNotPermittedOnKeyVault,
            AzureServiceUnauthorizedToAccessKeyVault,
            KeyVaultDoesNotExist,
            KeyVaultSecretDoesNotExist,
            UnknownError,
            ExternalPrivateKey,
            Unknown,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DefaultErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<default_error_response::Error>,
}
impl DefaultErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod default_error_response {
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
        pub details: Vec<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub innererror: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HostingEnvironmentProfile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl HostingEnvironmentProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyOnlyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ProxyOnlyResource {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub location: String,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            kind: None,
            location,
            type_: None,
            tags: None,
        }
    }
}
