// sled db to store and fetch scenarios
use crate::database::decode::decompress_string;
use crate::database::types::Urldata;
use crate::error::Error as CrateError;
use crate::scenarios::scenario_parse::generate_random_id;
use crate::scenarios::scenario_types::Graph;
use anyhow::Error;
use bincode::{deserialize, serialize};
use chrono::Utc;
use sled;
use sled::{Db, IVec}; //IVec Tree
use std::str;

#[derive(Debug, Clone)]
pub struct DBhandler {}

fn custom_merge_operator() -> impl Fn(&[u8], Option<&[u8]>, &[u8]) -> Option<Vec<u8>> {
    |_, existing_value, merged_bytes| {
        let mut merged = existing_value.map_or_else(Vec::new, |iv| iv.to_vec());
        merged.extend_from_slice(merged_bytes);
        Some(merged)
    }
}
/// sled db handler for bagpipes
impl DBhandler {
    /// return a sled::Db instance
    pub fn read_db(&self) -> Result<Db, Error> {
        let open: Db = sled::open("bp.db")?;
        // lets define our merging operations
        let merge_result = open.set_merge_operator(custom_merge_operator());
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

    /// save the logs of a thread to a db tree / list
    pub fn save_log(&self, thread_name: String, log_entry: String) -> Result<bool, CrateError> {
        let db = self.read_db()?;
        let utc_time = Utc::now().to_string(); // Convert UTC time to string
        let formated_date = &utc_time[..19]; // Slice the first 19 characters
        let date_n_log = format!("{} {}", formated_date, log_entry);
        let log_name_entry = format!("{}_logs", thread_name);
        let op_bytes = serialize(&date_n_log).unwrap();
        db.merge(log_name_entry.as_bytes(), op_bytes).unwrap();
        // Serialize the values and insert them into the database

        /*

        let tree = db.open_tree(thread_name.clone()).expect("Failed to open tree");

        tree.merge(thread_name, date_n_log)?;
        db.flush()?;3


        */
        Ok(true) // save log to
    }

    /// query the logs of a scenario worker
    pub fn query_logs(&self, thread_name: String) -> Result<Vec<String>, CrateError> {
        println!("query logs called");
        let db = self.read_db()?;
        let log_name_entry = format!("{}_logs", thread_name); // either we get a new db for the logs or just add a prefix
        let outme = String::from_utf8(
            db.get(log_name_entry)?
                .expect("Could not get logs")
                .to_vec(),
        )?;
        //      println!("Raw logs: {:?}", outme);
        /*
                // new line is: %\0\0\0\0\0\0\0 and )\0\0\0\0\0\0\0"
                let logs: Vec<String> = outme
                    .split_terminator("%\0\0\0\0\0\0\0")
                    .flat_map(|s| s.split_terminator(")\0\0\0\0\0\0\0"))
                    .map(|s| s.to_string())
                    .collect();
        */
        let logs: Vec<String> = outme
            .split_terminator("#\0\0\0\0\0\0\0")
            .flat_map(|s| s.split_terminator("&\0\0\0\0\0\0\0"))
            .map(|s| s.to_string())
            .collect();

        //   println!("Got logs: {:?}", logs);
        //      println!("filtered list: {:?}", logs);
        return Ok(logs);
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
