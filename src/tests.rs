#[cfg(test)]
mod tests {
    use super::*;
    use crate::chains::chains::{chains, get_rpc_endpoint};
    use crate::database::db::DBhandler;
    use crate::database::decode::decompress_string;
    use crate::scenarios::scenario_types::Graph;
    //    use crate::tx_format::generic::{
    //        assethubpolkadotconf, hydradxconf, interlayconf, polkadotconf,
    //   };
    use subxt::{OnlineClient, PolkadotConfig};
    use subxt_signer::sr25519::dev;

    #[test]
    fn it_works() {
        println!("running");
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    /*

        #[actix_rt::test]
        async fn read_db_entries() {
            let mut db_h = DBhandler {};
            let db = db_h.read_db().unwrap();
            let iterme = db.iter();
            println!("Printing values in database");
            for kv in iterme {
                match kv {
                    Ok((key, value)) => {
                        let str_key = String::from_utf8(key.to_vec()).unwrap();
                        println!("String key: {:?}", str_key);
                        let str_val = String::from_utf8(value.to_vec()).unwrap();
                        println!("String value: {:?}", str_val);
                        //let decode_me =

                        /*
                        match decompress_string(str_val).await {
                            Ok(value) => {
                                println!("managed to decode entry");
                            }
                            Err(err) => {
                                println!("could not decode entry, Error: {:?}", err);
                            }
                        }
                                */

                        println!("entry return okay!");
                    }
                    Err(err) => {
                        println!("error reading entry");
                    }
                }
            }
            println!("--.---Reading db entries done---.--");
            println!("test start");
            // valid encoded scenario data
            let inp = "eJzdV9tum0AQ/ZVon1rJpuyFXfBbajdJlVa9xG1TVVG0wBojY0gXiONG/vfOGvCF3JomUdW8wczOzOHMnFn7EqVZqHLU+3GJ4hD1UDCWcXrKfcfBLuqgYn6mGiu8nmV5XMRZinqX6AL1sLBswRnHxBOYYpd20Bz1uszlFmWe63BGhWCMLToolIU0UYn0VQIZ+3XGeCojU8F6taxh5ecRWFM5VatDEJ0X8wQMl/A4yvR0UCerYPUAVjKRYVZApMxzVRhfnaJxDdbet/CdNuScT/3MQBl8GJoaMgy1yoEK5Bx8/zw//+Qdn++XXkr3DwY/Z+/8w6+6nB+R6aT/c/j+eO9w4oCnfxxHeybxNCtTqIswvIQqkcBDWiZJBwVZWsigqF6hzCwOizHqEcY6aKziaAwuxjDgUYkKCgXgRjLJ1ZrsXT/PkrJQf0G6llEUp1GdctHZarJvhzwQdzZZYNdyCOeCYEEZd0hdz+OWR7kniCAe8Wzy1E0ez+FzBhfX9Phjq/2mwchB/2+L70P5DS0GPJD31Kcj7m4IuTJfo2SMPYs6nufaDmVQANclwSyozYSNbWG7zL2my3XOmztYH+ihfCbP0AZBNl4TRAjZJKjQ5S0SuA/aWzVA3ECF7O5FxxxuMepASwX2PC6ahgAMhgm2XY85AOXfaeDL0WC486JRws6uce8clP7LLVFge1MVJubRZFFJ4YlX3z3acLsuXEEDMrpbF9QllkMF5tzlMGkGualIoSKzGSWAhdqOjZ0H6eJiqGWaj5Q2fC6N20eGFcjNY3lW6kDddgnW19zj7bz2Ni2kjqoZvGE8V3v4QRi2yy4214e9sT44+fM5uldXW4N00kEqjK7+ZKpu02577zZ9unLn1uRdWdTNrMCDziYmMtJKpUuPMXyrPx6gTaWeKP0mDc3xZpC1zmbtw3ixGvQ8np4lyle/4uUYNbPaPB6Ns9nrqFm/lakB5MtgEmkYiLCfJZleIVtOdVDm0jfnTOT1t1C3tXJX1Fy5qxpu2gHPkJpqEXVb09GmZr2utqlZBzwnajb/g3TbDLQEtf6n0hLUOuA5UHOy+A1iJVCZ".to_string();
            //   let funkar = decompress_string(inp).await.unwrap(); //will fail if not decode is ok
            //   let comp: String = compress_string("test").await.unwrap();
            // println!("Comp: {:?}", comp);
            let dec: String = decompress_string(inp).await.unwrap();
            println!("Decoded diagram data json as a string");
            let graph: Graph = serde_json::from_str(dec.as_str()).expect("Failed to parse JSON");
            println!("Decoded diagram data to a rust object");
            // verify that it can decompress the compressed data correctly
            //  assert_eq!("test".to_string(), "test");
            println!("decoded: {:?}", dec);

            println!("test done, string decompressed ok");
        }
    */

    #[actix_rt::test]
    async fn polkadot_2_assethub_tx() -> Result<(), Box<dyn std::error::Error>> {
        println!("Connecting..");
        let api =
            OnlineClient::<PolkadotConfig>::from_url(get_rpc_endpoint(chains::Polkadot)).await?;
        println!("polkadot_2_assethub_tx test");
        let dest = dev::bob().public_key().into();
        let tx = polkadotconf::tx()
            .balances()
            .transfer_allow_death(dest, 10_000);
        //      let encoded_tx = tx.encode();

        // Convert the encoded transaction into a hexadecimal string
        //let hex_tx = hex::encode(encoded_tx);
        let unsigned_extrinsic = api.tx().create_unsigned(&tx)?;
        let hex_tx = format!("0x{}", hex::encode(unsigned_extrinsic.encoded()));

        //let hex_bytes = format!("0x{}", hex::encode(unsigned_extrinsic.encoded()));
        //  println!("unsigned extrinsic: {:?}", unsigned_extrinsic.encoded());

        println!("tx is: {:?}", hex_tx);
        Ok(())
    }
}
