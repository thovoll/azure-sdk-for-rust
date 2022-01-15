#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BaseJobParameters {
    #[serde(rename = "type")]
    pub type_: base_job_parameters::Type,
    pub properties: CreateJobProperties,
}
impl BaseJobParameters {
    pub fn new(type_: base_job_parameters::Type, properties: CreateJobProperties) -> Self {
        Self { type_, properties }
    }
}
pub mod base_job_parameters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        USql,
        Hive,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildJobParameters {
    #[serde(flatten)]
    pub base_job_parameters: BaseJobParameters,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl BuildJobParameters {
    pub fn new(base_job_parameters: BaseJobParameters) -> Self {
        Self {
            base_job_parameters,
            name: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateJobParameters {
    #[serde(flatten)]
    pub base_job_parameters: BaseJobParameters,
    pub name: String,
    #[serde(rename = "degreeOfParallelism", default, skip_serializing_if = "Option::is_none")]
    pub degree_of_parallelism: Option<i32>,
    #[serde(rename = "degreeOfParallelismPercent", default, skip_serializing_if = "Option::is_none")]
    pub degree_of_parallelism_percent: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "logFilePatterns", default, skip_serializing_if = "Vec::is_empty")]
    pub log_file_patterns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<JobRelationshipProperties>,
}
impl CreateJobParameters {
    pub fn new(base_job_parameters: BaseJobParameters, name: String) -> Self {
        Self {
            base_job_parameters,
            name,
            degree_of_parallelism: None,
            degree_of_parallelism_percent: None,
            priority: None,
            log_file_patterns: Vec::new(),
            related: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateJobProperties {
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    pub script: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl CreateJobProperties {
    pub fn new(script: String, type_: String) -> Self {
        Self {
            runtime_version: None,
            script,
            type_,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUSqlJobProperties {
    #[serde(flatten)]
    pub create_job_properties: CreateJobProperties,
    #[serde(rename = "compileMode", default, skip_serializing_if = "Option::is_none")]
    pub compile_mode: Option<create_u_sql_job_properties::CompileMode>,
}
impl CreateUSqlJobProperties {
    pub fn new(create_job_properties: CreateJobProperties) -> Self {
        Self {
            create_job_properties,
            compile_mode: None,
        }
    }
}
pub mod create_u_sql_job_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CompileMode {
        Semantic,
        Full,
        SingleBox,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Diagnostics {
    #[serde(rename = "columnNumber", default, skip_serializing_if = "Option::is_none")]
    pub column_number: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
    #[serde(rename = "lineNumber", default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<diagnostics::Severity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
impl Diagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod diagnostics {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Warning,
        Error,
        Info,
        SevereWarning,
        Deprecated,
        UserWarning,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HiveJobProperties {
    #[serde(flatten)]
    pub job_properties: JobProperties,
    #[serde(rename = "logsLocation", default, skip_serializing_if = "Option::is_none")]
    pub logs_location: Option<String>,
    #[serde(rename = "outputLocation", default, skip_serializing_if = "Option::is_none")]
    pub output_location: Option<String>,
    #[serde(rename = "statementCount", default, skip_serializing_if = "Option::is_none")]
    pub statement_count: Option<i32>,
    #[serde(rename = "executedStatementCount", default, skip_serializing_if = "Option::is_none")]
    pub executed_statement_count: Option<i32>,
}
impl HiveJobProperties {
    pub fn new(job_properties: JobProperties) -> Self {
        Self {
            job_properties,
            logs_location: None,
            output_location: None,
            statement_count: None,
            executed_statement_count: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobDataPath {
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<String>,
}
impl JobDataPath {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobErrorDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "endOffset", default, skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i32>,
    #[serde(rename = "errorId", default, skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
    #[serde(rename = "filePath", default, skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "helpLink", default, skip_serializing_if = "Option::is_none")]
    pub help_link: Option<String>,
    #[serde(rename = "internalDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub internal_diagnostics: Option<String>,
    #[serde(rename = "lineNumber", default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Option<JobInnerError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<job_error_details::Severity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "startOffset", default, skip_serializing_if = "Option::is_none")]
    pub start_offset: Option<i32>,
}
impl JobErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_error_details {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Warning,
        Error,
        Info,
        SevereWarning,
        Deprecated,
        UserWarning,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobInfoListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobInformationBasic>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl JobInfoListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobInformation {
    #[serde(flatten)]
    pub job_information_basic: JobInformationBasic,
    #[serde(rename = "errorMessage", default, skip_serializing_if = "Vec::is_empty")]
    pub error_message: Vec<JobErrorDetails>,
    #[serde(rename = "stateAuditRecords", default, skip_serializing_if = "Vec::is_empty")]
    pub state_audit_records: Vec<JobStateAuditRecord>,
    pub properties: JobProperties,
}
impl JobInformation {
    pub fn new(job_information_basic: JobInformationBasic, properties: JobProperties) -> Self {
        Self {
            job_information_basic,
            error_message: Vec::new(),
            state_audit_records: Vec::new(),
            properties,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobInformationBasic {
    #[serde(rename = "jobId", default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: job_information_basic::Type,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitter: Option<String>,
    #[serde(rename = "degreeOfParallelism", default, skip_serializing_if = "Option::is_none")]
    pub degree_of_parallelism: Option<i32>,
    #[serde(rename = "degreeOfParallelismPercent", default, skip_serializing_if = "Option::is_none")]
    pub degree_of_parallelism_percent: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "submitTime", default, skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<String>,
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<job_information_basic::State>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<job_information_basic::Result>,
    #[serde(rename = "logFolder", default, skip_serializing_if = "Option::is_none")]
    pub log_folder: Option<String>,
    #[serde(rename = "logFilePatterns", default, skip_serializing_if = "Vec::is_empty")]
    pub log_file_patterns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<JobRelationshipProperties>,
    #[serde(rename = "hierarchyQueueNode", default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_queue_node: Option<String>,
}
impl JobInformationBasic {
    pub fn new(name: String, type_: job_information_basic::Type) -> Self {
        Self {
            job_id: None,
            name,
            type_,
            submitter: None,
            degree_of_parallelism: None,
            degree_of_parallelism_percent: None,
            priority: None,
            submit_time: None,
            start_time: None,
            end_time: None,
            state: None,
            result: None,
            log_folder: None,
            log_file_patterns: Vec::new(),
            related: None,
            hierarchy_queue_node: None,
        }
    }
}
pub mod job_information_basic {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        USql,
        Hive,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum State {
        Accepted,
        Compiling,
        Ended,
        New,
        Queued,
        Running,
        Scheduling,
        Starting,
        Paused,
        WaitingForCapacity,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Result {
        None,
        Succeeded,
        Cancelled,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobInnerError {
    #[serde(rename = "diagnosticCode", default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<job_inner_error::Severity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(rename = "errorId", default, skip_serializing_if = "Option::is_none")]
    pub error_id: Option<String>,
    #[serde(rename = "helpLink", default, skip_serializing_if = "Option::is_none")]
    pub help_link: Option<String>,
    #[serde(rename = "internalDiagnostics", default, skip_serializing_if = "Option::is_none")]
    pub internal_diagnostics: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "innerError", default, skip_serializing_if = "Option::is_none")]
    pub inner_error: Box<Option<JobInnerError>>,
}
impl JobInnerError {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_inner_error {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Severity {
        Warning,
        Error,
        Info,
        SevereWarning,
        Deprecated,
        UserWarning,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobPipelineInformation {
    #[serde(rename = "pipelineId", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    #[serde(rename = "pipelineName", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    #[serde(rename = "pipelineUri", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_uri: Option<String>,
    #[serde(rename = "numJobsFailed", default, skip_serializing_if = "Option::is_none")]
    pub num_jobs_failed: Option<i32>,
    #[serde(rename = "numJobsCanceled", default, skip_serializing_if = "Option::is_none")]
    pub num_jobs_canceled: Option<i32>,
    #[serde(rename = "numJobsSucceeded", default, skip_serializing_if = "Option::is_none")]
    pub num_jobs_succeeded: Option<i32>,
    #[serde(rename = "auHoursFailed", default, skip_serializing_if = "Option::is_none")]
    pub au_hours_failed: Option<f64>,
    #[serde(rename = "auHoursCanceled", default, skip_serializing_if = "Option::is_none")]
    pub au_hours_canceled: Option<f64>,
    #[serde(rename = "auHoursSucceeded", default, skip_serializing_if = "Option::is_none")]
    pub au_hours_succeeded: Option<f64>,
    #[serde(rename = "lastSubmitTime", default, skip_serializing_if = "Option::is_none")]
    pub last_submit_time: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub runs: Vec<JobPipelineRunInformation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recurrences: Vec<String>,
}
impl JobPipelineInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobPipelineInformationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobPipelineInformation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl JobPipelineInformationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobPipelineRunInformation {
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "lastSubmitTime", default, skip_serializing_if = "Option::is_none")]
    pub last_submit_time: Option<String>,
}
impl JobPipelineRunInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobProperties {
    #[serde(rename = "runtimeVersion", default, skip_serializing_if = "Option::is_none")]
    pub runtime_version: Option<String>,
    pub script: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl JobProperties {
    pub fn new(script: String, type_: String) -> Self {
        Self {
            runtime_version: None,
            script,
            type_,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobRecurrenceInformation {
    #[serde(rename = "recurrenceId", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_id: Option<String>,
    #[serde(rename = "recurrenceName", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_name: Option<String>,
    #[serde(rename = "numJobsFailed", default, skip_serializing_if = "Option::is_none")]
    pub num_jobs_failed: Option<i32>,
    #[serde(rename = "numJobsCanceled", default, skip_serializing_if = "Option::is_none")]
    pub num_jobs_canceled: Option<i32>,
    #[serde(rename = "numJobsSucceeded", default, skip_serializing_if = "Option::is_none")]
    pub num_jobs_succeeded: Option<i32>,
    #[serde(rename = "auHoursFailed", default, skip_serializing_if = "Option::is_none")]
    pub au_hours_failed: Option<f64>,
    #[serde(rename = "auHoursCanceled", default, skip_serializing_if = "Option::is_none")]
    pub au_hours_canceled: Option<f64>,
    #[serde(rename = "auHoursSucceeded", default, skip_serializing_if = "Option::is_none")]
    pub au_hours_succeeded: Option<f64>,
    #[serde(rename = "lastSubmitTime", default, skip_serializing_if = "Option::is_none")]
    pub last_submit_time: Option<String>,
}
impl JobRecurrenceInformation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobRecurrenceInformationListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<JobRecurrenceInformation>,
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl JobRecurrenceInformationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JobRelationshipProperties {
    #[serde(rename = "pipelineId", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
    #[serde(rename = "pipelineName", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    #[serde(rename = "pipelineUri", default, skip_serializing_if = "Option::is_none")]
    pub pipeline_uri: Option<String>,
    #[serde(rename = "runId", default, skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "recurrenceId")]
    pub recurrence_id: String,
    #[serde(rename = "recurrenceName", default, skip_serializing_if = "Option::is_none")]
    pub recurrence_name: Option<String>,
}
impl JobRelationshipProperties {
    pub fn new(recurrence_id: String) -> Self {
        Self {
            pipeline_id: None,
            pipeline_name: None,
            pipeline_uri: None,
            run_id: None,
            recurrence_id,
            recurrence_name: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resourcePath", default, skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<job_resource::Type>,
}
impl JobResource {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod job_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        VertexResource,
        JobManagerResource,
        StatisticsResource,
        VertexResourceInUserFolder,
        JobManagerResourceInUserFolder,
        StatisticsResourceInUserFolder,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStateAuditRecord {
    #[serde(rename = "newState", default, skip_serializing_if = "Option::is_none")]
    pub new_state: Option<String>,
    #[serde(rename = "timeStamp", default, skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<String>,
    #[serde(rename = "requestedByUser", default, skip_serializing_if = "Option::is_none")]
    pub requested_by_user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}
impl JobStateAuditRecord {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStatistics {
    #[serde(rename = "lastUpdateTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time_utc: Option<String>,
    #[serde(rename = "finalizingTimeUtc", default, skip_serializing_if = "Option::is_none")]
    pub finalizing_time_utc: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stages: Vec<JobStatisticsVertexStage>,
}
impl JobStatistics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct JobStatisticsVertexStage {
    #[serde(rename = "dataRead", default, skip_serializing_if = "Option::is_none")]
    pub data_read: Option<i64>,
    #[serde(rename = "dataReadCrossPod", default, skip_serializing_if = "Option::is_none")]
    pub data_read_cross_pod: Option<i64>,
    #[serde(rename = "dataReadIntraPod", default, skip_serializing_if = "Option::is_none")]
    pub data_read_intra_pod: Option<i64>,
    #[serde(rename = "dataToRead", default, skip_serializing_if = "Option::is_none")]
    pub data_to_read: Option<i64>,
    #[serde(rename = "dataWritten", default, skip_serializing_if = "Option::is_none")]
    pub data_written: Option<i64>,
    #[serde(rename = "duplicateDiscardCount", default, skip_serializing_if = "Option::is_none")]
    pub duplicate_discard_count: Option<i32>,
    #[serde(rename = "failedCount", default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i32>,
    #[serde(rename = "maxVertexDataRead", default, skip_serializing_if = "Option::is_none")]
    pub max_vertex_data_read: Option<i64>,
    #[serde(rename = "minVertexDataRead", default, skip_serializing_if = "Option::is_none")]
    pub min_vertex_data_read: Option<i64>,
    #[serde(rename = "readFailureCount", default, skip_serializing_if = "Option::is_none")]
    pub read_failure_count: Option<i32>,
    #[serde(rename = "revocationCount", default, skip_serializing_if = "Option::is_none")]
    pub revocation_count: Option<i32>,
    #[serde(rename = "runningCount", default, skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i32>,
    #[serde(rename = "scheduledCount", default, skip_serializing_if = "Option::is_none")]
    pub scheduled_count: Option<i32>,
    #[serde(rename = "stageName", default, skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(rename = "succeededCount", default, skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<i32>,
    #[serde(rename = "tempDataWritten", default, skip_serializing_if = "Option::is_none")]
    pub temp_data_written: Option<i64>,
    #[serde(rename = "totalCount", default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "totalFailedTime", default, skip_serializing_if = "Option::is_none")]
    pub total_failed_time: Option<String>,
    #[serde(rename = "totalProgress", default, skip_serializing_if = "Option::is_none")]
    pub total_progress: Option<i32>,
    #[serde(rename = "totalSucceededTime", default, skip_serializing_if = "Option::is_none")]
    pub total_succeeded_time: Option<String>,
}
impl JobStatisticsVertexStage {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct USqlJobProperties {
    #[serde(flatten)]
    pub job_properties: JobProperties,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<JobResource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statistics: Option<JobStatistics>,
    #[serde(rename = "debugData", default, skip_serializing_if = "Option::is_none")]
    pub debug_data: Option<JobDataPath>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostics>,
    #[serde(rename = "algebraFilePath", default, skip_serializing_if = "Option::is_none")]
    pub algebra_file_path: Option<String>,
    #[serde(rename = "totalCompilationTime", default, skip_serializing_if = "Option::is_none")]
    pub total_compilation_time: Option<String>,
    #[serde(rename = "totalPauseTime", default, skip_serializing_if = "Option::is_none")]
    pub total_pause_time: Option<String>,
    #[serde(rename = "totalQueuedTime", default, skip_serializing_if = "Option::is_none")]
    pub total_queued_time: Option<String>,
    #[serde(rename = "totalRunningTime", default, skip_serializing_if = "Option::is_none")]
    pub total_running_time: Option<String>,
    #[serde(rename = "rootProcessNodeId", default, skip_serializing_if = "Option::is_none")]
    pub root_process_node_id: Option<String>,
    #[serde(rename = "yarnApplicationId", default, skip_serializing_if = "Option::is_none")]
    pub yarn_application_id: Option<String>,
    #[serde(rename = "yarnApplicationTimeStamp", default, skip_serializing_if = "Option::is_none")]
    pub yarn_application_time_stamp: Option<i64>,
    #[serde(rename = "compileMode", default, skip_serializing_if = "Option::is_none")]
    pub compile_mode: Option<u_sql_job_properties::CompileMode>,
}
impl USqlJobProperties {
    pub fn new(job_properties: JobProperties) -> Self {
        Self {
            job_properties,
            resources: Vec::new(),
            statistics: None,
            debug_data: None,
            diagnostics: Vec::new(),
            algebra_file_path: None,
            total_compilation_time: None,
            total_pause_time: None,
            total_queued_time: None,
            total_running_time: None,
            root_process_node_id: None,
            yarn_application_id: None,
            yarn_application_time_stamp: None,
            compile_mode: None,
        }
    }
}
pub mod u_sql_job_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CompileMode {
        Semantic,
        Full,
        SingleBox,
    }
}
