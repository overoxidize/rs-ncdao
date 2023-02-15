#[allow(dead_code)]
use crate::eos_api::api_types::{TransactResult};
use serde::{Deserialize, Serialize};

pub struct NCCreateDao {
    author: String,
    author_prv_key: String,
    token: String,
    descr: String,
}

pub struct NCCreateDaoProposal {
    proposer: String,
    proposer_prv_key: String,
    dao_id: String,
    dao_owner: String,
    title: String,
    summary: String,
    url: String,
    pass_rate: usize,
    vote_start: String,
    vote_end: String,
}

pub struct NCCreateDaoUserWhitelistProposal {
    proposer: String,
    proposer_prv_key: String,
    dao_id: String,
    dao_owner: String,
    user: String,
    _type: String,
    pass_rate: usize,
    vote_start: String,
    vote_end: String,
}

pub struct NCGetDaoWhiteList {
    dao_id: String,
    dao_owner: String,
    lower_bound: String,
    upper_bound: String,
    limit: String,
    reverse: bool,
}

struct NCStakeProposalQuantity {
    quantity: String,
    contract: String,
}

pub struct NCCreateDaoStakeProposal {
    proposer: String,
    proposer_prv_key: String,
    dao_id: String,
    dao_owner: String,
    to: String,
    quantity: NCStakeProposalQuantity,
    pass_rate: usize,
    vote_start: String,
    vote_end: String,
}

pub struct NCApproveDaoProposal {
    approver: String,
    approver_prv_key: String,
    dao_id: usize,
    dao_owner: String,
    proposal_id: usize,
    proposal_author: String,
}

pub struct NCExecuteDaoProposal {
    exec: String,
    exec_prv_key: String,
    dao_id: usize,
    dao_owner: String,
    proposal_id: usize,
    proposal_author: String,
}

pub struct NCGetDaoProposals {
    dao_id: String,
    dao_owner: String,

    proposal_id: String,
    proposal_author: String,

    lower_bound: String,
    upper_bound: String,
    limit: usize,
    reverse: bool,
}

pub struct NCDaoProposalVote {
    voter: String,
    voter_prv_key: String,
    dao_id: String,
    dao_owner: String,
    proposal_id: String,
    proposal_type: String,
    quantity: String,
    option: String, // YES/NO
}

pub struct NCDaoWithdrawVoteDeposit {
    voter: String,
    voter_prv_key: String,
    vote_id: String,
}

pub struct NCGetVotes {
    voter: String,
    lower_bound: String,
    upper_bound: String,
    limit: String,
    reverse: bool,
}

struct TxIdCreateDaoProposal {
    proposal: String,
    pool_code: String,
    pool_id: String,
}

pub struct NCReturnTxs {
    tx_id_create_acc: String,
    tx_id_create_col: String,
    tx_id_create_sch: String,
    tx_id_create_tpl: String,

    tx_id_create_perm: String,
    tx_id_link_perm: String,

    tx_id_create_pool: String,
    tx_id_stake_pool: String,
    pool_code: String,
    pool_id: String,
    tx_id_unstake_pool: String,

    tx_id_create_dao: String,
    tx_id_create_dao_proposal: String,
    dao_id: String,
    proposal_id: usize,
    tx_id_approve_dao_proposal: String,
    tx_id_execute_dao_proposal: String,
    tx_id_vote_dao_proposal: String,
    tx_id_withdraw_vote_deposit: String,

    tx_id_withdraw_from_pool: String,
    tx_id_add_to_white_list: String,
    tx_id_remove_from_white_list: String,
    tx_id_stake_main_dao: String,
    tx_id_unstake_main_dao: String,

    tx_id_mint_asset: String,
    asset_id: String,
    tx_id_modify_asset: String,

    tx_id_mint_nft: String,
    tx_id_mint_profile: String,
    tx_id_mint_file: String,
    tx_id_change_file: String,
    tx_id_bind_collection: String,

    tx_id_tx_nco_balance: String,
    tx_id: String,
    tx: TransactResult,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]

struct AuthSequence(String, u32);
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct ActionReceipt {
    receiver: String,
    act_digest: String,
    global_sequence: u64,
    recv_sequence: u32,
    auth_sequence: Vec<AuthSequence>,
    code_sequence: u32,
    abi_sequence: u32,
}


pub struct GetTableRowsPayload {
    pub json: bool,
    pub code: String,
    pub scope: String,
    pub table: String,
    pub table_key: String,
    pub lower_bound: String,
    pub upper_bound: String,
    pub key_type: String,
    pub index_position: String,
    pub encode_type: String,
    pub limit: i32,
    pub reverse: bool,
    pub show_payer: bool,
}
#[derive(Clone)]
pub struct ProposalPayload {
    pub id: String,
    pub contract: String,
}

pub struct RewardPayload {
    category: String,
}

pub struct SlicePayload {
    pub date: String,
}

pub struct TopPoolPayload {
    category: String,
}

pub struct VotePayload {
    proposal_id: String,
    voter: String,
}

pub struct WhiteListPayload {
    account: String,
}

pub struct ActionGenerator {
    contract: String,
    token_contract: String,
}

pub struct DAOPayload {
    id: String,
    owner: String,
    description_sha_256: String
}


// struct AbiType {
//     new_type_name: String,
//     new_type: String
// }

// struct  AbiStruct {
//     name: String,
//     struct_name: String,
//     base: String,
//     fields: Vec<AbiStructFields>
// }

// struct AbiStructFields {
//     name: String,
//     field_type: String,
// }

// struct AbiActions {
//     name: String,
//     abi_action_type: String,
//     ricardian_contract: String,
// }

// struct AbiTable {
//     table_name: String,
//     table_type: String,
//     index_type: String,
//     key_names: Vec<String>,
//     key_types: Vec<String>
// }

// struct RicardianClauses {
//     clause_id: String,
//     body: String,
// }

// struct ErrorMessages {
//     error_code: i32,
//     error_msg: String,
// }

// struct AbiExtension {
//     tag: i32,
//     value: String,
// }

// struct AbiVariantData {
//     variant_name: String,
//     variant_types: Vec<String>,
// }
// enum AbiVariant {
//     Some(Vec<AbiVariantData>),
//     None
// }

// struct ActionResultData {
//     result_name: String,
//     result_type: String,
// }

// enum ActionResult {
//     Some(Vec<ActionResultData>),
//     None
// }
// struct KvSecondaryIndices {
//     index_type: String
// }
// struct KvPrimaryIndex {
//     name: String,
//     pi_type: String,
// }
// struct KvTableData {
//     kvt_type: String,
//     primary_index: KvPrimaryIndex,
//     secondary_indices: Vec<KvSecondaryIndices>

// }

// enum KvTable {
//     Some(Vec<KvTableData>)
// }
// struct ABI {
//     version: String,
//     types: Vec<AbiType>,
//     structs: Vec<AbiStruct>,
//     actions: Vec<AbiActions>,
//     tables: Vec<AbiTable>,
//     ricardian_clauses: Vec<RicardianClauses>,
//     error_messages: Vec<ErrorMessages>,
//     abi_extensions: Vec<AbiExtension>,
//     variants: Option<AbiVariant>,
//     action_results: Option<ActionResult>,
//     kv_tables: Option<KvTable>


// }

// struct Header(i32, String);
// struct ProducerScheduleType {
//     version: i32,
//     producers: Vec<ProducerAuthority>
// }
// struct BlockHeader {
//     timestamp: String,
//     producer: String,
//     confirmed: i32,
//     previous: String,
//     tx_mroot: String,
//     action_mroot: String,
//     schedule_version: i32,
//     new_producers: Option<ProducerScheduleType>,
//     header_extensions: Vec<Header>
// }

// struct Authorization {
//     actor: String,
//     permission: String,
// }