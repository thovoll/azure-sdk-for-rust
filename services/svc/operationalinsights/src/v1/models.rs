#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Column {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Column {
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<String>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}
impl ErrorDetail {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            target: None,
            value: None,
            resources: Vec::new(),
            additional_properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<ErrorInfo>>,
    #[serde(rename = "additionalProperties", default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<serde_json::Value>,
}
impl ErrorInfo {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            details: Vec::new(),
            innererror: Box::new(None),
            additional_properties: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorInfo,
}
impl ErrorResponse {
    pub fn new(error: ErrorInfo) -> Self {
        Self { error }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataApplication {
    pub id: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub name: String,
    pub region: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_application::Related>,
}
impl MetadataApplication {
    pub fn new(id: String, resource_id: String, name: String, region: String) -> Self {
        Self {
            id,
            resource_id,
            name,
            region,
            related: None,
        }
    }
}
pub mod metadata_application {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataCategory {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_category::Related>,
}
impl MetadataCategory {
    pub fn new(id: String, display_name: String) -> Self {
        Self {
            id,
            display_name,
            description: None,
            related: None,
        }
    }
}
pub mod metadata_category {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
        #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub queries: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub solutions: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataFunction {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_function::Related>,
}
impl MetadataFunction {
    pub fn new(id: String, name: String, body: String) -> Self {
        Self {
            id,
            name,
            parameters: None,
            display_name: None,
            description: None,
            body,
            tags: None,
            properties: None,
            related: None,
        }
    }
}
pub mod metadata_function {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub solutions: Vec<String>,
        #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub categories: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub workspaces: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataPermissions {
    pub workspaces: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applications: Vec<serde_json::Value>,
}
impl MetadataPermissions {
    pub fn new(workspaces: Vec<serde_json::Value>) -> Self {
        Self {
            workspaces,
            resources: Vec::new(),
            applications: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataQuery {
    pub id: String,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub body: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_query::Related>,
}
impl MetadataQuery {
    pub fn new(id: String, body: String) -> Self {
        Self {
            id,
            display_name: None,
            description: None,
            body,
            labels: Vec::new(),
            tags: None,
            properties: None,
            related: None,
        }
    }
}
pub mod metadata_query {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub categories: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub solutions: Vec<String>,
        #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataResource {}
impl MetadataResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataResourceType {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_resource_type::Related>,
}
impl MetadataResourceType {
    pub fn new(id: String, type_: String) -> Self {
        Self {
            id,
            type_,
            display_name: None,
            description: None,
            labels: Vec::new(),
            tags: None,
            properties: None,
            related: None,
        }
    }
}
pub mod metadata_resource_type {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub categories: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub queries: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub workspaces: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub resources: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataResults {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<MetadataCategory>,
    #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<MetadataResourceType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub solutions: Vec<MetadataSolution>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<MetadataTable>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub functions: Vec<MetadataFunction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub queries: Vec<MetadataQuery>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applications: Vec<MetadataApplication>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub workspaces: Vec<MetadataWorkspace>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<MetadataResource>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<MetadataPermissions>,
}
impl MetadataResults {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataSolution {
    pub id: String,
    pub name: String,
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    pub related: metadata_solution::Related,
}
impl MetadataSolution {
    pub fn new(id: String, name: String, related: metadata_solution::Related) -> Self {
        Self {
            id,
            name,
            display_name: None,
            description: None,
            tags: None,
            properties: None,
            related,
        }
    }
}
pub mod metadata_solution {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Related {
        pub tables: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub categories: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub queries: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub workspaces: Vec<String>,
    }
    impl Related {
        pub fn new(tables: Vec<String>) -> Self {
            Self {
                tables,
                functions: Vec::new(),
                categories: Vec::new(),
                queries: Vec::new(),
                workspaces: Vec::new(),
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataTable {
    pub id: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "timespanColumn", default, skip_serializing_if = "Option::is_none")]
    pub timespan_column: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub columns: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_table::Related>,
}
impl MetadataTable {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            description: None,
            timespan_column: None,
            labels: Vec::new(),
            tags: None,
            properties: None,
            columns: Vec::new(),
            related: None,
        }
    }
}
pub mod metadata_table {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub categories: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub solutions: Vec<String>,
        #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub workspaces: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub queries: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MetadataWorkspace {
    pub id: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub name: String,
    pub region: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<metadata_workspace::Related>,
}
impl MetadataWorkspace {
    pub fn new(id: String, resource_id: String, name: String, region: String) -> Self {
        Self {
            id,
            resource_id,
            name,
            region,
            related: None,
        }
    }
}
pub mod metadata_workspace {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Related {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub tables: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub solutions: Vec<String>,
        #[serde(rename = "resourceTypes", default, skip_serializing_if = "Vec::is_empty")]
        pub resource_types: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub functions: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub resources: Vec<String>,
    }
    impl Related {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryBody {
    pub query: QueryParam,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timespan: Option<TimespanParam>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspaces: Option<WorkspacesParam>,
}
impl QueryBody {
    pub fn new(query: QueryParam) -> Self {
        Self {
            query,
            timespan: None,
            workspaces: None,
        }
    }
}
pub type QueryParam = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryResults {
    pub tables: Vec<Table>,
}
impl QueryResults {
    pub fn new(tables: Vec<Table>) -> Self {
        Self { tables }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
    pub rows: serde_json::Value,
}
impl Table {
    pub fn new(name: String, columns: Vec<Column>, rows: serde_json::Value) -> Self {
        Self { name, columns, rows }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Tags {}
impl Tags {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type TimespanParam = String;
pub type WorkspacesParam = Vec<String>;
