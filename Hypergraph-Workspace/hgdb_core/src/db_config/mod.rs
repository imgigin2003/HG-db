use std::fs;
use serde::Deserialize;

pub const BASE_DB_PATH: &str = "/Users/gigin/Documents/mydbs/rocksdb/";

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    pub db_path: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct LayeredServiceConfig {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct RocksDBConfig {
    pub create_if_missing: bool,
    pub optimize_for_point_lookups: bool,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub api: ApiConfig,
    pub layered_service: LayeredServiceConfig,
    pub rocksdb: RocksDBConfig,
}

pub fn get_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    match fs::read_to_string("Config.toml") {
        Ok(config_content) => {
            let config: AppConfig = toml::from_str(&config_content)?;
            Ok(config)
        }
        Err(e) => {
            // If config file doesn't exist, use default values
            eprintln!("Config file not found, using defaults: {}", e);
            Ok(AppConfig {
                database: DatabaseConfig {
                    db_path: BASE_DB_PATH.to_string(),
                },
                api: ApiConfig {
                    host: "127.0.0.1".to_string(),
                    port: 8080,
                },
                layered_service: LayeredServiceConfig {
                    url: "http://localhost:3000".to_string(),
                },
                rocksdb: RocksDBConfig {
                    create_if_missing: true,
                    optimize_for_point_lookups: true,
                },
            })
        }
    }
}

pub fn get_db() -> rocksdb::DB {
    let config = get_config().expect("Failed to load config");
    let db_path = config.database.db_path;
    
    let mut opts = rocksdb::Options::default();
    opts.create_if_missing(true);
    opts.optimize_for_point_lookup(1024);
    
    rocksdb::DB::open(&opts, db_path).expect("Failed to open database")
}