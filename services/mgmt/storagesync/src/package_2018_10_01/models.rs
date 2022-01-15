#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BackupRequest {
    #[serde(rename = "azureFileShare", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share: Option<String>,
}
impl BackupRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckNameAvailabilityParameters {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: check_name_availability_parameters::Type,
}
impl CheckNameAvailabilityParameters {
    pub fn new(name: String, type_: check_name_availability_parameters::Type) -> Self {
        Self { name, type_ }
    }
}
pub mod check_name_availability_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        #[serde(rename = "Microsoft.StorageSync/storageSyncServices")]
        MicrosoftStorageSyncStorageSyncServices,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckNameAvailabilityResult {
    #[serde(rename = "nameAvailable", default, skip_serializing_if = "Option::is_none")]
    pub name_available: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<check_name_availability_result::Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl CheckNameAvailabilityResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_name_availability_result {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Reason {
        Invalid,
        AlreadyExists,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudEndpointProperties>,
}
impl CloudEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<CloudEndpoint>,
}
impl CloudEndpointArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<CloudEndpointCreateParametersProperties>,
}
impl CloudEndpointCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointCreateParametersProperties {
    #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[serde(rename = "storageAccountShareName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_share_name: Option<String>,
    #[serde(rename = "storageAccountTenantId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_tenant_id: Option<String>,
}
impl CloudEndpointCreateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudEndpointProperties {
    #[serde(rename = "storageAccountResourceId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_resource_id: Option<String>,
    #[serde(rename = "storageAccountShareName", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_share_name: Option<String>,
    #[serde(rename = "storageAccountTenantId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_tenant_id: Option<String>,
    #[serde(rename = "partnershipId", default, skip_serializing_if = "Option::is_none")]
    pub partnership_id: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "backupEnabled", default, skip_serializing_if = "Option::is_none")]
    pub backup_enabled: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "lastWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[serde(rename = "lastOperationName", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
}
impl CloudEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FeatureStatus {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HealthState {
    Healthy,
    Error,
    SyncBlockedForRestore,
    SyncBlockedForChangeDetectionPostRestore,
    NoActivity,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OfflineDataTransferState {
    InProgress,
    Stopping,
    NotRunning,
    Complete,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OperationDirection {
    #[serde(rename = "do")]
    Do,
    #[serde(rename = "undo")]
    Undo,
    #[serde(rename = "cancel")]
    Cancel,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}
impl OperationDisplayInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplayResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplayResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplayInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationEntityListResult {
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationEntity>,
}
impl OperationEntityListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type PhysicalPath = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostBackupResponse {
    #[serde(rename = "backupMetadata", default, skip_serializing_if = "Option::is_none")]
    pub backup_metadata: Option<PostBackupResponseProperties>,
}
impl PostBackupResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostBackupResponseProperties {
    #[serde(rename = "cloudEndpointName", default, skip_serializing_if = "Option::is_none")]
    pub cloud_endpoint_name: Option<String>,
}
impl PostBackupResponseProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PostRestoreRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(rename = "replicaGroup", default, skip_serializing_if = "Option::is_none")]
    pub replica_group: Option<String>,
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "azureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "sourceAzureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub source_azure_file_share_uri: Option<String>,
    #[serde(rename = "failedFileList", default, skip_serializing_if = "Option::is_none")]
    pub failed_file_list: Option<String>,
    #[serde(rename = "restoreFileSpec", default, skip_serializing_if = "Vec::is_empty")]
    pub restore_file_spec: Vec<RestoreFileSpec>,
}
impl PostRestoreRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PreRestoreRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(rename = "replicaGroup", default, skip_serializing_if = "Option::is_none")]
    pub replica_group: Option<String>,
    #[serde(rename = "requestId", default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "azureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub azure_file_share_uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "sourceAzureFileShareUri", default, skip_serializing_if = "Option::is_none")]
    pub source_azure_file_share_uri: Option<String>,
    #[serde(rename = "backupMetadataPropertyBag", default, skip_serializing_if = "Option::is_none")]
    pub backup_metadata_property_bag: Option<String>,
    #[serde(rename = "restoreFileSpec", default, skip_serializing_if = "Vec::is_empty")]
    pub restore_file_spec: Vec<RestoreFileSpec>,
    #[serde(
        rename = "pauseWaitForSyncDrainTimePeriodInSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pause_wait_for_sync_drain_time_period_in_seconds: Option<i64>,
}
impl PreRestoreRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ProgressType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "initialize")]
    Initialize,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "upload")]
    Upload,
    #[serde(rename = "recall")]
    Recall,
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
pub struct RecallActionParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(rename = "recallPath", default, skip_serializing_if = "Option::is_none")]
    pub recall_path: Option<String>,
}
impl RecallActionParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServer {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegisteredServerProperties>,
}
impl RegisteredServer {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RegisteredServer>,
}
impl RegisteredServerArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<RegisteredServerCreateParametersProperties>,
}
impl RegisteredServerCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerCreateParametersProperties {
    #[serde(rename = "serverCertificate", default, skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "serverOSVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_os_version: Option<String>,
    #[serde(rename = "lastHeartBeat", default, skip_serializing_if = "Option::is_none")]
    pub last_heart_beat: Option<String>,
    #[serde(rename = "serverRole", default, skip_serializing_if = "Option::is_none")]
    pub server_role: Option<String>,
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "serverId", default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
}
impl RegisteredServerCreateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RegisteredServerProperties {
    #[serde(rename = "serverCertificate", default, skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
    #[serde(rename = "agentVersion", default, skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "serverOSVersion", default, skip_serializing_if = "Option::is_none")]
    pub server_os_version: Option<String>,
    #[serde(rename = "serverManagementErrorCode", default, skip_serializing_if = "Option::is_none")]
    pub server_management_error_code: Option<i64>,
    #[serde(rename = "lastHeartBeat", default, skip_serializing_if = "Option::is_none")]
    pub last_heart_beat: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "serverRole", default, skip_serializing_if = "Option::is_none")]
    pub server_role: Option<String>,
    #[serde(rename = "clusterId", default, skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "clusterName", default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "serverId", default, skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "storageSyncServiceUid", default, skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_uid: Option<String>,
    #[serde(rename = "lastWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[serde(rename = "lastOperationName", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
    #[serde(rename = "discoveryEndpointUri", default, skip_serializing_if = "Option::is_none")]
    pub discovery_endpoint_uri: Option<String>,
    #[serde(rename = "resourceLocation", default, skip_serializing_if = "Option::is_none")]
    pub resource_location: Option<String>,
    #[serde(rename = "serviceLocation", default, skip_serializing_if = "Option::is_none")]
    pub service_location: Option<String>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "managementEndpointUri", default, skip_serializing_if = "Option::is_none")]
    pub management_endpoint_uri: Option<String>,
    #[serde(rename = "monitoringConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<String>,
}
impl RegisteredServerProperties {
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
pub type ResourceId = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourcesMoveInfo {
    #[serde(rename = "targetResourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub target_resource_group: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceId>,
}
impl ResourcesMoveInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct RestoreFileSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub isdir: Option<bool>,
}
impl RestoreFileSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpoint {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointProperties>,
}
impl ServerEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ServerEndpoint>,
}
impl ServerEndpointArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointCreateParametersProperties>,
}
impl ServerEndpointCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointCreateParametersProperties {
    #[serde(rename = "serverLocalPath", default, skip_serializing_if = "Option::is_none")]
    pub server_local_path: Option<PhysicalPath>,
    #[serde(rename = "cloudTiering", default, skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[serde(rename = "volumeFreeSpacePercent", default, skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[serde(rename = "tierFilesOlderThanDays", default, skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "serverResourceId", default, skip_serializing_if = "Option::is_none")]
    pub server_resource_id: Option<ResourceId>,
    #[serde(rename = "offlineDataTransfer", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[serde(rename = "offlineDataTransferShareName", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
}
impl ServerEndpointCreateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointHealth {
    #[serde(rename = "downloadHealth", default, skip_serializing_if = "Option::is_none")]
    pub download_health: Option<HealthState>,
    #[serde(rename = "uploadHealth", default, skip_serializing_if = "Option::is_none")]
    pub upload_health: Option<HealthState>,
    #[serde(rename = "combinedHealth", default, skip_serializing_if = "Option::is_none")]
    pub combined_health: Option<HealthState>,
    #[serde(rename = "lastUpdatedTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "uploadStatus", default, skip_serializing_if = "Option::is_none")]
    pub upload_status: Option<SyncSessionStatus>,
    #[serde(rename = "downloadStatus", default, skip_serializing_if = "Option::is_none")]
    pub download_status: Option<SyncSessionStatus>,
    #[serde(rename = "currentProgress", default, skip_serializing_if = "Option::is_none")]
    pub current_progress: Option<SyncProgressStatus>,
    #[serde(rename = "offlineDataTransferStatus", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_status: Option<OfflineDataTransferState>,
}
impl ServerEndpointHealth {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointProperties {
    #[serde(rename = "serverLocalPath", default, skip_serializing_if = "Option::is_none")]
    pub server_local_path: Option<PhysicalPath>,
    #[serde(rename = "cloudTiering", default, skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[serde(rename = "volumeFreeSpacePercent", default, skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[serde(rename = "tierFilesOlderThanDays", default, skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[serde(rename = "friendlyName", default, skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "serverResourceId", default, skip_serializing_if = "Option::is_none")]
    pub server_resource_id: Option<ResourceId>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "lastWorkflowId", default, skip_serializing_if = "Option::is_none")]
    pub last_workflow_id: Option<String>,
    #[serde(rename = "lastOperationName", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_name: Option<String>,
    #[serde(rename = "syncStatus", default, skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<ServerEndpointHealth>,
    #[serde(rename = "offlineDataTransfer", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[serde(
        rename = "offlineDataTransferStorageAccountResourceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offline_data_transfer_storage_account_resource_id: Option<String>,
    #[serde(
        rename = "offlineDataTransferStorageAccountTenantId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub offline_data_transfer_storage_account_tenant_id: Option<String>,
    #[serde(rename = "offlineDataTransferShareName", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
}
impl ServerEndpointProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ServerEndpointUpdateProperties>,
}
impl ServerEndpointUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerEndpointUpdateProperties {
    #[serde(rename = "cloudTiering", default, skip_serializing_if = "Option::is_none")]
    pub cloud_tiering: Option<FeatureStatus>,
    #[serde(rename = "volumeFreeSpacePercent", default, skip_serializing_if = "Option::is_none")]
    pub volume_free_space_percent: Option<i64>,
    #[serde(rename = "tierFilesOlderThanDays", default, skip_serializing_if = "Option::is_none")]
    pub tier_files_older_than_days: Option<i64>,
    #[serde(rename = "offlineDataTransfer", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer: Option<FeatureStatus>,
    #[serde(rename = "offlineDataTransferShareName", default, skip_serializing_if = "Option::is_none")]
    pub offline_data_transfer_share_name: Option<String>,
}
impl ServerEndpointUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncApiError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<StorageSyncErrorDetails>,
}
impl StorageSyncApiError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<StorageSyncApiError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Option<StorageSyncApiError>,
}
impl StorageSyncError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
impl StorageSyncErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncService {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageSyncServiceProperties>,
}
impl StorageSyncService {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StorageSyncService>,
}
impl StorageSyncServiceArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageSyncServiceCreateParameters {
    pub location: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl StorageSyncServiceCreateParameters {
    pub fn new(location: String) -> Self {
        Self {
            location,
            tags: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceProperties {
    #[serde(rename = "storageSyncServiceStatus", default, skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_status: Option<i64>,
    #[serde(rename = "storageSyncServiceUid", default, skip_serializing_if = "Option::is_none")]
    pub storage_sync_service_uid: Option<String>,
}
impl StorageSyncServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceUpdateParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<StorageSyncServiceUpdateProperties>,
}
impl StorageSyncServiceUpdateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageSyncServiceUpdateProperties {}
impl StorageSyncServiceUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<subscription_state::State>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub istransitioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SubscriptionStateProperties>,
}
impl SubscriptionState {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod subscription_state {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Registered,
        Unregistered,
        Warned,
        Suspended,
        Deleted,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionStateProperties {}
impl SubscriptionStateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroup {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncGroupProperties>,
}
impl SyncGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SyncGroup>,
}
impl SyncGroupArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupCreateParameters {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SyncGroupCreateParametersProperties>,
}
impl SyncGroupCreateParameters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupCreateParametersProperties {}
impl SyncGroupCreateParametersProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncGroupProperties {
    #[serde(rename = "uniqueId", default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(rename = "syncGroupStatus", default, skip_serializing_if = "Option::is_none")]
    pub sync_group_status: Option<String>,
}
impl SyncGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncProgressStatus {
    #[serde(rename = "progressTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub progress_timestamp: Option<String>,
    #[serde(rename = "syncDirection", default, skip_serializing_if = "Option::is_none")]
    pub sync_direction: Option<ProgressType>,
    #[serde(rename = "perItemErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub per_item_error_count: Option<i64>,
    #[serde(rename = "appliedItemCount", default, skip_serializing_if = "Option::is_none")]
    pub applied_item_count: Option<i64>,
    #[serde(rename = "totalItemCount", default, skip_serializing_if = "Option::is_none")]
    pub total_item_count: Option<i64>,
    #[serde(rename = "appliedBytes", default, skip_serializing_if = "Option::is_none")]
    pub applied_bytes: Option<i64>,
    #[serde(rename = "totalBytes", default, skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<i64>,
}
impl SyncProgressStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyncSessionStatus {
    #[serde(rename = "lastSyncResult", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_result: Option<i64>,
    #[serde(rename = "lastSyncTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_timestamp: Option<String>,
    #[serde(rename = "lastSyncSuccessTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_success_timestamp: Option<String>,
    #[serde(rename = "lastSyncPerItemErrorCount", default, skip_serializing_if = "Option::is_none")]
    pub last_sync_per_item_error_count: Option<i64>,
}
impl SyncSessionStatus {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {}
impl TagsObject {
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
pub struct TriggerRolloverRequest {
    #[serde(rename = "serverCertificate", default, skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,
}
impl TriggerRolloverRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Workflow {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<WorkflowProperties>,
}
impl Workflow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowArray {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Workflow>,
}
impl WorkflowArray {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WorkflowProperties {
    #[serde(rename = "lastStepName", default, skip_serializing_if = "Option::is_none")]
    pub last_step_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkflowStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<OperationDirection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub steps: Option<String>,
    #[serde(rename = "lastOperationId", default, skip_serializing_if = "Option::is_none")]
    pub last_operation_id: Option<String>,
}
impl WorkflowProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "aborted")]
    Aborted,
    #[serde(rename = "failed")]
    Failed,
}
