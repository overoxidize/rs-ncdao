#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
mod io_sys;
mod types;
mod ncdao;
mod c_api;
mod submit;
mod json_rpc;
mod eos_api;

use reqwest::{Response, Error};

use crate::c_api::chain_api::{
    ChainApi, 
    AnyType, 
    get_table_rows, 
    get_table_rows_with_payload};
use io_sys::io::{NCInit};
use types::GetTableRowsPayload;

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

    let gtr_response: Response = get_table_rows().await;
    let data = gtr_response.json().await?;
    // let gtr_wp: Response = get_table_rows_with_payload(payload).await;


    println!("{:?}", data);

    Ok(())
}
