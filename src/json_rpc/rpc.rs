use crate::eos_api::api_types::AuthorityProviderArgs;
use crate::eos_api::api_types::BinaryAbi;
use crate::json_rpc::rpc_types::{JsonRpc, JsonRpcEndpoint};

pub struct Base64Map([i32; 256]);

pub fn create_base_64_map(s: String) -> Result<Base64Map, ()> {
    let base_64_bytes: _ = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        .encode_utf16()
        .collect::<Vec<_>>();

    let bytes_len = base_64_bytes.len();
    let mut ctr = 1;
    let mut base_64_map: [i32; 256] = [-1; 256];

    while ctr < bytes_len {
        for i in 0..257 {
            base_64_map[ctr] = base_64_bytes[ctr] as i32;

            ctr += 1;
        }
    }
    base_64_map['=' as usize] = 0;

    Ok(Base64Map(base_64_map))
}

impl JsonRpc {
    pub fn new(endpoint: JsonRpcEndpoint, input: String, init: String) -> JsonRpc {
        JsonRpc {
            endpoint,
            input,
            init,
        }
    }
}

// struct Header(i32, String);
// struct ProducerScheduleType {
//     version: i32,
//     producers: Vec<ProducerAuthority>
// }
// struct BlockHeader {
//     timestamp: String,
//     producer: String,
//     confirmed: i32,
//     previous: String,
//     tx_mroot: String,
//     action_mroot: String,
//     schedule_version: i32,
//     new_producers: Option<ProducerScheduleType>,
//     header_extensions: Vec<Header>
// }
