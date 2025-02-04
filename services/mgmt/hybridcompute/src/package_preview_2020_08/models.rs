#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailablePatchCountByClassification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub critical: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub definition: Option<i32>,
    #[serde(rename = "updateRollup", default, skip_serializing_if = "Option::is_none")]
    pub update_rollup: Option<i32>,
    #[serde(rename = "featurePack", default, skip_serializing_if = "Option::is_none")]
    pub feature_pack: Option<i32>,
    #[serde(rename = "servicePack", default, skip_serializing_if = "Option::is_none")]
    pub service_pack: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updates: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub other: Option<i32>,
}
impl AvailablePatchCountByClassification {
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            details: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorDetail,
}
impl ErrorResponse {
    pub fn new(error: ErrorDetail) -> Self {
        Self { error }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseCommon {
    #[serde(flatten)]
    pub error_response_v2: ErrorResponseV2,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorResponseCommon>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorResponseCommon {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseV2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response_v2::Error>,
}
impl ErrorResponseV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error_response_v2 {
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
        pub details: Vec<ErrorResponseV2>,
        #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
        pub additional_info: Vec<ErrorAdditionalInfo>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridComputePrivateLinkScope {
    #[serde(flatten)]
    pub private_link_scopes_resource: PrivateLinkScopesResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<HybridComputePrivateLinkScopeProperties>,
}
impl HybridComputePrivateLinkScope {
    pub fn new(private_link_scopes_resource: PrivateLinkScopesResource) -> Self {
        Self {
            private_link_scopes_resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HybridComputePrivateLinkScopeListResult {
    pub value: Vec<HybridComputePrivateLinkScope>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl HybridComputePrivateLinkScopeListResult {
    pub fn new(value: Vec<HybridComputePrivateLinkScope>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HybridComputePrivateLinkScopeProperties {
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<PublicNetworkAccessType>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "privateEndpointConnections", default, skip_serializing_if = "Vec::is_empty")]
    pub private_endpoint_connections: Vec<PrivateEndpointConnection>,
}
impl HybridComputePrivateLinkScopeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Identity {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl Identity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LinuxParameters {
    #[serde(rename = "classificationsToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub classifications_to_include: Vec<String>,
    #[serde(rename = "packageNameMasksToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub package_name_masks_to_include: Vec<String>,
    #[serde(rename = "packageNameMasksToExclude", default, skip_serializing_if = "Vec::is_empty")]
    pub package_name_masks_to_exclude: Vec<String>,
}
impl LinuxParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Machine {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<serde_json::Value>,
}
impl Machine {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            identity: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineAssessPatchesResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_assess_patches_result::Status>,
    #[serde(rename = "assessmentActivityId", default, skip_serializing_if = "Option::is_none")]
    pub assessment_activity_id: Option<String>,
    #[serde(rename = "rebootPending", default, skip_serializing_if = "Option::is_none")]
    pub reboot_pending: Option<bool>,
    #[serde(rename = "availablePatchCountByClassification", default, skip_serializing_if = "Option::is_none")]
    pub available_patch_count_by_classification: Option<AvailablePatchCountByClassification>,
    #[serde(rename = "startDateTime", default, skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "startedBy", default, skip_serializing_if = "Option::is_none")]
    pub started_by: Option<machine_assess_patches_result::StartedBy>,
    #[serde(rename = "patchServiceUsed", default, skip_serializing_if = "Option::is_none")]
    pub patch_service_used: Option<machine_assess_patches_result::PatchServiceUsed>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<machine_assess_patches_result::OsType>,
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetail>,
}
impl MachineAssessPatchesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_assess_patches_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Unknown,
        InProgress,
        Failed,
        Succeeded,
        CompletedWithWarnings,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StartedBy {
        User,
        Platform,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PatchServiceUsed {
        Unknown,
        #[serde(rename = "WU")]
        Wu,
        #[serde(rename = "WU_WSUS")]
        WuWsus,
        #[serde(rename = "YUM")]
        Yum,
        #[serde(rename = "APT")]
        Apt,
        Zypper,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineExtension {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl MachineExtension {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionInstanceView {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_extension_instance_view::Status>,
}
impl MachineExtensionInstanceView {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_extension_instance_view {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Status {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<status::Level>,
        #[serde(rename = "displayStatus", default, skip_serializing_if = "Option::is_none")]
        pub display_status: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub time: Option<String>,
    }
    impl Status {
        pub fn new() -> Self {
            Self::default()
        }
    }
    pub mod status {
        use super::*;
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum Level {
            Info,
            Warning,
            Error,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionProperties {
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<serde_json::Value>,
}
impl MachineExtensionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl MachineExtensionUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionUpdateProperties {
    #[serde(rename = "forceUpdateTag", default, skip_serializing_if = "Option::is_none")]
    pub force_update_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "typeHandlerVersion", default, skip_serializing_if = "Option::is_none")]
    pub type_handler_version: Option<String>,
    #[serde(rename = "autoUpgradeMinorVersion", default, skip_serializing_if = "Option::is_none")]
    pub auto_upgrade_minor_version: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "protectedSettings", default, skip_serializing_if = "Option::is_none")]
    pub protected_settings: Option<serde_json::Value>,
}
impl MachineExtensionUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineExtensionsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MachineExtension>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl MachineExtensionsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineInstallPatchesParameters {
    #[serde(rename = "maximumDuration")]
    pub maximum_duration: String,
    #[serde(rename = "rebootSetting")]
    pub reboot_setting: machine_install_patches_parameters::RebootSetting,
    #[serde(rename = "windowsParameters", default, skip_serializing_if = "Option::is_none")]
    pub windows_parameters: Option<WindowsParameters>,
    #[serde(rename = "linuxParameters", default, skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
}
impl MachineInstallPatchesParameters {
    pub fn new(maximum_duration: String, reboot_setting: machine_install_patches_parameters::RebootSetting) -> Self {
        Self {
            maximum_duration,
            reboot_setting,
            windows_parameters: None,
            linux_parameters: None,
        }
    }
}
pub mod machine_install_patches_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RebootSetting {
        IfRequired,
        Never,
        Always,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineInstallPatchesResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_install_patches_result::Status>,
    #[serde(rename = "installationActivityId", default, skip_serializing_if = "Option::is_none")]
    pub installation_activity_id: Option<String>,
    #[serde(rename = "rebootStatus", default, skip_serializing_if = "Option::is_none")]
    pub reboot_status: Option<machine_install_patches_result::RebootStatus>,
    #[serde(rename = "maintenanceWindowExceeded", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window_exceeded: Option<bool>,
    #[serde(rename = "excludedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub excluded_patch_count: Option<i32>,
    #[serde(rename = "notSelectedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub not_selected_patch_count: Option<i32>,
    #[serde(rename = "pendingPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub pending_patch_count: Option<i32>,
    #[serde(rename = "installedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub installed_patch_count: Option<i32>,
    #[serde(rename = "failedPatchCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_patch_count: Option<i32>,
    #[serde(rename = "startDateTime", default, skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
    #[serde(rename = "lastModifiedDateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_date_time: Option<String>,
    #[serde(rename = "startedBy", default, skip_serializing_if = "Option::is_none")]
    pub started_by: Option<machine_install_patches_result::StartedBy>,
    #[serde(rename = "patchServiceUsed", default, skip_serializing_if = "Option::is_none")]
    pub patch_service_used: Option<machine_install_patches_result::PatchServiceUsed>,
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<machine_install_patches_result::OsType>,
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetail>,
}
impl MachineInstallPatchesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_install_patches_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Unknown,
        InProgress,
        Failed,
        Succeeded,
        CompletedWithWarnings,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum RebootStatus {
        Unknown,
        NotNeeded,
        Required,
        Started,
        Failed,
        Completed,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StartedBy {
        User,
        Platform,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PatchServiceUsed {
        Unknown,
        #[serde(rename = "WU")]
        Wu,
        #[serde(rename = "WU_WSUS")]
        WuWsus,
        #[serde(rename = "YUM")]
        Yum,
        #[serde(rename = "APT")]
        Apt,
        Zypper,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MachineListResult {
    pub value: Vec<Machine>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl MachineListResult {
    pub fn new(value: Vec<Machine>) -> Self {
        Self { value, next_link: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineProperties {
    #[serde(rename = "locationData", default, skip_serializing_if = "Option::is_none")]
    pub location_data: Option<LocationData>,
    #[serde(rename = "osProfile", default, skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<serde_json::Value>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<machine_properties::Status>,
    #[serde(rename = "lastStatusChange", default, skip_serializing_if = "Option::is_none")]
    pub last_status_change: Option<String>,
    #[serde(rename = "errorDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub error_details: Vec<ErrorDetail>,
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "vmId", default, skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "machineFqdn", default, skip_serializing_if = "Option::is_none")]
    pub machine_fqdn: Option<String>,
    #[serde(rename = "clientPublicKey", default, skip_serializing_if = "Option::is_none")]
    pub client_public_key: Option<String>,
    #[serde(rename = "osName", default, skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    #[serde(rename = "osVersion", default, skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    #[serde(rename = "vmUuid", default, skip_serializing_if = "Option::is_none")]
    pub vm_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<MachineExtensionInstanceView>,
    #[serde(rename = "osSku", default, skip_serializing_if = "Option::is_none")]
    pub os_sku: Option<String>,
    #[serde(rename = "domainName", default, skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "adFqdn", default, skip_serializing_if = "Option::is_none")]
    pub ad_fqdn: Option<String>,
    #[serde(rename = "dnsFqdn", default, skip_serializing_if = "Option::is_none")]
    pub dns_fqdn: Option<String>,
    #[serde(rename = "privateLinkScopedResources", default, skip_serializing_if = "Vec::is_empty")]
    pub private_link_scoped_resources: Vec<String>,
}
impl MachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod machine_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Connected,
        Disconnected,
        Error,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineUpdate {
    #[serde(flatten)]
    pub update_resource: UpdateResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl MachineUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MachineUpdateProperties {
    #[serde(rename = "locationData", default, skip_serializing_if = "Option::is_none")]
    pub location_data: Option<LocationData>,
}
impl MachineUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OsProfile {
    #[serde(rename = "computerName", default, skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
}
impl OsProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationValue>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<serde_json::Value>,
}
impl OperationValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationValueDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
}
impl OperationValueDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnection {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateEndpointConnectionProperties>,
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
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PrivateEndpointConnectionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointConnectionProperties {
    #[serde(rename = "privateEndpoint", default, skip_serializing_if = "Option::is_none")]
    pub private_endpoint: Option<PrivateEndpointProperty>,
    #[serde(rename = "privateLinkServiceConnectionState", default, skip_serializing_if = "Option::is_none")]
    pub private_link_service_connection_state: Option<PrivateLinkServiceConnectionStateProperty>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl PrivateEndpointConnectionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateEndpointProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl PrivateEndpointProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PrivateLinkResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
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
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
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
}
impl PrivateLinkResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivateLinkScopesResource {
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
impl PrivateLinkScopesResource {
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
pub struct PrivateLinkServiceConnectionStateProperty {
    pub status: String,
    pub description: String,
    #[serde(rename = "actionsRequired", default, skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<String>,
}
impl PrivateLinkServiceConnectionStateProperty {
    pub fn new(status: String, description: String) -> Self {
        Self {
            status,
            description,
            actions_required: None,
        }
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
pub enum PublicNetworkAccessType {
    Enabled,
    Disabled,
}
impl Default for PublicNetworkAccessType {
    fn default() -> Self {
        Self::Disabled
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
pub struct ScopedResource {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ScopedResourceProperties>,
}
impl ScopedResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopedResourceListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ScopedResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ScopedResourceListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ScopedResourceProperties {
    #[serde(rename = "linkedResourceId", default, skip_serializing_if = "Option::is_none")]
    pub linked_resource_id: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl ScopedResourceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl TagsResource {
    pub fn new() -> Self {
        Self::default()
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl UpdateResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsParameters {
    #[serde(rename = "classificationsToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub classifications_to_include: Vec<String>,
    #[serde(rename = "kbNumbersToInclude", default, skip_serializing_if = "Vec::is_empty")]
    pub kb_numbers_to_include: Vec<String>,
    #[serde(rename = "kbNumbersToExclude", default, skip_serializing_if = "Vec::is_empty")]
    pub kb_numbers_to_exclude: Vec<String>,
    #[serde(rename = "excludeKbsRequiringReboot", default, skip_serializing_if = "Option::is_none")]
    pub exclude_kbs_requiring_reboot: Option<bool>,
    #[serde(rename = "maxPatchPublishDate", default, skip_serializing_if = "Option::is_none")]
    pub max_patch_publish_date: Option<String>,
}
impl WindowsParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocationData {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    #[serde(rename = "countryOrRegion", default, skip_serializing_if = "Option::is_none")]
    pub country_or_region: Option<String>,
}
impl LocationData {
    pub fn new(name: String) -> Self {
        Self {
            name,
            city: None,
            district: None,
            country_or_region: None,
        }
    }
}
