use std::collections::HashMap;

pub struct chain_info {
    rpc_endoints: Vec<String>,
}

/// enum of supported chains
pub enum chains {
    Hydradx,
    Polkadot,
    Interlay,
    Assethub,
}


pub struct asset_decimal_info {
    chain: String,
    asset_decimals: u64, 
}


#[derive(Debug)]
struct AssetChainInfo {
    name: String,
    paraid: u64,
    prefix: u64,
    token_decimals: u64,
    relay_parent: String,
    parachain: bool,
    relay: bool,
}

pub fn list_chains() -> HashMap<u64, AssetChainInfo> {
    let mut chain_list = HashMap::new();

    let asset_hub = AssetChainInfo {
        name: "assetHub".to_string(),
        paraid: 1000,
        prefix: 63,
        token_decimals: 10,
        relay_parent: "polkadot".to_string(),
        parachain: true,
        relay: false,
    };
    chain_list.insert(1000, asset_hub);

    // Define other ChainInfo structs and insert them into the chain_list HashMap

    let hydradx: AssetChainInfo = AssetChainInfo {
        name: "hydraDx".to_string(),
        paraid: 2034,
        prefix: 0,
        token_decimals: 12,
        relay_parent: "polkadot".to_string(),
        parachain: true,
        relay: false,
    };
    chain_list.insert(2034, hydradx);

    let KusamaAssethub: AssetChainInfo = AssetChainInfo {
        name: "kusama_assethub".to_string(),
        paraid: 3000,
        prefix: 42,
        token_decimals: 12,
        relay_parent: "kusama".to_string(),
        parachain: true,
        relay: false,
    };
    chain_list.insert(3000, KusamaAssethub);


    let RococoAssethub: AssetChainInfo = AssetChainInfo {
        name: "rococo_assethub".to_string(),
        paraid: 3000,
        prefix: 42,
        token_decimals: 12,
        relay_parent: "rococo".to_string(),
        parachain: true,
        relay: false,
    };
    chain_list.insert(30009, RococoAssethub);

    let polkadot: AssetChainInfo = AssetChainInfo {
        name: "polkadot".to_string(),
        paraid: 0,
        prefix: 0,
        token_decimals: 10,
        relay_parent: "polkadot".to_string(),
        parachain: false,
        relay: true,
    };
    chain_list.insert(10, polkadot);

    let Turing: AssetChainInfo = AssetChainInfo {
        name: "turing".to_string(),
        paraid: 2114,
        prefix: 51,
        token_decimals: 10,
        relay_parent: "kusama".to_string(),
        parachain: true,
        relay: false,
    };
    chain_list.insert(2114, Turing);

    let Moonriver: AssetChainInfo = AssetChainInfo {
        name: "moonriver".to_string(),
        paraid: 2023,
        prefix: 42,
        token_decimals: 18,
        relay_parent: "kusama".to_string(),
        parachain: true,
        relay: false,
    };
    chain_list.insert(2023, Moonriver);

    let MangataX: AssetChainInfo = AssetChainInfo {
        name: "mangatax".to_string(),
        paraid: 2110,
        prefix: 42,
        token_decimals: 12,
        relay_parent: "kusama".to_string(),
        parachain: true,
        relay: false,
    };
    chain_list.insert(21108, MangataX);


    let Interlay: AssetChainInfo = AssetChainInfo {
        name: "interlay".to_string(),
        paraid: 2032,
        prefix: 0,
        token_decimals: 12,
        relay_parent: "polkadot".to_string(),
        parachain: true,
        relay: false,
    };
    chain_list.insert(2032, Interlay);




    // Return the HashMap
    chain_list
}


pub fn  get_token_decimals_by_chain_name(chain_name: &str) -> u64 {
    let chain_list = list_chains();
    let selected_chain = chain_list.values().find(|&chain| chain.name == chain_name);

    match selected_chain {
        Some(chain) => chain.token_decimals,
        None => 10, // return default 10
    }
}

pub fn get_asset_decimals_for_chain(chain: String) {
//    match chain {

  //  }

}


/// return the rpc endpoint to use
pub fn get_rpc_endpoint(chain_select: chains) -> String {
    let _hydradx = vec![
        "wss://hydradx-rpc.dwellir.com",
        "wss://hydradx.api.onfinality.io/public-ws",
        "wss://rpc.hydradx.cloud",
    ];

    let _polkadot = vec![
        "wss://polkadot-rpc.dwellir.com",
        "wss://rpc.polkadot.io",
        "wss://polkadot.api.onfinality.io/public-ws",
    ];

    let _assethub = vec![
        "wss://polkadot-asset-hub-rpc.polkadot.io",
        "wss://statemint.api.onfinality.io/public-ws",
    ];

    match chain_select {
        chains::Polkadot => return "wss://polkadot-rpc.dwellir.com:443".to_string(),
        chains::Hydradx => return "wss://hydradx-rpc.dwellir.com:443".to_string(),
        chains::Interlay => return "wss://interlay-rpc.dwellir.com".to_string(),
        chains::Assethub => return "wss://polkadot-asset-hub-rpc.polkadot.io:443".to_string(),
    }
}
