use crate::eos_api::api_types::{Authorization};
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

enum ActionResult {
    Some(Vec<ActionResultData>),
    None
}
struct ActionResultData {
    result_name: String,
    result_type: String,
}




struct RicardianClauses {
    clause_id: String,
    body: String,
}

struct ErrorMessages {
    error_code: i32,
    error_msg: String,
}


struct AbiType {
    new_type_name: String,
    new_type: String
}

struct  AbiStruct {
    name: String,
    struct_name: String,
    base: String,
    fields: Vec<AbiStructFields>
}

struct AbiStructFields {
    name: String,
    field_type: String,
}

struct AbiActions {
    name: String,
    abi_action_type: String,
    ricardian_contract: String,
}

struct AbiTable {
    table_name: String,
    table_type: String,
    index_type: String,
    key_names: Vec<String>,
    key_types: Vec<String>
}

struct AbiExtension {
    tag: i32,
    value: String,
}

struct AbiVariantData {
    variant_name: String,
    variant_types: Vec<String>,
}
enum AbiVariant {
    Some(Vec<AbiVariantData>),
    None
}

enum KvTable {
    Some(Vec<KvTableData>)
}
struct KvTableData {
    kvt_type: String,
    primary_index: KvPrimaryIndex,
    secondary_indices: Vec<KvSecondaryIndices>

}
struct KvPrimaryIndex {
    name: String,
    pi_type: String,
}
struct KvSecondaryIndices {
    index_type: String
}

pub struct JsonRpcEndpoint(String);

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