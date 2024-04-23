// generic subxt lib
use crate::chains::chains::{chains, get_rpc_endpoint};
use subxt::{
    config::{Config, DefaultExtrinsicParams, PolkadotConfig},
    OnlineClient, SubstrateConfig,
};


/*

/// Polkadot's runtime types
#[subxt::subxt(
    runtime_metadata_insecure_url = "wss://polkadot-rpc.dwellir.com:443",
    derive_for_all_types = "PartialEq, Clone"
)]
pub mod polkadotrt {}

/// Polkadot Assethub runtime types
#[subxt::subxt(
    runtime_metadata_insecure_url = "wss://polkadot-asset-hub-rpc.polkadot.io:443",
    derive_for_all_types = "PartialEq, Clone"
)]
pub mod assethubpolkadotrt {}
use assethubpolkadotrt::runtime_types::xcm::v2::multilocation::MultiLocation as Assethub_MultiLocation;

pub enum AssetHubConfig {}

impl Config for AssetHubConfig {
    type Hash = <SubstrateConfig as Config>::Hash;
    type AccountId = <SubstrateConfig as Config>::AccountId;
    type Address = <PolkadotConfig as Config>::Address;
    type Signature = <SubstrateConfig as Config>::Signature;
    type Hasher = <SubstrateConfig as Config>::Hasher;
    type Header = <SubstrateConfig as Config>::Header;
    type ExtrinsicParams = DefaultExtrinsicParams<AssetHubConfig>;
    // Here we use the MultiLocation from the metadata as a part of the config:
    // The `ChargeAssetTxPayment` signed extension that is part of the ExtrinsicParams above, now uses the type:
    type AssetId = Assethub_MultiLocation;
}

#[subxt::subxt(runtime_metadata_insecure_url = "wss://hydradx-rpc.dwellir.com")]
pub mod hydradxrt {}

#[subxt::subxt(runtime_metadata_insecure_url = "wss://rpc-interlay.luckyfriday.io")]
pub mod interlayrt {}

async fn testtx() {
    // let api =
}

//pub fn tx_to_hex()


*/