#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AdditionalFeaturesServerConfigurations {
    #[serde(rename = "isRServicesEnabled", default, skip_serializing_if = "Option::is_none")]
    pub is_r_services_enabled: Option<bool>,
}
impl AdditionalFeaturesServerConfigurations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoBackupSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(rename = "enableEncryption", default, skip_serializing_if = "Option::is_none")]
    pub enable_encryption: Option<bool>,
    #[serde(rename = "retentionPeriod", default, skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
    #[serde(rename = "storageAccountUrl", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_url: Option<String>,
    #[serde(rename = "storageAccessKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_access_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "backupSystemDbs", default, skip_serializing_if = "Option::is_none")]
    pub backup_system_dbs: Option<bool>,
    #[serde(rename = "backupScheduleType", default, skip_serializing_if = "Option::is_none")]
    pub backup_schedule_type: Option<auto_backup_settings::BackupScheduleType>,
    #[serde(rename = "fullBackupFrequency", default, skip_serializing_if = "Option::is_none")]
    pub full_backup_frequency: Option<auto_backup_settings::FullBackupFrequency>,
    #[serde(rename = "fullBackupStartTime", default, skip_serializing_if = "Option::is_none")]
    pub full_backup_start_time: Option<i32>,
    #[serde(rename = "fullBackupWindowHours", default, skip_serializing_if = "Option::is_none")]
    pub full_backup_window_hours: Option<i32>,
    #[serde(rename = "logBackupFrequency", default, skip_serializing_if = "Option::is_none")]
    pub log_backup_frequency: Option<i32>,
}
impl AutoBackupSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auto_backup_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum BackupScheduleType {
        Manual,
        Automated,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FullBackupFrequency {
        Daily,
        Weekly,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AutoPatchingSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(rename = "dayOfWeek", default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<auto_patching_settings::DayOfWeek>,
    #[serde(rename = "maintenanceWindowStartingHour", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window_starting_hour: Option<i32>,
    #[serde(rename = "maintenanceWindowDuration", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_window_duration: Option<i32>,
}
impl AutoPatchingSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod auto_patching_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DayOfWeek {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityGroupListener {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailabilityGroupListenerProperties>,
}
impl AvailabilityGroupListener {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityGroupListenerListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailabilityGroupListener>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AvailabilityGroupListenerListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityGroupListenerProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "availabilityGroupName", default, skip_serializing_if = "Option::is_none")]
    pub availability_group_name: Option<String>,
    #[serde(rename = "loadBalancerConfigurations", default, skip_serializing_if = "Vec::is_empty")]
    pub load_balancer_configurations: Vec<LoadBalancerConfiguration>,
    #[serde(
        rename = "createDefaultAvailabilityGroupIfNotExist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_default_availability_group_if_not_exist: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
impl AvailabilityGroupListenerProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyVaultCredentialSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(rename = "credentialName", default, skip_serializing_if = "Option::is_none")]
    pub credential_name: Option<String>,
    #[serde(rename = "azureKeyVaultUrl", default, skip_serializing_if = "Option::is_none")]
    pub azure_key_vault_url: Option<String>,
    #[serde(rename = "servicePrincipalName", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_name: Option<String>,
    #[serde(rename = "servicePrincipalSecret", default, skip_serializing_if = "Option::is_none")]
    pub service_principal_secret: Option<String>,
}
impl KeyVaultCredentialSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LoadBalancerConfiguration {
    #[serde(rename = "privateIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<PrivateIpAddress>,
    #[serde(rename = "publicIpAddressResourceId", default, skip_serializing_if = "Option::is_none")]
    pub public_ip_address_resource_id: Option<String>,
    #[serde(rename = "loadBalancerResourceId", default, skip_serializing_if = "Option::is_none")]
    pub load_balancer_resource_id: Option<String>,
    #[serde(rename = "probePort", default, skip_serializing_if = "Option::is_none")]
    pub probe_port: Option<i32>,
    #[serde(rename = "sqlVirtualMachineInstances", default, skip_serializing_if = "Vec::is_empty")]
    pub sql_virtual_machine_instances: Vec<String>,
}
impl LoadBalancerConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        #[serde(rename = "user")]
        User,
        #[serde(rename = "system")]
        System,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
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
pub struct PrivateIpAddress {
    #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "subnetResourceId", default, skip_serializing_if = "Option::is_none")]
    pub subnet_resource_id: Option<String>,
}
impl PrivateIpAddress {
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
pub struct ResourceIdentity {
    #[serde(rename = "principalId", default, skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<resource_identity::Type>,
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl ResourceIdentity {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod resource_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlStorageSettings {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub luns: Vec<i32>,
    #[serde(rename = "defaultFilePath", default, skip_serializing_if = "Option::is_none")]
    pub default_file_path: Option<String>,
}
impl SqlStorageSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ServerConfigurationsManagementSettings {
    #[serde(rename = "sqlConnectivityUpdateSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_connectivity_update_settings: Option<SqlConnectivityUpdateSettings>,
    #[serde(rename = "sqlWorkloadTypeUpdateSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_workload_type_update_settings: Option<SqlWorkloadTypeUpdateSettings>,
    #[serde(rename = "sqlStorageUpdateSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_storage_update_settings: Option<SqlStorageUpdateSettings>,
    #[serde(
        rename = "additionalFeaturesServerConfigurations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_features_server_configurations: Option<AdditionalFeaturesServerConfigurations>,
}
impl ServerConfigurationsManagementSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlConnectivityUpdateSettings {
    #[serde(rename = "connectivityType", default, skip_serializing_if = "Option::is_none")]
    pub connectivity_type: Option<sql_connectivity_update_settings::ConnectivityType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "sqlAuthUpdateUserName", default, skip_serializing_if = "Option::is_none")]
    pub sql_auth_update_user_name: Option<String>,
    #[serde(rename = "sqlAuthUpdatePassword", default, skip_serializing_if = "Option::is_none")]
    pub sql_auth_update_password: Option<String>,
}
impl SqlConnectivityUpdateSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_connectivity_update_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ConnectivityType {
        #[serde(rename = "LOCAL")]
        Local,
        #[serde(rename = "PRIVATE")]
        Private,
        #[serde(rename = "PUBLIC")]
        Public,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlStorageUpdateSettings {
    #[serde(rename = "diskCount", default, skip_serializing_if = "Option::is_none")]
    pub disk_count: Option<i32>,
    #[serde(rename = "startingDeviceId", default, skip_serializing_if = "Option::is_none")]
    pub starting_device_id: Option<i32>,
    #[serde(rename = "diskConfigurationType", default, skip_serializing_if = "Option::is_none")]
    pub disk_configuration_type: Option<sql_storage_update_settings::DiskConfigurationType>,
}
impl SqlStorageUpdateSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_storage_update_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DiskConfigurationType {
        #[serde(rename = "NEW")]
        New,
        #[serde(rename = "EXTEND")]
        Extend,
        #[serde(rename = "ADD")]
        Add,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlVirtualMachine {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<ResourceIdentity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlVirtualMachineProperties>,
}
impl SqlVirtualMachine {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            identity: None,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SqlVirtualMachineGroup {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SqlVirtualMachineGroupProperties>,
}
impl SqlVirtualMachineGroup {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineGroupListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlVirtualMachineGroup>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SqlVirtualMachineGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineGroupProperties {
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "sqlImageOffer", default, skip_serializing_if = "Option::is_none")]
    pub sql_image_offer: Option<String>,
    #[serde(rename = "sqlImageSku", default, skip_serializing_if = "Option::is_none")]
    pub sql_image_sku: Option<sql_virtual_machine_group_properties::SqlImageSku>,
    #[serde(rename = "scaleType", default, skip_serializing_if = "Option::is_none")]
    pub scale_type: Option<sql_virtual_machine_group_properties::ScaleType>,
    #[serde(rename = "clusterManagerType", default, skip_serializing_if = "Option::is_none")]
    pub cluster_manager_type: Option<sql_virtual_machine_group_properties::ClusterManagerType>,
    #[serde(rename = "clusterConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub cluster_configuration: Option<sql_virtual_machine_group_properties::ClusterConfiguration>,
    #[serde(rename = "wsfcDomainProfile", default, skip_serializing_if = "Option::is_none")]
    pub wsfc_domain_profile: Option<WsfcDomainProfile>,
}
impl SqlVirtualMachineGroupProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_virtual_machine_group_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SqlImageSku {
        Developer,
        Enterprise,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ScaleType {
        #[serde(rename = "HA")]
        Ha,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ClusterManagerType {
        #[serde(rename = "WSFC")]
        Wsfc,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ClusterConfiguration {
        Domainful,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineGroupUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SqlVirtualMachineGroupUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<SqlVirtualMachine>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SqlVirtualMachineListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineProperties {
    #[serde(rename = "virtualMachineResourceId", default, skip_serializing_if = "Option::is_none")]
    pub virtual_machine_resource_id: Option<String>,
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(rename = "sqlImageOffer", default, skip_serializing_if = "Option::is_none")]
    pub sql_image_offer: Option<String>,
    #[serde(rename = "sqlServerLicenseType", default, skip_serializing_if = "Option::is_none")]
    pub sql_server_license_type: Option<sql_virtual_machine_properties::SqlServerLicenseType>,
    #[serde(rename = "sqlManagement", default, skip_serializing_if = "Option::is_none")]
    pub sql_management: Option<sql_virtual_machine_properties::SqlManagement>,
    #[serde(rename = "sqlImageSku", default, skip_serializing_if = "Option::is_none")]
    pub sql_image_sku: Option<sql_virtual_machine_properties::SqlImageSku>,
    #[serde(rename = "sqlVirtualMachineGroupResourceId", default, skip_serializing_if = "Option::is_none")]
    pub sql_virtual_machine_group_resource_id: Option<String>,
    #[serde(rename = "wsfcDomainCredentials", default, skip_serializing_if = "Option::is_none")]
    pub wsfc_domain_credentials: Option<WsfcDomainCredentials>,
    #[serde(rename = "autoPatchingSettings", default, skip_serializing_if = "Option::is_none")]
    pub auto_patching_settings: Option<AutoPatchingSettings>,
    #[serde(rename = "autoBackupSettings", default, skip_serializing_if = "Option::is_none")]
    pub auto_backup_settings: Option<AutoBackupSettings>,
    #[serde(rename = "keyVaultCredentialSettings", default, skip_serializing_if = "Option::is_none")]
    pub key_vault_credential_settings: Option<KeyVaultCredentialSettings>,
    #[serde(
        rename = "serverConfigurationsManagementSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_configurations_management_settings: Option<ServerConfigurationsManagementSettings>,
    #[serde(rename = "storageConfigurationSettings", default, skip_serializing_if = "Option::is_none")]
    pub storage_configuration_settings: Option<StorageConfigurationSettings>,
}
impl SqlVirtualMachineProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_virtual_machine_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SqlServerLicenseType {
        #[serde(rename = "PAYG")]
        Payg,
        #[serde(rename = "AHUB")]
        Ahub,
        #[serde(rename = "DR")]
        Dr,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SqlManagement {
        Full,
        LightWeight,
        NoAgent,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SqlImageSku {
        Developer,
        Express,
        Standard,
        Enterprise,
        Web,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlVirtualMachineUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl SqlVirtualMachineUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SqlWorkloadTypeUpdateSettings {
    #[serde(rename = "sqlWorkloadType", default, skip_serializing_if = "Option::is_none")]
    pub sql_workload_type: Option<sql_workload_type_update_settings::SqlWorkloadType>,
}
impl SqlWorkloadTypeUpdateSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod sql_workload_type_update_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum SqlWorkloadType {
        #[serde(rename = "GENERAL")]
        General,
        #[serde(rename = "OLTP")]
        Oltp,
        #[serde(rename = "DW")]
        Dw,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageConfigurationSettings {
    #[serde(rename = "sqlDataSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_data_settings: Option<SqlStorageSettings>,
    #[serde(rename = "sqlLogSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_log_settings: Option<SqlStorageSettings>,
    #[serde(rename = "sqlTempDbSettings", default, skip_serializing_if = "Option::is_none")]
    pub sql_temp_db_settings: Option<SqlStorageSettings>,
    #[serde(rename = "diskConfigurationType", default, skip_serializing_if = "Option::is_none")]
    pub disk_configuration_type: Option<storage_configuration_settings::DiskConfigurationType>,
    #[serde(rename = "storageWorkloadType", default, skip_serializing_if = "Option::is_none")]
    pub storage_workload_type: Option<storage_configuration_settings::StorageWorkloadType>,
}
impl StorageConfigurationSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod storage_configuration_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DiskConfigurationType {
        #[serde(rename = "NEW")]
        New,
        #[serde(rename = "EXTEND")]
        Extend,
        #[serde(rename = "ADD")]
        Add,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum StorageWorkloadType {
        #[serde(rename = "GENERAL")]
        General,
        #[serde(rename = "OLTP")]
        Oltp,
        #[serde(rename = "DW")]
        Dw,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WsfcDomainCredentials {
    #[serde(rename = "clusterBootstrapAccountPassword", default, skip_serializing_if = "Option::is_none")]
    pub cluster_bootstrap_account_password: Option<String>,
    #[serde(rename = "clusterOperatorAccountPassword", default, skip_serializing_if = "Option::is_none")]
    pub cluster_operator_account_password: Option<String>,
    #[serde(rename = "sqlServiceAccountPassword", default, skip_serializing_if = "Option::is_none")]
    pub sql_service_account_password: Option<String>,
}
impl WsfcDomainCredentials {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WsfcDomainProfile {
    #[serde(rename = "domainFqdn", default, skip_serializing_if = "Option::is_none")]
    pub domain_fqdn: Option<String>,
    #[serde(rename = "ouPath", default, skip_serializing_if = "Option::is_none")]
    pub ou_path: Option<String>,
    #[serde(rename = "clusterBootstrapAccount", default, skip_serializing_if = "Option::is_none")]
    pub cluster_bootstrap_account: Option<String>,
    #[serde(rename = "clusterOperatorAccount", default, skip_serializing_if = "Option::is_none")]
    pub cluster_operator_account: Option<String>,
    #[serde(rename = "sqlServiceAccount", default, skip_serializing_if = "Option::is_none")]
    pub sql_service_account: Option<String>,
    #[serde(rename = "fileShareWitnessPath", default, skip_serializing_if = "Option::is_none")]
    pub file_share_witness_path: Option<String>,
    #[serde(rename = "storageAccountUrl", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_url: Option<String>,
    #[serde(rename = "storageAccountPrimaryKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_primary_key: Option<String>,
}
impl WsfcDomainProfile {
    pub fn new() -> Self {
        Self::default()
    }
}
