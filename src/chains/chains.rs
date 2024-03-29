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
