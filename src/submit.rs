use crate::io_sys::{NCInit, NCInitServices, NCInitUrls};
use crate::types::{ActionGenerator, AnyType};
use crate::chain_api::ChainApi;
struct AuthorityProviderArgs {
    transaction: AnyType,
    available_keys: Vec<String>
}

// struct NCSubmitApi {

// }
