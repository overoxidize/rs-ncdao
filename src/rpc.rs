use crate::submit::AuthorityProviderArgs;
pub struct JsonRpc {
    pub endpoint: String
}

pub struct BinaryAbi {
    account_name: String,
    abi: Vec<u8>,

}

pub fn create_base_64_map(s: String) -> Result<Vec<i8>, ()> {

    let base_64_bytes =  "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes();
    
    let bytes_len = base_64_bytes.len();
    let mut ctr = 0;
    let mut base_64_map: Vec::<i8> = vec![-1; 256];
    while ctr < bytes_len {
        
        for i in 0..257 {
            base_64_map[ctr] = base_64_bytes[ctr] as i8;

            ctr += 1;
        }
    }
    return Ok(base_64_map);


}


impl JsonRpc {

    pub fn get_required_keys(args: AuthorityProviderArgs) -> Option<Vec<String>> {
        unimplemented!()
    }

    pub fn get_raw_abi(account_name: String) -> Option<BinaryAbi> {
        let raw_abi = reqwest::get(account_name);

        
        unimplemented!()
    }
}

















// use eosio::ProducerAuthority;

// use crate::types::{TransactionReceiptHeader, TransactionTrace};




// struct AbiType {
//     new_type_name: String,
//     new_type: String
// }

// struct  AbiStruct {
//     name: String,
//     struct_name: String,
//     base: String,
//     fields: Vec<AbiStructFields>
// }

// struct AbiStructFields {
//     name: String,
//     field_type: String,
// }

// struct AbiActions {
//     name: String,
//     abi_action_type: String,
//     ricardian_contract: String,
// }

// struct AbiTable {
//     table_name: String,
//     table_type: String,
//     index_type: String,
//     key_names: Vec<String>,
//     key_types: Vec<String>
// }

// struct RicardianClauses {
//     clause_id: String,
//     body: String,
// }

// struct ErrorMessages {
//     error_code: i32,
//     error_msg: String,
// }

// struct AbiExtension {
//     tag: i32,
//     value: String,
// }

// struct AbiVariantData {
//     variant_name: String,
//     variant_types: Vec<String>,
// }
// enum AbiVariant {
//     Some(Vec<AbiVariantData>),
//     None
// }

// struct ActionResultData {
//     result_name: String,
//     result_type: String,
// }

// enum ActionResult {
//     Some(Vec<ActionResultData>),
//     None
// }
// struct KvSecondaryIndices {
//     index_type: String
// }
// struct KvPrimaryIndex {
//     name: String,
//     pi_type: String,
// }
// struct KvTableData {
//     kvt_type: String,
//     primary_index: KvPrimaryIndex,
//     secondary_indices: Vec<KvSecondaryIndices>

// }

// enum KvTable {
//     Some(Vec<KvTableData>)
// }
// struct ABI {
//     version: String,
//     types: Vec<AbiType>,
//     structs: Vec<AbiStruct>,
//     actions: Vec<AbiActions>,
//     tables: Vec<AbiTable>,
//     ricardian_clauses: Vec<RicardianClauses>,
//     error_messages: Vec<ErrorMessages>,
//     abi_extensions: Vec<AbiExtension>,
//     variants: Option<AbiVariant>,
//     action_results: Option<ActionResult>,
//     kv_tables: Option<KvTable>


// }

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

// struct Authorization {
//     actor: String,
//     permission: String,
// }