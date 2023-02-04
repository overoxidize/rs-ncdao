
pub struct NCInit {
    services: NCInitServices,
    urls: NCInitUrls,
    is_proxy: bool,
    debug: bool
  }

pub struct NCInitUrls {
    nodeos_url: String,
    hyperion_url: String,
    atomicassets_url: String,
    nodeos_proxy_url: String
  }
  
pub struct NCInitServices {
    eosio_contract: String,
    token_contract: String,
    maindao_contract: String,
    staking_contract: String,
    daos_contract: String,
    neftymarket_contract: String,
    atomicassets_contract: String,
  }


pub static DEV_NODEOS_URL: &str = "https://nodeos-dev.newcoin.org";
pub static DEV_HYPERION_URL: &str = "https://hyperion-dev.newcoin.org";
pub static DEV_ATOMICASSETS_URL: &str = "https://atomic-nefty-devnet.newcoin.org/";
pub static DEV_NODEOS_PROXY_URL: &str = "https://auth-eu-dev.newsafe.org/v1/tx/newcoin";
  
pub static P_DEV_NODEOS_URL: &str = "https://nodeos-dev.newcoin.org";
pub static P_DEV_HYPERION_URL: &str = "https://hyperion-dev.newcoin.org";
pub static P_DEV_ATOMIC_NEFTY_URL: &str = "https://hyperion-dev.newcoin.org";
pub static P_DEV_NEWSAFE_URL: &str = "https://api.newsafe.org/v1/tx/newcoin";

  
//   export const devnet_services: NCInitServices =
//   {
//     eosio_contract: "eosio",
//     token_contract: "eosio.token",
//     maindao_contract: "pool.nco",
//     staking_contract: "pools2.nco",
//     daos_contract: "daos2.nco",
//     neftymarket_contract: "market.nefty",
//     atomicassets_contract: "atomicassets"
//   };
  