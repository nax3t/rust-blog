use std::{fs, sync::Mutex};
use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    static ref TEST_DBS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub fn register_test_db(db_name: String) {
    let mut dbs = TEST_DBS.lock().unwrap();
    dbs.insert(db_name);
}

#[ctor::dtor]
fn cleanup_test_dbs() {
    let mut dbs = TEST_DBS.lock().unwrap();
    for db_name in dbs.iter() {
        fs::remove_file(db_name).ok();
    }
    dbs.clear();
}
