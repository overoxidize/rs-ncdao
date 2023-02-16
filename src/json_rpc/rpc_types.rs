use std::collections::HashMap;

use crate::{eos_api::api_types::{Authorization, TransactionTrace}};
use serde::{Deserialize, Serialize};
use sha2::digest::crypto_common::Key;
use crate::eos_api::api_types::AnyType;
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
    kv_tables: Option<KvTable>,
}

#[derive(Serialize, Deserialize)]
enum ActionResult {
    Some(Vec<ActionResultData>),
    None,
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
    new_type: String,
}

#[derive(Serialize, Deserialize)]
struct AbiStruct {
    name: String,
    struct_name: String,
    base: String,
    fields: Vec<AbiStructFields>,
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
    key_types: Vec<String>,
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
    None,
}

#[derive(Serialize, Deserialize)]
enum KvTable {
    Some(Vec<KvTableData>),
}
#[derive(Serialize, Deserialize)]
struct KvTableData {
    kvt_type: String,
    primary_index: KvPrimaryIndex,
    secondary_indices: Vec<KvSecondaryIndices>,
}
#[derive(Serialize, Deserialize)]
struct KvPrimaryIndex {
    name: String,
    pi_type: String,
}
#[derive(Serialize, Deserialize)]
struct KvSecondaryIndices {
    index_type: String,
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
    serialized_ctx_free_data: Vec<u8>,
}

pub struct ProcessedAction {
    account: String,
    name: String,
    authorization: Vec<Authorization>,
}
pub struct GetBlockInfoResult {
    timestamp: String,
    producer: String,
    confirmed: u32,
    previous: String,
    transaction_mroot: String,
    action_mroot: String,
    schedule_version: u32,
    producer_signature: String,
    id: String,
    block_num: u32,
    ref_block_num: u32,
    ref_block_prefix: u32,
}
struct ProducerKey {
    producer_name: String,
    block_signing_key: String,
}

struct ProducerScheduleType {
    version: u32,
    producers: Vec<ProducerKey>,
}
/** Return value of `/v1/chain/get_block` */
pub struct GetBlockResult {
    timestamp: String,
    producer: String,
    confirmed: u32,
    previous: String,
    transaction_mroot: String,
    action_mroot: String,
    schedule_version: u32,
    new_producers: Option<ProducerScheduleType>,
    producer_signature: String,
    transactions: Option<AnyType>,
    id: String,
    block_num: u32,
    ref_block_prefix: u32,
}
struct BlockHeader {
    timestamp: String,
    producer: String,
    confirmed: u32,
    previous: String,
    transaction_mroot: String,
    action_mroot: String,
    schedule_version: u32,
    new_producers: ProducerScheduleType,
    header_extensions: Vec<HashMap<u32, String>>
}

/** Used to calculate TAPoS fields in transactions */
pub struct BlockTaposInfo {
    block_num: u32,
    id: String,
    timestamp: String,
    header: BlockHeader,
}
struct SignedBlockHeader {
    producer_signature: String,
}

struct ScheduleInfo {
    schedule_lib_num: u32,
    schedule_hash: String,
    schedule: ProducerScheduleType,
}

struct ProtocolFeatureActivationSet {
    protocol_features: Vec<String>
}

struct StateExtension {
    security_group_info: SecurityGroupInfo
}
struct SecurityGroupInfo {
    version: u32,
    participants: Vec<String>
}

struct KeyWeight {
    key: String,
    weight: u32,
}

struct BlockSigningAuthority {
    threshold: u32,
    keys: Vec<KeyWeight>,
}

struct ProducerAuthority {
    producer_name: String,
    authority: BlockSigningAuthority
}

struct ProducerAuthoritySchedule {
    version: u32,
    producers: Vec<ProducerAuthority>
}

struct IncrementalMerkle {
    _active_nodes: Vec<String>,
    _node_count: u32
}


/** Return value of `/v1/chain/get_block_header_state` */
pub struct GetBlockHeaderStateResult {
    id: String,
    header: SignedBlockHeader,
    pending_schedule: ScheduleInfo,
    activated_protocol_features: ProtocolFeatureActivationSet,
    additional_signatures: Vec<String>,
    block_num: u32,
    dpos_proposed_irreversible_blocknum: u32,
    dpos_irreversible_blocknum: u32,
    active_schedule: ProducerAuthoritySchedule,
    blockroot_merkle: IncrementalMerkle,
    producer_to_last_produced: HashMap<String, u32>,
    producer_to_last_implied_irb: HashMap<String, u32>,
    // valid_block_signing_authority: BlockSigningAuthority,
    valid_block_signing_authority: AnyType,
    confirm_count: Vec<u32>,
    state_extension: StateExtension,
}

pub struct ReadOnlyTransactResult {
    head_block_num: u32,
    head_block_id: String,
    last_irreversible_block_num: u32,
    last_irreversible_block_id: String,
    code_hash: String,
    pending_transactions: Vec<String>,
    result: TransactionTrace,
}

/** Return value of `/v1/chain/get_info` */
pub struct GetInfoResult {
    server_version: String,
    chain_id: String,
    head_block_num: u32,
    last_irreversible_block_num: u32,
    last_irreversible_block_id: String,
    last_irreversible_block_time: String,
    head_block_id: String,
    head_block_time: String,
    head_block_producer: String,
    virtual_block_cpu_limit: u32,
    virtual_block_net_limit: u32,
    block_cpu_limit: u32,
    block_net_limit: u32,
    server_version_string: String,
    fork_db_head_block_num: u32,
    fork_db_head_block_id: String,
    server_full_version_string: String,
    first_block_num: u32,
}