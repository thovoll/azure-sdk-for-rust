#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfidentialLedgerEnclaves {
    #[serde(rename = "currentNodeId")]
    pub current_node_id: EntityId,
    #[serde(rename = "enclaveQuotes")]
    pub enclave_quotes: EnclaveQuotes,
}
impl ConfidentialLedgerEnclaves {
    pub fn new(current_node_id: EntityId, enclave_quotes: EnclaveQuotes) -> Self {
        Self {
            current_node_id,
            enclave_quotes,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidentialLedgerError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ConfidentialLedgerErrorBody>,
}
impl ConfidentialLedgerError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfidentialLedgerErrorBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub innererror: Box<Option<ConfidentialLedgerErrorBody>>,
}
impl ConfidentialLedgerErrorBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Consortium {
    pub members: Vec<ConsortiumMember>,
}
impl Consortium {
    pub fn new(members: Vec<ConsortiumMember>) -> Self {
        Self { members }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsortiumMember {
    pub certificate: String,
    pub id: EntityId,
}
impl ConsortiumMember {
    pub fn new(certificate: String, id: EntityId) -> Self {
        Self { certificate, id }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Constitution {
    pub digest: String,
    pub script: String,
}
impl Constitution {
    pub fn new(digest: String, script: String) -> Self {
        Self { digest, script }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnclaveQuote {
    #[serde(rename = "nodeId")]
    pub node_id: EntityId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mrenclave: Option<String>,
    #[serde(rename = "quoteVersion")]
    pub quote_version: String,
    pub raw: String,
}
impl EnclaveQuote {
    pub fn new(node_id: EntityId, quote_version: String, raw: String) -> Self {
        Self {
            node_id,
            mrenclave: None,
            quote_version,
            raw,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EnclaveQuotes {}
impl EnclaveQuotes {
    pub fn new() -> Self {
        Self::default()
    }
}
pub type EntityId = String;
pub type LedgerEntries = Vec<LedgerEntry>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerEntry {
    pub contents: String,
    #[serde(rename = "subLedgerId", default, skip_serializing_if = "Option::is_none")]
    pub sub_ledger_id: Option<SubLedgerId>,
    #[serde(rename = "transactionId", default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<TransactionId>,
}
impl LedgerEntry {
    pub fn new(contents: String) -> Self {
        Self {
            contents,
            sub_ledger_id: None,
            transaction_id: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerQueryResult {
    pub state: LedgerQueryState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry: Option<LedgerEntry>,
}
impl LedgerQueryResult {
    pub fn new(state: LedgerQueryState) -> Self {
        Self { state, entry: None }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LedgerQueryState {
    Loading,
    Ready,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerUser {
    #[serde(rename = "assignedRole")]
    pub assigned_role: LedgerUserRole,
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,
}
impl LedgerUser {
    pub fn new(assigned_role: LedgerUserRole) -> Self {
        Self {
            assigned_role,
            user_id: None,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LedgerUserRole {
    Administrator,
    Contributor,
    Reader,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerWriteResult {
    #[serde(rename = "subLedgerId")]
    pub sub_ledger_id: SubLedgerId,
}
impl LedgerWriteResult {
    pub fn new(sub_ledger_id: SubLedgerId) -> Self {
        Self { sub_ledger_id }
    }
}
pub type MerkleProof = Vec<MerkleProofElement>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MerkleProofElement {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
}
impl MerkleProofElement {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PagedLedgerEntries {
    pub state: LedgerQueryState,
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
    pub entries: LedgerEntries,
}
impl PagedLedgerEntries {
    pub fn new(state: LedgerQueryState, entries: LedgerEntries) -> Self {
        Self {
            state,
            next_link: None,
            entries,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReceiptContents {
    pub leaf: String,
    #[serde(rename = "nodeId")]
    pub node_id: EntityId,
    pub proof: MerkleProof,
    pub root: String,
    pub signature: String,
}
impl ReceiptContents {
    pub fn new(leaf: String, node_id: EntityId, proof: MerkleProof, root: String, signature: String) -> Self {
        Self {
            leaf,
            node_id,
            proof,
            root,
            signature,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoleAssignment {
    #[serde(rename = "roleName")]
    pub role_name: LedgerUserRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl RoleAssignment {
    pub fn new(role_name: LedgerUserRole) -> Self {
        Self {
            role_name,
            description: None,
        }
    }
}
pub type SubLedgerId = String;
pub type TransactionId = String;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionReceipt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt: Option<ReceiptContents>,
    pub state: LedgerQueryState,
    #[serde(rename = "transactionId")]
    pub transaction_id: TransactionId,
}
impl TransactionReceipt {
    pub fn new(state: LedgerQueryState, transaction_id: TransactionId) -> Self {
        Self {
            receipt: None,
            state,
            transaction_id,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TransactionState {
    Committed,
    Pending,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionStatus {
    pub state: TransactionState,
    #[serde(rename = "transactionId")]
    pub transaction_id: TransactionId,
}
impl TransactionStatus {
    pub fn new(state: TransactionState, transaction_id: TransactionId) -> Self {
        Self { state, transaction_id }
    }
}
pub type UserId = String;
