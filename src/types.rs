#[allow(dead_code)]
use serde::{Deserialize, Serialize};
// use eosio::{TransactionTrace};

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

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Authorization {
    actor: String,
    permission: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ProcessedAction {
    account: String,
    name: String,
    authorization: Vec<Authorization>,
    data: Option<AnyType>,
    hex_data: String,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub enum AnyType {
    // This is probably a code smell, or a bad idea, either way,
    // it is only meant to be a place holder until I figure out how to
    // best represent the `Any` type from TS' behavior in Rust, best practices considered.
    Some,
    None,
    #[serde(other)]
    #[default]
    Other,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct ActionTrace {
    action_ordinal: u32,
    creator_action_ordinal: u32,
    closest_unnotified_ancestor_action_ordinal: u32,
    receipt: ActionReceipt,
    receiver: String,
    act: ProcessedAction,
    context_free: bool,
    elapsed: u32,
    console: String,
    trx_id: String,
    block_num: u32,
    block_time: String,
    producer_block_id: Option<String>,
    account_ram_deltas: Vec<AccountDelta>,
    account_disk_deltas: Vec<AccountDelta>,
    except: Option<AnyType>,
    error_code: Option<u32>,
    return_value: Option<AnyType>,
    return_value_hex_data: String,
    return_value_data: Option<AnyType>,
    inline_traces: Vec<ActionTrace>,
}
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct AccountDelta {
    account: String,
    delta: u32,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct TransactionReceiptHeader {
    status: String,
    cpu_usage_us: usize,
    net_usage_words: usize,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct TransactionTrace {
    id: String,
    block_num: usize,
    block_time: String,
    producer_block_id: Option<String>,
    receipt: Option<TransactionReceiptHeader>,
    elapsed: u32,
    net_usage: u32,
    scheduled: bool,
    action_traces: Vec<ActionTrace>,
    account_ram_data: Option<AccountDelta>,
    except: Option<String>,
    error_code: Option<u32>, // Rewrite this as custom error type i.e Type Error = std::result::Result<T, ErrorType>
    bill_to_accounts: Vec<String>,
}

pub struct TransactResult {
    transaction_id: String,
    processed: TransactionTrace,
}

pub struct GetTableRowsPayload<'a> {
    pub json: bool,
    pub code: &'a String,
    pub scope: &'a String,
    pub table: String,
    pub table_key: Option<String>,
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
pub struct ProposalPayload<'a> {
    pub id: &'a String,
    pub contract: &'a String,
}

pub struct RewardPayload {
    category: String,
}

pub struct SlicePayload<'a> {
    pub date: &'a String,
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