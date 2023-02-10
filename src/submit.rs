use crate::io_sys::{NCInit, NCInitServices, NCInitUrlsDev};
use crate::types::{ActionGenerator, AnyType};
use crate::chain_api::ChainApi;
pub struct AuthorityProviderArgs {
    transaction: AnyType,
    available_keys: Vec<String>
}

// struct NCSubmitApi {

// }

