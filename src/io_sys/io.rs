pub struct NCInit {
    services: NCInitServices,
    urls: NCInitUrlsDev,
    is_proxy: bool,
    debug: bool,
}

impl Default for NCInit {
    fn default() -> Self {
        let service_val = NCInitServices::default();
        let url_vals = NCInitUrlsDev::default();

        NCInit {
            services: service_val,
            urls: url_vals,
            is_proxy: false,
            debug: true,
        }
    }
}

#[derive(Clone)]

pub struct NCInitUrlsDev {
    pub nodeos_url: String,
    pub hyperion_url: String,
    pub atomicassets_url: String,
    pub nodeos_proxy_url: String,
}
pub struct NCInitUrlsProd {
    pub nodeos_url: String,
    pub hyperion_url: String,
    pub atomicassets_url: String,
    pub nodeos_proxy_url: String,
}

impl Default for NCInitUrlsDev {
    fn default() -> Self {
        Self {
            nodeos_url: String::from("https://nodeos-dev.newcoin.org"),
            hyperion_url: String::from("https://hyperion-dev.newcoin.org"),
            atomicassets_url: String::from("https://atomic-nefty-devnet.newcoin.org/"),
            nodeos_proxy_url: String::from("https://auth-eu-dev.newsafe.org/v1/tx/newcoin"),
        }
    }
}

impl Default for NCInitUrlsProd {
    fn default() -> Self {
        Self {
            nodeos_url: String::from("https://nodeos-dev.newcoin.org"),
            hyperion_url: String::from("https://hyperion-dev.newcoin.org"),
            atomicassets_url: String::from("https://atomic-nefty-devnet.newcoin.org/"),
            nodeos_proxy_url: String::from("https://api.newsafe.org/v1/tx/newcoin"),
        }
    }
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

impl Default for NCInitServices {
    fn default() -> Self {
        Self {
            eosio_contract: String::from("eosio"),
            token_contract: String::from("eosio.token"),
            maindao_contract: String::from("pool.nco"),
            staking_contract: String::from("pools2.nco"),
            daos_contract: String::from("daos2.nco"),
            neftymarket_contract: String::from("market.nefty"),
            atomicassets_contract: String::from("atomicassets"),
        }
    }
}
