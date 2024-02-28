// generic subxt lib

use subxt::{config::PolkadotConfig, OnlineClient};

#[subxt::subxt(runtime_metadata_url = "wss://rpc.polkadot.io:443")]
pub mod polkadotconf {}

#[subxt::subxt(runtime_metadata_url = "wss://polkadot-asset-hub-rpc.polkadot.io")]
pub mod assethubpolkadotconf {}

#[subxt::subxt(runtime_metadata_url = "wss://hydradx-rpc.dwellir.com")]
pub mod hydradxconf {}

#[subxt::subxt(runtime_metadata_url = "wss://rpc-interlay.luckyfriday.io")]
pub mod interlayconf {}

async fn testtx() {
    // let api =
}
