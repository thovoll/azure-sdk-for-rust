#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableContacts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spacecraft: Option<ResourceReference>,
    #[serde(rename = "groundStationName", default, skip_serializing_if = "Option::is_none")]
    pub ground_station_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContactInstanceProperties>,
}
impl AvailableContacts {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableContactsListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableContacts>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AvailableContactsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableGroundStation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AvailableGroundStationProperties>,
}
impl AvailableGroundStation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableGroundStationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<AvailableGroundStation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AvailableGroundStationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailableGroundStationProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "providerName", default, skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
    #[serde(rename = "longitudeDegrees", default, skip_serializing_if = "Option::is_none")]
    pub longitude_degrees: Option<f64>,
    #[serde(rename = "latitudeDegrees", default, skip_serializing_if = "Option::is_none")]
    pub latitude_degrees: Option<f64>,
    #[serde(rename = "altitudeMeters", default, skip_serializing_if = "Option::is_none")]
    pub altitude_meters: Option<f64>,
}
impl AvailableGroundStationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Capability {
    EarthObservation,
    Communication,
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
}
impl CloudErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Contact {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContactsProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Contact {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactInstanceProperties {
    #[serde(rename = "maximumElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub maximum_elevation_degrees: Option<f64>,
    #[serde(rename = "txStartTime", default, skip_serializing_if = "Option::is_none")]
    pub tx_start_time: Option<String>,
    #[serde(rename = "txEndTime", default, skip_serializing_if = "Option::is_none")]
    pub tx_end_time: Option<String>,
    #[serde(rename = "rxStartTime", default, skip_serializing_if = "Option::is_none")]
    pub rx_start_time: Option<String>,
    #[serde(rename = "rxEndTime", default, skip_serializing_if = "Option::is_none")]
    pub rx_end_time: Option<String>,
    #[serde(rename = "startAzimuthDegrees", default, skip_serializing_if = "Option::is_none")]
    pub start_azimuth_degrees: Option<f64>,
    #[serde(rename = "endAzimuthDegrees", default, skip_serializing_if = "Option::is_none")]
    pub end_azimuth_degrees: Option<f64>,
    #[serde(rename = "startElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub start_elevation_degrees: Option<f64>,
    #[serde(rename = "endElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub end_elevation_degrees: Option<f64>,
}
impl ContactInstanceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Contact>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ContactListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactParameters {
    #[serde(rename = "contactProfile")]
    pub contact_profile: ResourceReference,
    #[serde(rename = "groundStationName")]
    pub ground_station_name: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
}
impl ContactParameters {
    pub fn new(contact_profile: ResourceReference, ground_station_name: String, start_time: String, end_time: String) -> Self {
        Self {
            contact_profile,
            ground_station_name,
            start_time,
            end_time,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactProfile {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ContactProfilesProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl ContactProfile {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            etag: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactProfileLink {
    pub polarization: contact_profile_link::Polarization,
    pub direction: contact_profile_link::Direction,
    #[serde(rename = "gainOverTemperature", default, skip_serializing_if = "Option::is_none")]
    pub gain_over_temperature: Option<f64>,
    #[serde(rename = "eirpdBW", default, skip_serializing_if = "Option::is_none")]
    pub eirpd_bw: Option<f64>,
    pub channels: Vec<ContactProfileLinkChannel>,
}
impl ContactProfileLink {
    pub fn new(
        polarization: contact_profile_link::Polarization,
        direction: contact_profile_link::Direction,
        channels: Vec<ContactProfileLinkChannel>,
    ) -> Self {
        Self {
            polarization,
            direction,
            gain_over_temperature: None,
            eirpd_bw: None,
            channels,
        }
    }
}
pub mod contact_profile_link {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Polarization {
        #[serde(rename = "RHCP")]
        Rhcp,
        #[serde(rename = "LHCP")]
        Lhcp,
        #[serde(rename = "dualRhcpLhcp")]
        DualRhcpLhcp,
        #[serde(rename = "linearVertical")]
        LinearVertical,
        #[serde(rename = "linearHorizontal")]
        LinearHorizontal,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        #[serde(rename = "uplink")]
        Uplink,
        #[serde(rename = "downlink")]
        Downlink,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactProfileLinkChannel {
    #[serde(rename = "centerFrequencyMHz")]
    pub center_frequency_m_hz: f64,
    #[serde(rename = "bandwidthMHz")]
    pub bandwidth_m_hz: f64,
    #[serde(rename = "endPoint")]
    pub end_point: EndPoint,
    #[serde(rename = "modulationConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub modulation_configuration: Option<String>,
    #[serde(rename = "demodulationConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub demodulation_configuration: Option<String>,
    #[serde(rename = "encodingConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub encoding_configuration: Option<String>,
    #[serde(rename = "decodingConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub decoding_configuration: Option<String>,
}
impl ContactProfileLinkChannel {
    pub fn new(center_frequency_m_hz: f64, bandwidth_m_hz: f64, end_point: EndPoint) -> Self {
        Self {
            center_frequency_m_hz,
            bandwidth_m_hz,
            end_point,
            modulation_configuration: None,
            demodulation_configuration: None,
            encoding_configuration: None,
            decoding_configuration: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContactProfileListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ContactProfile>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ContactProfileListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactProfilesProperties {
    #[serde(rename = "minimumViableContactDuration", default, skip_serializing_if = "Option::is_none")]
    pub minimum_viable_contact_duration: Option<String>,
    #[serde(rename = "minimumElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub minimum_elevation_degrees: Option<f64>,
    #[serde(rename = "autoTrackingConfiguration", default, skip_serializing_if = "Option::is_none")]
    pub auto_tracking_configuration: Option<contact_profiles_properties::AutoTrackingConfiguration>,
    #[serde(rename = "eventHubUri", default, skip_serializing_if = "Option::is_none")]
    pub event_hub_uri: Option<String>,
    pub links: Vec<ContactProfileLink>,
}
impl ContactProfilesProperties {
    pub fn new(links: Vec<ContactProfileLink>) -> Self {
        Self {
            minimum_viable_contact_duration: None,
            minimum_elevation_degrees: None,
            auto_tracking_configuration: None,
            event_hub_uri: None,
            links,
        }
    }
}
pub mod contact_profiles_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AutoTrackingConfiguration {
        #[serde(rename = "disabled")]
        Disabled,
        #[serde(rename = "xBand")]
        XBand,
        #[serde(rename = "sBand")]
        SBand,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactsProperties {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<contacts_properties::Status>,
    #[serde(rename = "reservationStartTime")]
    pub reservation_start_time: String,
    #[serde(rename = "reservationEndTime")]
    pub reservation_end_time: String,
    #[serde(rename = "rxStartTime", default, skip_serializing_if = "Option::is_none")]
    pub rx_start_time: Option<String>,
    #[serde(rename = "rxEndTime", default, skip_serializing_if = "Option::is_none")]
    pub rx_end_time: Option<String>,
    #[serde(rename = "txStartTime", default, skip_serializing_if = "Option::is_none")]
    pub tx_start_time: Option<String>,
    #[serde(rename = "txEndTime", default, skip_serializing_if = "Option::is_none")]
    pub tx_end_time: Option<String>,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "maximumElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub maximum_elevation_degrees: Option<f64>,
    #[serde(rename = "startAzimuthDegrees", default, skip_serializing_if = "Option::is_none")]
    pub start_azimuth_degrees: Option<f64>,
    #[serde(rename = "endAzimuthDegrees", default, skip_serializing_if = "Option::is_none")]
    pub end_azimuth_degrees: Option<f64>,
    #[serde(rename = "groundStationName")]
    pub ground_station_name: String,
    #[serde(rename = "startElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub start_elevation_degrees: Option<f64>,
    #[serde(rename = "endElevationDegrees", default, skip_serializing_if = "Option::is_none")]
    pub end_elevation_degrees: Option<f64>,
    #[serde(rename = "contactProfile")]
    pub contact_profile: ResourceReference,
}
impl ContactsProperties {
    pub fn new(
        reservation_start_time: String,
        reservation_end_time: String,
        ground_station_name: String,
        contact_profile: ResourceReference,
    ) -> Self {
        Self {
            status: None,
            reservation_start_time,
            reservation_end_time,
            rx_start_time: None,
            rx_end_time: None,
            tx_start_time: None,
            tx_end_time: None,
            error_message: None,
            maximum_elevation_degrees: None,
            start_azimuth_degrees: None,
            end_azimuth_degrees: None,
            ground_station_name,
            start_elevation_degrees: None,
            end_elevation_degrees: None,
            contact_profile,
        }
    }
}
pub mod contacts_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        #[serde(rename = "scheduled")]
        Scheduled,
        #[serde(rename = "cancelled")]
        Cancelled,
        #[serde(rename = "succeeded")]
        Succeeded,
        #[serde(rename = "failed")]
        Failed,
        #[serde(rename = "providerCancelled")]
        ProviderCancelled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndPoint {
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
    #[serde(rename = "endPointName")]
    pub end_point_name: String,
    pub port: String,
    pub protocol: end_point::Protocol,
}
impl EndPoint {
    pub fn new(ip_address: String, end_point_name: String, port: String, protocol: end_point::Protocol) -> Self {
        Self {
            ip_address,
            end_point_name,
            port,
            protocol,
        }
    }
}
pub mod end_point {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
    }
}
pub type Etag = String;
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
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<SystemData>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceIdListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<serde_json::Value>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl ResourceIdListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ResourceReference {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Spacecraft {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SpacecraftsProperties>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl Spacecraft {
    pub fn new(tracked_resource: TrackedResource) -> Self {
        Self {
            tracked_resource,
            properties: None,
            etag: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpacecraftLink {
    #[serde(rename = "centerFrequencyMHz")]
    pub center_frequency_m_hz: f64,
    #[serde(rename = "bandwidthMHz")]
    pub bandwidth_m_hz: f64,
    pub direction: spacecraft_link::Direction,
    pub polarization: spacecraft_link::Polarization,
}
impl SpacecraftLink {
    pub fn new(
        center_frequency_m_hz: f64,
        bandwidth_m_hz: f64,
        direction: spacecraft_link::Direction,
        polarization: spacecraft_link::Polarization,
    ) -> Self {
        Self {
            center_frequency_m_hz,
            bandwidth_m_hz,
            direction,
            polarization,
        }
    }
}
pub mod spacecraft_link {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Direction {
        #[serde(rename = "uplink")]
        Uplink,
        #[serde(rename = "downlink")]
        Downlink,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Polarization {
        #[serde(rename = "RHCP")]
        Rhcp,
        #[serde(rename = "LHCP")]
        Lhcp,
        #[serde(rename = "dualRhcpLhcp")]
        DualRhcpLhcp,
        #[serde(rename = "linearVertical")]
        LinearVertical,
        #[serde(rename = "linearHorizontal")]
        LinearHorizontal,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SpacecraftListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Spacecraft>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl SpacecraftListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpacecraftsProperties {
    #[serde(rename = "noradId")]
    pub norad_id: String,
    #[serde(rename = "authorizationStatus", default, skip_serializing_if = "Option::is_none")]
    pub authorization_status: Option<spacecrafts_properties::AuthorizationStatus>,
    #[serde(rename = "authorizationStatusExtended", default, skip_serializing_if = "Option::is_none")]
    pub authorization_status_extended: Option<String>,
    #[serde(rename = "titleLine", default, skip_serializing_if = "Option::is_none")]
    pub title_line: Option<String>,
    #[serde(rename = "tleLine1", default, skip_serializing_if = "Option::is_none")]
    pub tle_line1: Option<String>,
    #[serde(rename = "tleLine2", default, skip_serializing_if = "Option::is_none")]
    pub tle_line2: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<SpacecraftLink>,
}
impl SpacecraftsProperties {
    pub fn new(norad_id: String) -> Self {
        Self {
            norad_id,
            authorization_status: None,
            authorization_status_extended: None,
            title_line: None,
            tle_line1: None,
            tle_line2: None,
            links: Vec::new(),
        }
    }
}
pub mod spacecrafts_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum AuthorizationStatus {
        Allowed,
        Pending,
        Denied,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TagsObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
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
