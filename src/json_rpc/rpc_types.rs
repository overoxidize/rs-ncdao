use crate::eos_api::api_types::{Authorization};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
pub struct Abi {
    version: String,
    types: Vec<AbiType>,
    structs: Vec<AbiStruct>,
    actions: Vec<AbiActions>,
    tables: Vec<AbiTable>,
    ricardian_clauses: Vec<RicardianClauses>,
    error_messages: Vec<ErrorMessages>,
    abi_extensions: Vec<AbiExtension>,
    variants: Option<AbiVariant>,
    action_results: Option<ActionResult>,
    kv_tables: Option<KvTable>
}

#[derive(Serialize, Deserialize)]
enum ActionResult {
    Some(Vec<ActionResultData>),
    None
}
#[derive(Serialize, Deserialize)]
struct ActionResultData {
    result_name: String,
    result_type: String,
}

#[derive(Serialize, Deserialize)]
struct RicardianClauses {
    clause_id: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct ErrorMessages {
    error_code: i32,
    error_msg: String,
}

#[derive(Serialize, Deserialize)]
struct AbiType {
    new_type_name: String,
    new_type: String
}

#[derive(Serialize, Deserialize)]
struct  AbiStruct {
    name: String,
    struct_name: String,
    base: String,
    fields: Vec<AbiStructFields>
}
#[derive(Serialize, Deserialize)]
struct AbiStructFields {
    name: String,
    field_type: String,
}
#[derive(Serialize, Deserialize)]
struct AbiActions {
    name: String,
    abi_action_type: String,
    ricardian_contract: String,
}
#[derive(Serialize, Deserialize)]
struct AbiTable {
    table_name: String,
    table_type: String,
    index_type: String,
    key_names: Vec<String>,
    key_types: Vec<String>
}
#[derive(Serialize, Deserialize)]
struct AbiExtension {
    tag: i32,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct AbiVariantData {
    variant_name: String,
    variant_types: Vec<String>,
}
#[derive(Serialize, Deserialize)]
enum AbiVariant {
    Some(Vec<AbiVariantData>),
    None
}

#[derive(Serialize, Deserialize)]
enum KvTable {
    Some(Vec<KvTableData>)
}
#[derive(Serialize, Deserialize)]
struct KvTableData {
    kvt_type: String,
    primary_index: KvPrimaryIndex,
    secondary_indices: Vec<KvSecondaryIndices>

}
#[derive(Serialize, Deserialize)]
struct KvPrimaryIndex {
    name: String,
    pi_type: String,
}
#[derive(Serialize, Deserialize)]
struct KvSecondaryIndices {
    index_type: String
}

#[derive(Serialize, Deserialize)]
pub struct JsonRpcEndpoint(pub String);
#[derive(Serialize, Deserialize)]
pub struct JsonRpc {
    pub endpoint: JsonRpcEndpoint,
    pub input: String,
    pub init: String,
}

pub struct PushTransactionArgs {
    signatures: Vec<String>,
    compression: i32,
    serialized_transaction: Vec<u8>,
    serialized_ctx_free_data: Vec<u8>

}

pub struct ProcessedAction {
    account: String,
    name: String,
    authorization: Vec<Authorization>
}