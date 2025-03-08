use std::fs; // for reading the config file
use rocksdb::{DB, SingleThreaded, DBWithThreadMode}; // for database operations

pub fn get_db_path() -> String {

    // Read the config file and extract the db_path
    // Assuming the config file is named "Config.toml" and is in the same directory as the executable
    let config_content = fs::read_to_string("Config.toml").expect("Failed to read config file");
    // Parse the config file to find the db_path
    let db_path = config_content.lines()
        .find(|line| line.starts_with("db_path"))
        .and_then(|line| line.split('=').nth(1))
        .map(|path| path.trim().trim_matches('"').to_string())
        .expect("Failed to parse db_path");
    db_path
}

// This function returns a reference to the database, which is a local variable and will be dropped when the function returns
pub fn get_db() -> DBWithThreadMode<SingleThreaded> {

    // To avoid this, we can use a global variable or a thread-local variable to store the database instance
    let db_path = get_db_path();
    // Create a new database instance with the specified path
    let db = DB::open_default(db_path).unwrap(); //.expect("Failed to open RocksDB");
    db // Here we cannot return a reference to db because it is a local variable (dangling pointer!)
}