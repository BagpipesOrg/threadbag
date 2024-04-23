// xcm transaction generation
// todo cache the connections
use crate::chains::chains::{chains, get_rpc_endpoint};
//use crate::tx_format::generic::{assethubpolkadotconf, polkadotconf};
use crate::error::Error;
use hex::FromHex;
use subxt::{utils::AccountId32, OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;

//use crate::tx_format::generic::{polkadotconf, assethubpolkadotconf, hydradxconf, interlayconf};
//use crate::tx_format::generic::{assethubpolkadotrt, AssetHubConfig};

async fn dot2assethub() {}

/// call this for all xcm tx from assethub to other chain
pub struct AssethubTx {
    // <T: subxt::Config>
    //  client: OnlineClient<T>,
}

impl AssethubTx {
    // <T: subxt::Config>

    /*
        async fn new() -> Result<Self, Error> {
            let api =
                OnlineClient::<PolkadotConfig>::from_url(get_rpc_endpoint(chains::Polkadot)).await?;
            AssethubTx { api }
        }
    */

    /// create a xcm asset transfer to destination chain
    pub async fn xcm_tx(
        &self,
        dest_chain: chains,
        amount: u128,
        assetid: u128,
        dest_account: String,
    ) -> Result<(), Error> {
        //       let api =
        //          OnlineClient::<AssetHubConfig>::from_url(get_rpc_endpoint(chains::Assethub)).await?;

        match dest_chain {
            chains::Polkadot => {}
            chains::Hydradx => {}

            /// xcm v2
            chains::Interlay => {
                //            use assethubpolkadotrt::runtime_types::xcm::v2::junction::Junction::{
                //                 GeneralIndex, GeneralKey, Parachain,
                //             };
                //               use assethubpolkadotrt::runtime_types::xcm::v2::multilocation::Junctions::{
                //                  X1, X2,
                //              };
                //           use assethubpolkadotrt::runtime_types::xcm::v3::WeightLimit;
                //          use assethubpolkadotrt::runtime_types::xcm::VersionedMultiAssets;
                //    use assethubpolkadotrt::runtime_types::xcm::VersionedMultiLocation;
                //  use assethubpolkadotrt::runtime_types::staging_xcm::v2::multilocation::MultiLocation;
                // use assethubpolkadotrt::runtime_types::xcm::v2::junction::Junction::Accountid32;
                //         use assethubpolkadotrt::runtime_types::xcm::v2::multiasset::Fungibility;

                // send to this parachain
                /*
                                let  dest =  VersionedMultiLocation::V3(MultiLocation {
                                    parents: 1,
                                    interior: X1(Parachain(2032)), // todo get the paraid from the core
                                });
                                let d_ac =  AccountId32::from_str("5Eg2fntJ27qsari4FGrGhrMqKFDRnkNSR6UshkZYBGXmSuC8")
                                    .expect("Sovereign account identifier is valid");
                              //  let mydest = VersionedMultiLocation;


                                let benef =     VersionedMultiLocation::V3(MultiLocation {
                                    parents: 0,
                                    interior: X2( AccountId32 { d_ac }),
                                });

                                let assets =           VersionedMultiAssets::V3(MultiAssets(vec![MultiAsset {
                                    id: AssetId::Concrete(MultiLocation {
                                        parents: 0,
                                        interior: X2(PalletInstance(50),
                                        GeneralIndex(assetid)),
                                    }),
                                    fun: Fungibility::Fungible(amount),
                                }]));


                                let tx = assethubpolkadotrt::tx()
                                .polkadot_xcm()
                                .limited_reserve_transfer_assets(
                                    dest,
                                    benef, //beneficiary:
                                    assets,// assets:
                                    0,
                                    WeightLimit::Unlimited, // fee_asset_item
                                 //   { V2: destination },
                                 //     { V2: account },
                                 //     { V2: [asset] },
                              //        { Unlimited: None }
                                    );

                //

                                let unsigned_extrinsic = api.tx().create_unsigned(&tx)?;
                                let hex_tx = format!("0x{}", hex::encode(unsigned_extrinsic.encoded()));

                                //return hex_tx;
                            }
                            _ => {
                                // unsupported chains
                                return Err(Error::InvalidDestinationChain);
                            }
                        }
                */
            }

            _ => return Err(Error::UnsupportedDestinationChain),
        }
        Ok(())
    }
}
