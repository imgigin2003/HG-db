use hgdb_core::db_config;

#[cfg(test)]
mod tests {
    use super::*;  // Import everything from the parent module (lib.rs).

    #[test]
    fn test_get_db_path() {
        let path = db_config::get_db_path();
        assert_eq!(path, "/Users/gigin/Documents/mydbs/rocksdb/data");
    }

    #[test]
    fn test_get_db_path_from_confi() {
        let db = db_config::get_db();
        let path = db.path().to_str().unwrap();
        assert_eq!(path, "/Users/gigin/Documents/mydbs/rocksdb/data");
    }
}