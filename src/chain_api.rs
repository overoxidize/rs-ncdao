
extern crate reqwest;
extern crate serde;
use reqwest::{get, Response};
use futures::executor::block_on;
use std::any::Any;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::{
    thread,
    time::Duration,
};
use crate::types::{TransactResult, GetTableRowsPayload, ProposalPayload, SlicePayload, VotePayload, TopPoolPayload, WhiteListPayload};
use crate::io_sys::{DEV_NODEOS_URL};
type Error = reqwest::Error;

#[derive(Default, Clone, Serialize, Debug, Deserialize)]
pub enum AnyType {
    Some,
    None,
    #[default]
    Other
}


#[derive(Default, Debug, Deserialize, Clone)]
pub struct ChainApi {
    pub nodeos_url: String,
    pub contract: String,
    pub fetch: Option<AnyType>,
}

const URL: String = DEV_NODEOS_URL.to_owned() + "/v1/chain/get_table_rows";
pub async fn get_table_rows_with_payload(payload: GetTableRowsPayload) -> Response {

    
    let payload = GetTableRowsPayload {
        json: true,
        code: payload.code,
        scope: payload.scope,
        table: "proposals".into(),
        table_key: payload.table_key,
        lower_bound: payload.lower_bound.into(),
        upper_bound: payload.upper_bound.into(),
        key_type: "i64".into(),
        index_position: "1".into(),
        encode_type: "".into(),
        limit: 0,
        reverse: false,
        show_payer: false
    };

    let resp: Result<Response, reqwest::Error> = reqwest::get(URL)
        .await;
        
        return resp.unwrap();
        
}

pub async fn get_table_rows() -> Response {
    let resp = get(String::from(URL))
        .await;
        
        let value = resp.unwrap();
        value
}

pub async fn get_proposal_by_id(opts: ProposalPayload) -> Response {
    let payload = ProposalPayload {
        id: opts.id,
        contract: opts.contract
    };
    let resp = get_table_rows_with_payload(payload).await;

    resp
    
}


async fn get_proposal_by_contract(opts: ProposalPayload) -> Result<Response, Error> {
    let payload = ProposalPayload {
        id: opts.id,
        contract: opts.contract
    };

let resp = get_table_rows_with_payload(payload).await;

Ok(resp)
}

pub async fn get_slice(opts: SlicePayload) -> Response {

    let payload = SlicePayload {
       date: opts.date
    };
let resp = get_table_rows_with_payload(payload).await;

    resp
}
// async getSlice(opts: SlicePayload): Promise<any> {
//   return this.getTableRows({
//   });
// }
pub async fn get_top_pool(opts: TopPoolPayload) -> Response  {
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
  
    // pub async fn get_table_rows_with_payload(&self, payload: GetTableRowsPayload) -> Response {
    //     let resp: Result<Response, reqwest::Error> = reqwest::get( String::from(&self.nodeos_url) + "/v1/chain/get_table_rows")
    //         .await;
            
    //         return resp.unwrap();
            
    // }

    // pub async fn get_table_rows(&self) -> Response {
    //     let resp = get(String::from(&self.nodeos_url) + "/v1/chain/get_table_rows")
    //         .await;
            
    //         let value = resp.unwrap();
    //         value
    // }

    // pub async fn get_proposal_by_id(self, opts: ProposalPayload<'_>) -> Response {
    //     let payload = GetTableRowsPayload {
    //         json: true,
    //         code: &self.contract,
    //         scope: &self.contract,
    //         table: "proposals".into(),
    //         table_key: Some(opts.id.to_string()),
    //         lower_bound: opts.id.into(),
    //         upper_bound: opts.id.into(),
    //         key_type: "i64".into(),
    //         index_position: "1".into(),
    //         encode_type: "".into(),
    //         limit: 0,
    //         reverse: false,
    //         show_payer: false
    //     };
    //     let resp = self.get_table_rows_with_payload(payload).await;

    //     resp
        
    // }
  

//   async fn get_proposal_by_contract(self, opts: ProposalPayload<'_>) -> Result<Response, Error> {
//     let payload = GetTableRowsPayload {
//         json: true,
//         code: &self.contract,
//         scope: &self.contract,
//         table: "proposals".into(),
//         table_key: Some(opts.contract.into()),
//         lower_bound: opts.contract.into(),
//         upper_bound: opts.contract.into(),
//         key_type: "name".into(),
//         index_position: "2".into(),
//         encode_type: "".into(),
//         limit: 0,
//         reverse: false,
//         show_payer: false

//     };

//     let resp = self.get_table_rows_with_payload(payload).await;

//     Ok(resp)
//   }

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
    pub async fn get_top_pool(opts: TopPoolPayload) -> Response  {
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

    pub async fn get_vote(opts: VotePayload) -> Response  {
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
