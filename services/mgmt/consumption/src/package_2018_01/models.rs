#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Budget {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BudgetProperties>,
}
impl Budget {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetProperties {
    pub category: budget_properties::Category,
    pub amount: f64,
    #[serde(rename = "timeGrain")]
    pub time_grain: budget_properties::TimeGrain,
    #[serde(rename = "timePeriod")]
    pub time_period: BudgetTimePeriod,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Filters>,
    #[serde(rename = "currentSpend", default, skip_serializing_if = "Option::is_none")]
    pub current_spend: Option<CurrentSpend>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<serde_json::Value>,
}
impl BudgetProperties {
    pub fn new(
        category: budget_properties::Category,
        amount: f64,
        time_grain: budget_properties::TimeGrain,
        time_period: BudgetTimePeriod,
    ) -> Self {
        Self {
            category,
            amount,
            time_grain,
            time_period,
            filters: None,
            current_spend: None,
            notifications: None,
        }
    }
}
pub mod budget_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Category {
        Cost,
        Usage,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum TimeGrain {
        Monthly,
        Quarterly,
        Annually,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BudgetTimePeriod {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}
impl BudgetTimePeriod {
    pub fn new(start_date: String) -> Self {
        Self {
            start_date,
            end_date: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BudgetsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Budget>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl BudgetsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CurrentSpend {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
impl CurrentSpend {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Filters {
    #[serde(rename = "resourceGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_groups: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub meters: Vec<String>,
}
impl Filters {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Marketplace {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MarketplaceProperties>,
}
impl Marketplace {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplaceProperties {
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[serde(rename = "resourceRate", default, skip_serializing_if = "Option::is_none")]
    pub resource_rate: Option<f64>,
    #[serde(rename = "offerName", default, skip_serializing_if = "Option::is_none")]
    pub offer_name: Option<String>,
    #[serde(rename = "resourceGroup", default, skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "orderNumber", default, skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "consumedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub consumed_quantity: Option<f64>,
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    #[serde(rename = "pretaxCost", default, skip_serializing_if = "Option::is_none")]
    pub pretax_cost: Option<f64>,
    #[serde(rename = "isEstimated", default, skip_serializing_if = "Option::is_none")]
    pub is_estimated: Option<bool>,
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "subscriptionGuid", default, skip_serializing_if = "Option::is_none")]
    pub subscription_guid: Option<String>,
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "departmentName", default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[serde(rename = "consumedService", default, skip_serializing_if = "Option::is_none")]
    pub consumed_service: Option<String>,
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<String>,
    #[serde(rename = "publisherName", default, skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[serde(rename = "planName", default, skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
}
impl MarketplaceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MarketplacesListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Marketplace>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl MarketplacesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MeterDetails {
    #[serde(rename = "meterName", default, skip_serializing_if = "Option::is_none")]
    pub meter_name: Option<String>,
    #[serde(rename = "meterCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_category: Option<String>,
    #[serde(rename = "meterSubCategory", default, skip_serializing_if = "Option::is_none")]
    pub meter_sub_category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "meterLocation", default, skip_serializing_if = "Option::is_none")]
    pub meter_location: Option<String>,
    #[serde(rename = "totalIncludedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub total_included_quantity: Option<f64>,
    #[serde(rename = "pretaxStandardRate", default, skip_serializing_if = "Option::is_none")]
    pub pretax_standard_rate: Option<f64>,
}
impl MeterDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Notification {
    pub enabled: bool,
    pub operator: notification::Operator,
    pub threshold: f64,
    #[serde(rename = "contactEmails")]
    pub contact_emails: Vec<String>,
    #[serde(rename = "contactRoles", default, skip_serializing_if = "Vec::is_empty")]
    pub contact_roles: Vec<String>,
    #[serde(rename = "contactGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub contact_groups: Vec<String>,
}
impl Notification {
    pub fn new(enabled: bool, operator: notification::Operator, threshold: f64, contact_emails: Vec<String>) -> Self {
        Self {
            enabled,
            operator,
            threshold,
            contact_emails,
            contact_roles: Vec::new(),
            contact_groups: Vec::new(),
        }
    }
}
pub mod notification {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Operator {
        EqualTo,
        GreaterThan,
        GreaterThanOrEqualTo,
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
pub struct PriceSheetModel {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pricesheets: Vec<PriceSheetProperties>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl PriceSheetModel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PriceSheetProperties {
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "meterDetails", default, skip_serializing_if = "Option::is_none")]
    pub meter_details: Option<MeterDetails>,
    #[serde(rename = "unitOfMeasure", default, skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<String>,
    #[serde(rename = "includedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub included_quantity: Option<f64>,
    #[serde(rename = "partNumber", default, skip_serializing_if = "Option::is_none")]
    pub part_number: Option<String>,
    #[serde(rename = "unitPrice", default, skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<f64>,
    #[serde(rename = "currencyCode", default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}
impl PriceSheetProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PriceSheetResult {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PriceSheetModel>,
}
impl PriceSheetResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ProxyResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "eTag", default, skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
}
impl ProxyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationDetails {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationDetailsProperties>,
}
impl ReservationDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationDetailsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationDetails>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ReservationDetailsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationDetailsProperties {
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[serde(rename = "reservedHours", default, skip_serializing_if = "Option::is_none")]
    pub reserved_hours: Option<f64>,
    #[serde(rename = "usageDate", default, skip_serializing_if = "Option::is_none")]
    pub usage_date: Option<String>,
    #[serde(rename = "usedHours", default, skip_serializing_if = "Option::is_none")]
    pub used_hours: Option<f64>,
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "totalReservedQuantity", default, skip_serializing_if = "Option::is_none")]
    pub total_reserved_quantity: Option<f64>,
}
impl ReservationDetailsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSummaries {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationSummariesProperties>,
}
impl ReservationSummaries {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSummariesListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationSummaries>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ReservationSummariesListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSummariesProperties {
    #[serde(rename = "reservationOrderId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_id: Option<String>,
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    #[serde(rename = "skuName", default, skip_serializing_if = "Option::is_none")]
    pub sku_name: Option<String>,
    #[serde(rename = "reservedHours", default, skip_serializing_if = "Option::is_none")]
    pub reserved_hours: Option<f64>,
    #[serde(rename = "usageDate", default, skip_serializing_if = "Option::is_none")]
    pub usage_date: Option<String>,
    #[serde(rename = "usedHours", default, skip_serializing_if = "Option::is_none")]
    pub used_hours: Option<f64>,
    #[serde(rename = "minUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub min_utilization_percentage: Option<f64>,
    #[serde(rename = "avgUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub avg_utilization_percentage: Option<f64>,
    #[serde(rename = "maxUtilizationPercentage", default, skip_serializing_if = "Option::is_none")]
    pub max_utilization_percentage: Option<f64>,
}
impl ReservationSummariesProperties {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageDetail {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UsageDetailProperties>,
}
impl UsageDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageDetailProperties {
    #[serde(rename = "billingPeriodId", default, skip_serializing_if = "Option::is_none")]
    pub billing_period_id: Option<String>,
    #[serde(rename = "invoiceId", default, skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[serde(rename = "usageStart", default, skip_serializing_if = "Option::is_none")]
    pub usage_start: Option<String>,
    #[serde(rename = "usageEnd", default, skip_serializing_if = "Option::is_none")]
    pub usage_end: Option<String>,
    #[serde(rename = "instanceName", default, skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(rename = "instanceId", default, skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "instanceLocation", default, skip_serializing_if = "Option::is_none")]
    pub instance_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "usageQuantity", default, skip_serializing_if = "Option::is_none")]
    pub usage_quantity: Option<f64>,
    #[serde(rename = "billableQuantity", default, skip_serializing_if = "Option::is_none")]
    pub billable_quantity: Option<f64>,
    #[serde(rename = "pretaxCost", default, skip_serializing_if = "Option::is_none")]
    pub pretax_cost: Option<f64>,
    #[serde(rename = "isEstimated", default, skip_serializing_if = "Option::is_none")]
    pub is_estimated: Option<bool>,
    #[serde(rename = "meterId", default, skip_serializing_if = "Option::is_none")]
    pub meter_id: Option<String>,
    #[serde(rename = "meterDetails", default, skip_serializing_if = "Option::is_none")]
    pub meter_details: Option<MeterDetails>,
    #[serde(rename = "subscriptionGuid", default, skip_serializing_if = "Option::is_none")]
    pub subscription_guid: Option<String>,
    #[serde(rename = "subscriptionName", default, skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
    #[serde(rename = "accountName", default, skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "departmentName", default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "consumedService", default, skip_serializing_if = "Option::is_none")]
    pub consumed_service: Option<String>,
    #[serde(rename = "costCenter", default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<String>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<String>,
}
impl UsageDetailProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageDetailsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<UsageDetail>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl UsageDetailsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
