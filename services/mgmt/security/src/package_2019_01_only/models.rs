#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Alert {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertProperties>,
}
impl Alert {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertConfidenceReason {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
impl AlertConfidenceReason {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertEntity {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl AlertEntity {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertExtendedProperties {}
impl AlertExtendedProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Alert>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AlertList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "reportedTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub reported_time_utc: Option<String>,
    #[serde(rename = "vendorName", default, skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[serde(rename = "alertName", default, skip_serializing_if = "Option::is_none")]
    pub alert_name: Option<String>,
    #[serde(rename = "alertDisplayName", default, skip_serializing_if = "Option::is_none")]
    pub alert_display_name: Option<String>,
    #[serde(rename = "detectedTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub detected_time_utc: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "remediationSteps", default, skip_serializing_if = "Option::is_none")]
    pub remediation_steps: Option<String>,
    #[serde(rename = "actionTaken", default, skip_serializing_if = "Option::is_none")]
    pub action_taken: Option<String>,
    #[serde(rename = "reportedSeverity", default, skip_serializing_if = "Option::is_none")]
    pub reported_severity: Option<alert_properties::ReportedSeverity>,
    #[serde(rename = "compromisedEntity", default, skip_serializing_if = "Option::is_none")]
    pub compromised_entity: Option<String>,
    #[serde(rename = "associatedResource", default, skip_serializing_if = "Option::is_none")]
    pub associated_resource: Option<String>,
    #[serde(rename = "extendedProperties", default, skip_serializing_if = "Option::is_none")]
    pub extended_properties: Option<AlertExtendedProperties>,
    #[serde(rename = "systemSource", default, skip_serializing_if = "Option::is_none")]
    pub system_source: Option<String>,
    #[serde(rename = "canBeInvestigated", default, skip_serializing_if = "Option::is_none")]
    pub can_be_investigated: Option<bool>,
    #[serde(rename = "isIncident", default, skip_serializing_if = "Option::is_none")]
    pub is_incident: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entities: Vec<AlertEntity>,
    #[serde(rename = "confidenceScore", default, skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<f32>,
    #[serde(rename = "confidenceReasons", default, skip_serializing_if = "Vec::is_empty")]
    pub confidence_reasons: Vec<AlertConfidenceReason>,
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "workspaceArmId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_arm_id: Option<String>,
    #[serde(rename = "correlationKey", default, skip_serializing_if = "Option::is_none")]
    pub correlation_key: Option<String>,
}
impl AlertProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod alert_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ReportedSeverity {
        Informational,
        Low,
        Medium,
        High,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<CloudErrorBody>,
}
impl CloudError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CloudErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CloudErrorBody>,
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportSettingProperties {
    pub enabled: bool,
}
impl DataExportSettingProperties {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataExportSettings {
    #[serde(flatten)]
    pub setting: Setting,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DataExportSettingProperties>,
}
impl DataExportSettings {
    pub fn new(setting: Setting) -> Self {
        Self { setting, properties: None }
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
pub struct Setting {
    #[serde(flatten)]
    pub resource: Resource,
    pub kind: setting::Kind,
}
impl Setting {
    pub fn new(kind: setting::Kind) -> Self {
        Self {
            resource: Resource::default(),
            kind,
        }
    }
}
pub mod setting {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        DataExportSettings,
        AlertSuppressionSetting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SettingsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Setting>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SettingsList {
    pub fn new() -> Self {
        Self::default()
    }
}
