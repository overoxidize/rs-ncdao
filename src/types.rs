use crate::eos_api::api_types::TransactResult;
use eosio::ProducerAuthority;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

struct NCGetAccInfo {
    owner: String,
    contract: String,
    token_name: String,
}
struct NCMintAsset {
    creator: String,
    col_name: String,
    sch_name: String,
    tmpl_id: u32,
    immutable_data: Vec<HashMap<String, ()>>,
    mutable_data: Vec<HashMap<String, ()>>,
    payer: String,
    payer_prv_key: String,
}

struct NCKeyValPair {
    key: String,
    value: Vec<String>,
}

struct NCGetPoolInfo {
    owner: String,
    code: String,
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

struct TxIdStakePool {
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
    description_sha_256: String,
}
struct NCReturnInfo {
    acc_balances: Vec<String>,
}

struct Header(i32, String);
struct ProducerScheduleType {
    version: i32,
    producers: Vec<ProducerAuthority>,
}
struct BlockHeader {
    timestamp: String,
    producer: String,
    confirmed: i32,
    previous: String,
    tx_mroot: String,
    action_mroot: String,
    schedule_version: i32,
    new_producers: Option<ProducerScheduleType>,
    header_extensions: Vec<Header>,
}

struct NCKeyPair {
    pub_key: String,
    prv_key: String,
}

struct NCNameType {
    name: String,
    r#type: String,
}

struct NCBuyRam {
    user: String,
    payer: String,
    payer_prv_key: String,
    ram_amt: u32,
}

struct NCCreateUser {
    new_user: String,
    newacc_pub_active_key: String,
    newacc_pub_owner_key: String,
    payer: String,
    payer_prv_key: String,
    ram_amt: u32,
    cpu_amount: String,
    net_amount: String,
    xfer: bool, // stake or transfer CPU/NET to the account
}

struct NCCreateCollection {
    user: String,
    user_prv_active_key: String,
    collection_name: String,
    schema_name: String,
    schema_fields: Vec<String>,
    template_name: String,
    template_fields: Vec<String>,
    mkt_fee: u32,
    allow_notify: bool,
    xferable: bool,
    burnable: bool,
    max_supply: u32,
}

struct NCCreatePermission {
    author: String,
    perm_name: String,
    perm_pub_key: String,
    author_prv_active_key: String,
}

struct NCLinkPerm {
    author: String, // the owner of the permission
    perm_to_link: String,
    action_owner: String,
    action_to_link: String,
    author_prv_active_key: String,
}

struct NCCreatePool {
    owner: String,
    owner_prv_active_key: String,
    ticker: String,
    is_inflatable: bool,
    is_deflatable: bool,
    is_treasury: bool,
}

struct NCStakeMainDao {
    amt: String,
    payer: String,
    payer_prv_key: String,
}

struct NCStakePool {
    owner: String,
    amt: String,
    payer: String,
    payer_prv_key: String,
}

struct NCUnstakePool {
    amt: String,
    payer: String,
    payer_prv_key: String,
}

struct NCTxNcoBal {
    to: String,
    amt: String,
    payer: String,
    memo: String,
    payer_prv_key: String,
}

struct NCTxBal {
    to: String,
    amt: String,
    payer: String,
    memo: String,
    payer_prv_key: String,
}

struct NCPoolInfoTotal {
    quantity: String,
    contract: String,
}
struct NCPoolInfo {
    id: String,
    code: String,
    owner: String,
    description: String,
    total: NCPoolInfoTotal,
    creation_date: String,
    last_update_date: String,
}

struct NCPoolsInfo {
    rows: Vec<String>,
    more: bool,
    next_key: String,
}

pub struct NCModifyAsset {
    editor: String,
    owner: String,
    asset_id: String,
    new_data: Vec<NCKeyPair>,
    payer: String,
    payer_prv_key: String,
}

pub struct NCMintNftToRoot {
    creator: String,
    immutable_data: Vec<NCKeyPair>,
    mutable_data: Vec<NCKeyPair>,
    payer: String,
    payer_prv_key: String,
}

pub struct NCBindCollection {
    creator: String,
    col_name: String,
    description: String,
    image: String,
    payer: String,
    payer_prv_key: String,
}

pub struct NCMintProfile {
    status: String,
    offer: String,

    display_namee: String,
    source: String, // one of the socials
    authority: String,
    signature: String,

    content: String,
    bio: String,
    full_name: String,
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    phone: String,

    content_type: String,
    content_url: String,
    cover_content_url: String,
    blur_hash: String,
    aspect_ratio: String,
    creator: String,

    instagram: String,
    tiktok: String,
    youtube: String,
    twitter: String,
    spotify: String,
    pinterest: String,
    snapchat: String,
    reddit: String,
    discord: String,
    tumblr: String,
    soundcloud: String,
    apple: String,
    telegram: String,
    signal: String,
    medium: String,
    facebook: String,
    facebook_id: String,
    youtube_id: String,
    payer: String,

    payer_prv_key: String,
    user_prv_active_key: String,
}

pub struct NCSwapNCOtoCC {
    amt: String,
    payer: String,
    payer_prv_key: String,
    creator_to: String,
}
