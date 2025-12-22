use std::fs;
use rocksdb::{DB, SingleThreaded, DBWithThreadMode};

pub const BASE_DB_PATH: &str = "/Users/gigin/Documents/mydbs/rocksdb/";

pub fn get_db_path() -> String {
    // Check for a test-specific subfolder override via environment variable
    let base_path = match fs::read_to_string("Config.toml") {
        Ok(config_content) => {
            config_content
                .lines()
                .find(|line| line.starts_with("db_path"))
                .and_then(|line| line.split('=').nth(1))
                .map(|path| path.trim().trim_matches('"').to_string())
                .unwrap_or_else(|| BASE_DB_PATH.to_string())
        }
        Err(_) => BASE_DB_PATH.to_string(),
    };

    // Append subfolder if specified by environment variable
    if let Ok(subfolder) = std::env::var("DB_SUBFOLDER") {
        format!("{}/{}", base_path, subfolder)
    } else {
        base_path
    }
}

pub fn get_db() -> DBWithThreadMode<SingleThreaded> {
    let db_path = get_db_path();
    let db = DB::open_default(&db_path).unwrap();
    db
}