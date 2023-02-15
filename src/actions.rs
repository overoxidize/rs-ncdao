use std::fmt::Error;
use eosio::{self, AccountName};
use std::any::{Any, TypeId};
use crate::types::{NCKeyValPair, NCNameType, NCCreateUser};

struct EosioAuthorizationObject {actor: String, permission: String}

struct EosioActionObject {
    account: String,
    name: String,
    authorization: Vec<EosioAuthorizationObject>,
    data: dyn Any + 'static
}


struct ActionGenerator {
    contract: String,
    token_contract: String
}

impl ActionGenerator {
    pub fn new() -> ActionGenerator {

        ActionGenerator {
            contract: String::from("A random smart contract"),
            token_contract: String::from("ERC-20 token contract")
        }
    }

    fn create_account(
        // Due to the need for a return type on a function that will instantiate an object,
        // the create account function will need addition fields in this language.
        new_name: String,
        payer: String,
        newacc_public_active_key: String,
        newacc_public_owner_key: String,
    ) -> NCCreateUser {
        unimplemented!()
    }
}