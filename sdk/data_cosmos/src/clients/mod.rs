//! Clients for interacting with Cosmos resources.
//!
//! Each resource has its own client, meaning if you want to interact with Attachments, for example,
//! you need to use the [`AttachmentClient`].
//!
//! # Example
//!
//! ```no_run
//! use azure_data_cosmos::prelude::*;
//!
//! let account = todo!("Get Cosmos account name from the Azure Portal");
//! let authorization_token = todo!("Get Cosmos authorization token from the Azure Portal");
//! let database_name: String = todo!("Think of some database name");
//!
//! // Create an http client, then a `CosmosClient`, and then a `DatabaseClient`
//! let client = CosmosClient::new(account, authorization_token, CosmosOptions::default());
//! let client = client.into_database_client(database_name);
//! ```

mod attachment_client;
mod collection_client;
mod cosmos_client;
mod database_client;
mod document_client;
mod permission_client;
mod stored_procedure_client;
mod trigger_client;
mod user_client;
mod user_defined_function_client;

pub use attachment_client::AttachmentClient;
pub use collection_client::CollectionClient;
pub use cosmos_client::{CosmosClient, CosmosOptions};
pub use database_client::DatabaseClient;
pub use document_client::DocumentClient;
pub use permission_client::PermissionClient;
pub use stored_procedure_client::StoredProcedureClient;
pub use trigger_client::TriggerClient;
pub use user_client::UserClient;
pub use user_defined_function_client::UserDefinedFunctionClient;
