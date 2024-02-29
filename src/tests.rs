#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::db::DBhandler;

    #[test]
    fn it_works() {
        println!("running");
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn read_db_entries() {
        let mut db_h = DBhandler {};
        let db = db_h.read_db().unwrap();
        let iterme = db.iter();
        println!("Printing values in database");
        for kv in iterme {
            match kv {
                Ok((key, value)) => {
                    let str_key = String::from_utf8(key.to_vec());
                    println!("String key: {:?}", str_key);
                    let str_val = String::from_utf8(value.to_vec());
                    println!("String value: {:?}", str_val);
                    println!("entry return okay!");
                }
                Err(err) => {
                    println!("error reading entry");
                }
            }
        }
        println!("--.---Reading db entries done---.--");
    }
}
