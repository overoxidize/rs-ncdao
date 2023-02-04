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

use chain_api::{ChainApi, AnyType, get_table_rows, get_table_rows_with_payload};
use futures::Future;
use io_sys::{NCInit, NCInitServices, NCInitUrlsDev};
use types::GetTableRowsPayload;

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {

    // let payload = GetTableRowsPayload {
    //     json: true,
    //     code: code,
    //     scope: payload.scope,
    //     table: "proposals".into(),
    //     table_key: payload.table_key,
    //     lower_bound: payload.lower_bound.into(),
    //     upper_bound: payload.upper_bound.into(),
    //     key_type: "i64".into(),
    //     index_position: "1".into(),
    //     encode_type: "".into(),
    //     limit: 0,
    //     reverse: false,
    //     show_payer: false,
    // };

    let url_val = NCInitUrlsDev::default().nodeos_url.clone();
    let gtr_response: Response = get_table_rows().await;
    // let gtr_wp: Response = get_table_rows_with_payload(payload).await;
    let data = gtr_response.json().await?;


    println!("{:?}", data);

    Ok(())
}
