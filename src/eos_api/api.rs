use flate2::{Compress, Decompress};
use std::collections::HashMap;
use std::hash::Hash;
use crate::eos_api::api_types::{
    // AbiProvider,
    // ActionSerializerType,
    // AuthorityProvider,
    BinaryAbi,
    // CachedAbi,
    // ContextFreeGroupCallback,
    // Query,
    // QueryConfig,
    // SignatureProvider,
    // TransactConfig,
    // Transaction,
    TransactResult, 

};
use crate::json_rpc::rpc_types::{
    Abi,
    // BlockTaposInfo,
    // GetInfoResult,
    PushTransactionArgs,
    // GetBlockHeaderStateResult,
    // GetBlockInfoResult,
    // GetBlockResult,
    // ReadOnlyTransactResult,
    JsonRpc
};


pub struct Api {
    pub rpc: JsonRpc, //
    // pub authority_provider: AuthorityProvider,
    // pub abi_provider: AbiProvider
    // pub signature_provider: SignatureProvider,
    // pub chain_id: String,

    // pub text_encoder: TextEncoder
    // pub text_decoder: TextDecoder
        //* The functionality of these objects may end up being supplied by flate2 */
    
    pub abi_types: HashMap<String, ()>,

    pub transaction_types: HashMap<String, ()>,

    pub contracts: HashMap<String, ()>, //

    // pub cached_abis: HashMap<String, CachedAbi>
    
}


impl Api {
    fn new() -> Api {
        // let val;
        // Api {
        //     rpc: JsonRpc {endpoint: val, input: "".to_owned(), init: "".to_owned()} ,
        //     authority_provider: AuthorityProvider,
        //     abi_provider: AbiProvider,
        //     chain_id: String,
        //     text_encoder: TextEncoder,
        //     text_decoder: TextDecoder,
        //     abi_types: HashMap::new(),
        //     transaction_types: HashMap::new(),
        //     contracts: HashMap::new()

        // };
        unimplemented!();
    }
}