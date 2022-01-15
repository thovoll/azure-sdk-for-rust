#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[serde(rename = "groupShortName")]
    pub group_short_name: String,
    pub enabled: bool,
    #[serde(rename = "emailReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub email_receivers: Vec<EmailReceiver>,
    #[serde(rename = "smsReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub sms_receivers: Vec<SmsReceiver>,
    #[serde(rename = "webhookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub webhook_receivers: Vec<WebhookReceiver>,
    #[serde(rename = "itsmReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub itsm_receivers: Vec<ItsmReceiver>,
    #[serde(rename = "azureAppPushReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_app_push_receivers: Vec<AzureAppPushReceiver>,
    #[serde(rename = "automationRunbookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub automation_runbook_receivers: Vec<AutomationRunbookReceiver>,
}
impl ActionGroup {
    pub fn new(group_short_name: String, enabled: bool) -> Self {
        Self {
            group_short_name,
            enabled,
            email_receivers: Vec::new(),
            sms_receivers: Vec::new(),
            webhook_receivers: Vec::new(),
            itsm_receivers: Vec::new(),
            azure_app_push_receivers: Vec::new(),
            automation_runbook_receivers: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActionGroupResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ActionGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ActionGroupPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupPatchBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroupPatch>,
}
impl ActionGroupPatchBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroup>,
}
impl ActionGroupResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlert {
    pub scopes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    pub condition: ActivityLogAlertAllOfCondition,
    pub actions: ActivityLogAlertActionList,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl ActivityLogAlert {
    pub fn new(scopes: Vec<String>, condition: ActivityLogAlertAllOfCondition, actions: ActivityLogAlertActionList) -> Self {
        Self {
            scopes,
            enabled: None,
            condition,
            actions,
            description: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertActionGroup {
    #[serde(rename = "actionGroupId")]
    pub action_group_id: String,
    #[serde(rename = "webhookProperties", default, skip_serializing_if = "Option::is_none")]
    pub webhook_properties: Option<serde_json::Value>,
}
impl ActivityLogAlertActionGroup {
    pub fn new(action_group_id: String) -> Self {
        Self {
            action_group_id,
            webhook_properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityLogAlertActionList {
    #[serde(rename = "actionGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub action_groups: Vec<ActivityLogAlertActionGroup>,
}
impl ActivityLogAlertActionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertAllOfCondition {
    #[serde(rename = "allOf")]
    pub all_of: Vec<ActivityLogAlertLeafCondition>,
}
impl ActivityLogAlertAllOfCondition {
    pub fn new(all_of: Vec<ActivityLogAlertLeafCondition>) -> Self {
        Self { all_of }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertLeafCondition {
    pub field: String,
    pub equals: String,
}
impl ActivityLogAlertLeafCondition {
    pub fn new(field: String, equals: String) -> Self {
        Self { field, equals }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityLogAlertList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActivityLogAlertResource>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ActivityLogAlertList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityLogAlertPatch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ActivityLogAlertPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityLogAlertPatchBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActivityLogAlertPatch>,
}
impl ActivityLogAlertPatchBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActivityLogAlertResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActivityLogAlert>,
}
impl ActivityLogAlertResource {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRunbookReceiver {
    #[serde(rename = "automationAccountId")]
    pub automation_account_id: String,
    #[serde(rename = "runbookName")]
    pub runbook_name: String,
    #[serde(rename = "webhookResourceId")]
    pub webhook_resource_id: String,
    #[serde(rename = "isGlobalRunbook")]
    pub is_global_runbook: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
}
impl AutomationRunbookReceiver {
    pub fn new(automation_account_id: String, runbook_name: String, webhook_resource_id: String, is_global_runbook: bool) -> Self {
        Self {
            automation_account_id,
            runbook_name,
            webhook_resource_id,
            is_global_runbook,
            name: None,
            service_uri: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAppPushReceiver {
    pub name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}
impl AzureAppPushReceiver {
    pub fn new(name: String, email_address: String) -> Self {
        Self { name, email_address }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailReceiver {
    pub name: String,
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
impl EmailReceiver {
    pub fn new(name: String, email_address: String) -> Self {
        Self {
            name,
            email_address,
            status: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableRequest {
    #[serde(rename = "receiverName")]
    pub receiver_name: String,
}
impl EnableRequest {
    pub fn new(receiver_name: String) -> Self {
        Self { receiver_name }
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
pub struct ItsmReceiver {
    pub name: String,
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[serde(rename = "ticketConfiguration")]
    pub ticket_configuration: String,
    pub region: String,
}
impl ItsmReceiver {
    pub fn new(name: String, workspace_id: String, connection_id: String, ticket_configuration: String, region: String) -> Self {
        Self {
            name,
            workspace_id,
            connection_id,
            ticket_configuration,
            region,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReceiverStatus {
    NotSpecified,
    Enabled,
    Disabled,
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
pub struct SmsReceiver {
    pub name: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
impl SmsReceiver {
    pub fn new(name: String, country_code: String, phone_number: String) -> Self {
        Self {
            name,
            country_code,
            phone_number,
            status: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookReceiver {
    pub name: String,
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
}
impl WebhookReceiver {
    pub fn new(name: String, service_uri: String) -> Self {
        Self { name, service_uri }
    }
}
