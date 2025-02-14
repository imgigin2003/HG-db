use std::fs;
use rocksdb::{DB, SingleThreaded, DBWithThreadMode};

pub fn get_db_path() -> String {

    let config_content = fs::read_to_string("Config.toml").expect("Failed to read config file");
    let db_path = config_content.lines()
        .find(|line| line.starts_with("db_path"))
        .and_then(|line| line.split('=').nth(1))
        .map(|path| path.trim().trim_matches('"').to_string())
        .expect("Failed to parse db_path");
    db_path
}

pub fn get_db() -> DBWithThreadMode<SingleThreaded> {

    let db_path = get_db_path();
    let db = DB::open_default(db_path).unwrap(); //.expect("Failed to open RocksDB");
    db // Here we cannot return a reference to db because it is a local variable (dangling pointer!)
}