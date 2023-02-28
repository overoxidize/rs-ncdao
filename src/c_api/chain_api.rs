extern crate dotenv;
extern crate reqwest;
extern crate serde;
use crate::eos_api::api_types::{AnyType, TransactResult};
use crate::io_sys::io::{NCInit, NCInitServices, NCInitUrlsDev};
use crate::types::{
    GetTableRowsPayload, ProposalPayload, SlicePayload, TopPoolPayload, VotePayload,
    WhiteListPayload,
};
use dotenv::dotenv;
use reqwest::header::{self, HeaderValue, InvalidHeaderValue};
use reqwest::{get, header::HeaderMap, Client, Request, Response};
use serde::{Deserialize, Serialize};
use std::env;
type Error = reqwest::Error;

#[derive(Default, Clone, Serialize, Debug, Deserialize)]

pub struct ChainApi {
    pub nodeos_url: String,
    pub contract: String,
    pub fetch: Option<AnyType>,
}

// TODO: *Some* uses of .clone() and .into() are anti-patterns, and eventually, an Rc or Arc will be
// needed to provide trackable shared ownership of a piece of data.
// https://rust-unofficial.github.io/patterns/anti_patterns/borrow_clone.html

pub async fn get_table_rows() -> Result<Response, Error> {
    dotenv().ok();

    let init_url = NCInitUrlsDev::default().nodeos_url.clone();
    let init_proxy_url = NCInitUrlsDev::default().nodeos_url.clone();
    let url_val: String = init_url.clone() + "v1/tx/newcoin";

    let mut headers = HeaderMap::new();

    fn header_value(val: String) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(&val)
    }

    let hv: fn(String) -> Result<HeaderValue, InvalidHeaderValue> = header_value;

    headers.insert("username", hv(String::from("dx.io")).unwrap());

    headers.insert("phone_number", hv(String::from("+420111111112")).unwrap());
    headers.insert(
        "verification_code",
        hv(String::from(dotenv!("VERIFICATION_CODE"))).unwrap(),
    );
    headers.insert(
        "private_key",
        hv(String::from(dotenv!("PRIVATE_KEY"))).unwrap(),
    );

    let init_client = Client::builder().build().unwrap();

    let resp = init_client.get(url_val).headers(headers).send().await;

    resp
}

pub async fn get_table_rows_with_payload(opts: &GetTableRowsPayload) -> Result<Response, Error> {
    dotenv().ok();

    let init_url = NCInitUrlsDev::default().nodeos_url.clone();

    let url_val: String = init_url.clone() + "v1/tx/newcoin";

    let mut headers = HeaderMap::new();

    fn header_value(val: String) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(&val)
    }

    let hv: fn(String) -> Result<HeaderValue, InvalidHeaderValue> = header_value;

    headers.insert("username", hv(String::from("dx.io")).unwrap());

    headers.insert("phone_number", hv(String::from("+420111111112")).unwrap());
    headers.insert(
        "verification_code",
        hv(String::from(dotenv!("VERIFICATION_CODE"))).unwrap(),
    );
    headers.insert(
        "private_key",
        hv(String::from(dotenv!("PRIVATE_KEY"))).unwrap(),
    );

    let init_client = Client::builder().build().unwrap();

    init_client.get(url_val).headers(headers).send().await

}

pub async fn get_proposal_by_id(opts: &GetTableRowsPayload) -> Result<Response, Error> {
    dotenv().ok();

    let init_url = NCInitUrlsDev::default().nodeos_proxy_url.clone();

    let url_val: String = init_url.clone() + "v1/tx/newcoin";

    let mut headers = HeaderMap::new();

    fn header_value(val: String) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(&val)
    }

    let hv: fn(String) -> Result<HeaderValue, InvalidHeaderValue> = header_value;

    headers.insert("username", hv(String::from("dx.io")).unwrap());

    headers.insert("phone_number", hv(String::from("+420111111112")).unwrap());
    headers.insert(
        "verification_code",
        hv(String::from(dotenv!("VERIFICATION_CODE"))).unwrap(),
    );
    headers.insert(
        "Authorization",
        hv("newsafe".to_owned() + &String::from(dotenv!("PRIVATE_KEY"))).unwrap(),
    );

    let init_client = Client::builder().build().unwrap();

    get_table_rows_with_payload(opts).await
}

// pub async fn get_proposal_by_contract(opts: ProposalPayload) -> Result<Response, Error> {


// }

// async fn get_proposal_by_contract(opts: ProposalPayload) -> Result<Response, Error> {
//     let payload = ProposalPayload {
//         id: opts.id,
//         contract: opts.contract,
//     };

//     let gtr_payload = GetTableRowsPayload {
//         json: true,
//         code: payload.contract.clone(),
//         scope: payload.contract.clone(),
//         table: "proposals".into(),
//         table_key: payload.contract.clone(),
//         lower_bound: payload.contract.clone(),
//         upper_bound: payload.contract.clone(),
//         key_type: "i64".into(),
//         index_position: "1".into(),
//         encode_type: "".into(),
//         limit: 0,
//         reverse: false,
//         show_payer: false,
//     };

//     // let resp = get_table_rows_with_payload(gtr_payload).await;
//     unimplemented!()
//     Ok(resp)
// }

// pub async fn get_slice(opts: SlicePayload) -> Response {

//     let slice_data = SlicePayload { date: opts.date };
//     let payload_1 = GetTableRowsPayload {
//         json: true,
//         code: payload.code,
//         scope: payload.scope,
//         table: "proposals".into(),
//         table_key: payload.table_key,
//         lower_bound: payload.lower_bound.into(),
//         upper_bound: payload.upper_bound.into(),
//         key_type: "i64".into(),
//         index_position: "1".into(),
//         encode_type: "".into(),
//         limit: 0,
//         reverse: false,
//         show_payer: false,
//     };
//     let resp = get_table_rows_with_payload(payload).await;

//     resp
// }
// async getSlice(opts: SlicePayload): Promise<any> {
//   return this.getTableRows({
//   });
// }
pub async fn get_top_pool(opts: TopPoolPayload) -> Response {
    unimplemented!()
}

impl ChainApi {
    pub fn new(nodeos_url: String, contract: String, fetch: Option<AnyType>) -> ChainApi {
        ChainApi {
            nodeos_url,
            contract,
            fetch,
        }
    }

    //   pub async fn get_slice(self, opts: SlicePayload<'_>) -> Response {

    //     let payload = GetTableRowsPayload {
    //         json: true,
    //         code: self.code,
    //         scope: self.scope,
    //         table: "slice".into(),
    //         table_key: Some(opts.date.into()),
    //         lower_bound: opts.date.into(),
    //         upper_bound: opts.date.into(),
    //         key_type: "i64".into(),
    //         index_position: "1".into(),
    //         encode_type: "".into(),
    //         limit: 0,
    //         reverse: false,
    //         show_payer: false

    //     };
    //     let resp = self.get_table_rows_with_payload(payload).await;

    //         resp
    //   }
    // async getSlice(opts: SlicePayload): Promise<any> {
    //   return this.getTableRows({
    //   });
    // }
    pub async fn get_top_pool(opts: TopPoolPayload) -> Response {
        unimplemented!()
    }
    // async getTopPool(opts: TopPoolPayload): Promise<any> {
    //   return this.getTableRows({
    //     json: true,
    //     code: this.contract,
    //     scope: this.contract,
    //     table: "toppools",
    //     table_key: opts.category,
    //     lower_bound: opts.category,
    //     upper_bound: opts.category,
    //     key_type: "i64",
    //     index_position: "1",
    //   });
    // }

    pub async fn get_vote(opts: VotePayload) -> Response {
        unimplemented!()
    }

    // async getVote(opts: VotePayload): Promise<any> {
    //   return this.getTableRows({
    //     json: true,
    //     code: this.contract,
    //     scope: opts.proposal_id,
    //     table: "votes",
    //     table_key: opts.voter,
    //     lower_bound: opts.voter,
    //     upper_bound: opts.voter,
    //     key_type: "name",
    //     index_position: "1",
    //   });
    // }

    pub async fn get_contract_whitelist(opts: WhiteListPayload) -> Response {
        unimplemented!()
    }
    // async getContractWhiteList(opts: WhiteListPayload): Promise<any> {
    //   return this.getTableRows({
    //     json: true,
    //     code: this.contract,
    //     scope: this.contract,
    //     table: "cntwhitelist",
    //     table_key: opts.account,
    //     lower_bound: opts.account,
    //     upper_bound: opts.account,
    //     key_type: "name",
    //     index_position: "1",
    //   });
    // }
}
