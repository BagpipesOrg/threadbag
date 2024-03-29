// sled db to store and fetch scenarios
use crate::database::decode::decompress_string;
use crate::database::types::Urldata;
use crate::error::Error as CrateError;
use crate::scenarios::scenario_parse::generate_random_id;
use crate::scenarios::scenario_types::Graph;

use anyhow::Error;
use sled;
use sled::{Db, IVec}; //IVec Tree

use std::str;

#[derive(Debug, Clone)]
pub struct DBhandler {}

/// sled db handler for bagpipes
impl DBhandler {
    /// return a sled::Db instance
    pub fn read_db(&self) -> Result<Db, Error> {
        let open: Db = sled::open("bp.db")?;
        return Ok(open);
    }
    /*
        /// decode the ts encoded blob
        pub async fn decode_entry(&self, input: String) -> Result<String, Error> {
            let outp = decompress_string(input).await?;
            return Ok(outp);
        }
    */
    /// save entry in database
    pub fn saveurl(&self, longurl: Urldata) -> Result<String, Error> {
        let url_data = IVec::from(longurl.url.as_bytes());
        let my_id = generate_random_id();

        let db_instance: Db = self.read_db()?;
        db_instance.insert(my_id.clone(), url_data)?;
        db_instance.flush()?;
        Ok(my_id)
    }
    /// return entry in the db
    pub fn get_entry(&self, key: String) -> Result<String, CrateError> {
        let db: Db = self.read_db()?; //  lots of io usage
        match db.get(key.as_bytes()) {
            Ok(Some(value)) => {
                let outputen: String = String::from_utf8(value.to_vec()).expect("Invalid UTF-8");
                return Ok(outputen);
            }
            _ => return Err(CrateError::NoEntryInDb),
        }
    }
    /// println! db stats
    pub fn display_stats(&self) -> Result<(), CrateError> {
        let db = self.read_db()?;
        let amount_of_entries = count_entries(&db);
        let size = db.size_on_disk()?;
        println!("[Database Checker] - Metadata stats:");
        println!(
            "[Database Checker] - Amount of entries in the database: {:?}",
            amount_of_entries
        );
        println!("[Database Checker] - Size on disk: {:?}", size);

        Ok(())
    }
    /// query for an item and decode it to a Graph
    pub async fn get_decoded_entry(&self, key: String) -> Result<Graph, CrateError> {
        let out = self.get_entry(key)?;
        let decoded = decompress_string(out)
            .await
            .expect("Failed to decompress string, invalid value");

        let graph: Graph = serde_json::from_str(decoded.as_str()).expect("Failed to parse JSON");
        return Ok(graph);
    }

    pub fn new() -> DBhandler {
        return DBhandler {};
    }
}

fn count_entries(db: &Db) -> usize {
    let mut total_entries = 0;

    // Iterate over all entries and count them
    for _ in db.iter() {
        total_entries += 1;
    }

    total_entries
}
