
pub struct chain_info {
    rpc_endoints: Vec<String>
}


pub enum chains {
    hydradx,
    polkadot,
    interlay,
    assethub
}

/// return the rpc endpoint to use 
pub fn get_rpc_endpoint(){

    let hydradx = vec![
        "wss://hydradx-rpc.dwellir.com",
        "wss://hydradx.api.onfinality.io/public-ws",
        "wss://rpc.hydradx.cloud",
    ];

    let polkadot = vec![
        "wss://polkadot-rpc.dwellir.com",
        "wss://rpc.polkadot.io",
        "wss://polkadot.api.onfinality.io/public-ws",
    ];

    let assethub = vec![
        "wss://polkadot-asset-hub-rpc.polkadot.io",
        "wss://statemint.api.onfinality.io/public-ws",
    ];
}