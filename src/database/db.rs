// sled db to store and fetch scenarios
use crate::database::types::Urldata;
use crate::error::Error as CrateError;
use crate::scenarios::scenario_parse::generate_random_id;
use anyhow::Error;
use sled;
use sled::{Db, IVec}; //IVec Tree

use std::str;

#[derive(Debug, Clone)]
pub struct DBhandler {}

/// sled db handler
impl DBhandler {
    pub fn read_db(&self) -> Result<Db, Error> {
        let open: Db = sled::open("bp.db")?;
        return Ok(open);
    }

    pub fn saveurl(&self, longurl: Urldata) -> Result<String, Error> {
        let url_data = IVec::from(longurl.url.as_bytes());
        let my_id = generate_random_id();

        let db_instance: Db = self.read_db()?;
        db_instance.insert(my_id.clone(), url_data)?;

        Ok(my_id)
    }

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
}
