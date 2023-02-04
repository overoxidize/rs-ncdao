#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
mod io_sys;
mod types;
mod ncdao;
mod chain_api;
mod submit;
use reqwest::Response;

// TODO: use static variables to instantiate NCInit structs.
// TODO: pull in types from eos rust libraries

use chain_api::{ChainApi, AnyType};
use futures::Future;
use io_sys::{NCInit, NCInitServices, NCInitUrlsDev};
use types::GetTableRowsPayload;

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    // let response: Response = reqwest::get(DEV_NODEOS_URL.to_string() + "/v1/chain/get_table_rows".into()).await?;

    // let data = response.json().await?;


    // println!("{:?}", data);

    Ok(())
}
