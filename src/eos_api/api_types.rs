use serde::{Serialize, Deserialize};
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
pub struct AccountDelta {
    account: String,
    delta: u32,
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

pub struct BinaryAbi {
    account_name: String,
    abi: Vec<u8>,

}

pub struct AuthorityProviderArgs {
    transaction: AnyType,
    available_keys: Vec<String>
}

pub struct AbiProvider;

impl AbiProvider {
    pub fn get_required_keys(args: AuthorityProviderArgs) -> Option<Vec<String>> {
        unimplemented!()
    }


    pub fn get_raw_abi(account_name: String) -> Option<BinaryAbi> {
        let raw_abi = reqwest::get(account_name);

        
        unimplemented!()
    }
}