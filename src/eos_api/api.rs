use crate::eos_api::api_types::{
    AbiProvider,
    Api,
    AuthorityProvider,
    ActionSerializerType,
    AuthorityProviderArgs,
    BinaryAbi,
    CachedAbi,
    ContextFreeGroupCallback,
    Query,
    QueryConfig,
    SignatureProvider,
    SignatureProviderArgs,
    TransactConfig,
    Transaction,
    TransactResult,
};
use crate::json_rpc::rpc_types::{
    Abi,
    GetBlockHeaderStateResult,
    GetBlockInfoResult,
    GetBlockResult,
    ReadOnlyTransactResult,
    JsonRpc,
    JsonRpcEndpoint,
    BlockTaposInfo,
    GetInfoResult,
    PushTransactionArgs,
};
use flate2::{Compress, Decompress};
use std::collections::HashMap;
impl Api {
    fn new() -> Api {
        let val: JsonRpcEndpoint = JsonRpcEndpoint("".to_owned());
        Api {
            rpc: JsonRpc {
                endpoint: val,
                input: "".to_owned(),
                init: "".to_owned(),
            },
            authority_provider: AuthorityProvider,
            abi_provider: AbiProvider,
            signature_provider: SignatureProvider,
            chain_id: "".to_owned(),
            //     text_encoder: TextEncoder,
            //     text_decoder: TextDecoder,
            abi_types: HashMap::new(),
            transaction_types: HashMap::new(),
            contracts: HashMap::new(),
            cached_abis: HashMap::new(),
        }
    }
}

impl AbiProvider {
    pub fn get_required_keys(args: AuthorityProviderArgs) -> Option<Vec<String>> {
        unimplemented!()
    }

    pub fn get_raw_abi(account_name: String) -> Option<BinaryAbi> {
        let raw_abi = reqwest::get(account_name);

        unimplemented!()
    }
}

impl SignatureProvider {
    pub fn get_available_keys(args: SignatureProviderArgs) -> Option<Vec<String>> {
        Some(args.required_keys)
    }
}
