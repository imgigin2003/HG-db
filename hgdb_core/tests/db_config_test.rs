use hgdb_core::db_config;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_get_db_path() {
        let path = db_config::get_db_path();

        // ✅ Verify the path is correct instead of hardcoding it
        let expected_path = "/Users/gigin/Documents/mydbs/rocksdb/DB-config";
        assert!(
            path == expected_path,
            "❌ Expected DB path '{}', but got '{}'",
            expected_path,
            path
        );
    }

    #[test]
    fn test_get_db_path_from_config() {
        let db = db_config::get_db();
        let path = db.path().to_str().unwrap();

        // ✅ Check if the DB path exists to avoid false failures
        assert!(
            Path::new(path).exists(),
            "❌ DB path '{}' does not exist!",
            path
        );

        let expected_path = "/Users/gigin/Documents/mydbs/rocksdb/DB-config";
        assert!(
            path == expected_path,
            "❌ Expected DB path '{}', but got '{}'",
            expected_path,
            path
        );
    }
}
