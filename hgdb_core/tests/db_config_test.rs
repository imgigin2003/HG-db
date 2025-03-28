use hgdb_core::db_config::BASE_DB_PATH;
use std::fs::remove_dir_all;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    const SUBFOLDER: &str = "db-config";
    lazy_static::lazy_static! {
        static ref DB_PATH: String = format!("{}/{}", BASE_DB_PATH, SUBFOLDER);
    }

    // Helper function to clean up the database directory
    fn cleanup_db() -> Result<(), Box<dyn std::error::Error>> {
        if let Err(e) = remove_dir_all(DB_PATH.as_str()) {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(format!("❌ Failed to remove DB directory '{}': {:?}", DB_PATH.as_str(), e).into());
            }
            println!("ℹ️ DB directory '{}' not found, proceeding with clean state", DB_PATH.as_str());
        } else {
            println!("🧹 Successfully removed DB directory '{}'", DB_PATH.as_str());
        }
        Ok(())
    }

    #[test]
    fn test_get_db_path() {
        // Set the environment variable for this test
        std::env::set_var("DB_SUBFOLDER", SUBFOLDER);
        cleanup_db().expect("Failed to clean up DB directory");
        let path = hgdb_core::db_config::get_db_path(); // Fully qualified to avoid ambiguity
        let expected_path = DB_PATH.as_str();
        assert!(
            path == expected_path,
            "❌ Expected DB path '{}', but got '{}'",
            DB_PATH.as_str(), path
        );
    }

    #[test]
    fn test_get_db_path_from_config() {
        // Set the environment variable for this test
        std::env::set_var("DB_SUBFOLDER", SUBFOLDER);
        cleanup_db().expect("Failed to clean up DB directory");
        let db = hgdb_core::db_config::get_db(); // Fully qualified to avoid ambiguity
        let path = db.path().to_str().unwrap();
        assert!(
            Path::new(path).exists(),
            "❌ DB path '{}' does not exist after initialization!",
            path
        );
        let expected_path = DB_PATH.as_str();
        assert!(
            path == expected_path,
            "❌ Expected DB path '{}', but got '{}'",
            DB_PATH.as_str(), path
        );
    }
}