<<<<<<< HEAD
use hgdb_core::db_config::BASE_DB_PATH;
=======
use hgdb_core::db_config;
use std::error::Error;
>>>>>>> parent of 6c9614f (Revert "Fixed methods")
use std::fs::remove_dir_all;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

<<<<<<< HEAD
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
=======
    // Constant for the expected database path
    const DB_PATH: &str = "/Users/gigin/Documents/mydbs/rocksdb/DB-config";

    // Helper function to clean up the database directory
    fn cleanup_db() -> Result<(), Box<dyn Error>> {
        if let Err(e) = remove_dir_all(DB_PATH) {
            if e.kind() != std::io::ErrorKind::NotFound {
                return Err(format!("❌ Failed to remove DB directory '{}': {:?}", DB_PATH, e).into());
            }
            println!("ℹ️ DB directory '{}' not found, proceeding with clean state", DB_PATH);
        } else {
            println!("🧹 Successfully removed DB directory '{}'", DB_PATH);
>>>>>>> parent of 6c9614f (Revert "Fixed methods")
        }
        Ok(())
    }

    #[test]
<<<<<<< HEAD
    fn test_get_db_path() {
        // Set the environment variable for this test
        std::env::set_var("DB_SUBFOLDER", SUBFOLDER);
        cleanup_db().expect("Failed to clean up DB directory");
        let path = hgdb_core::db_config::get_db_path(); // Fully qualified to avoid ambiguity
        let expected_path = DB_PATH.as_str();
        assert!(
            path == expected_path,
=======
    fn test_get_db_path() -> Result<(), Box<dyn Error>> {
        // Clean up before running the test
        cleanup_db()?;

        let path = db_config::get_db_path();

        // Verify the path matches the expected value
        assert_eq!(
            path, DB_PATH,
>>>>>>> parent of 6c9614f (Revert "Fixed methods")
            "❌ Expected DB path '{}', but got '{}'",
            DB_PATH, path
        );

        Ok(())
    }

    #[test]
<<<<<<< HEAD
    fn test_get_db_path_from_config() {
        // Set the environment variable for this test
        std::env::set_var("DB_SUBFOLDER", SUBFOLDER);
        cleanup_db().expect("Failed to clean up DB directory");
        let db = hgdb_core::db_config::get_db(); // Fully qualified to avoid ambiguity
        let path = db.path().to_str().unwrap();
=======
    fn test_get_db_path_from_config() -> Result<(), Box<dyn Error>> {
        // Clean up before running the test
        cleanup_db()?;

        let db = db_config::get_db();
        let path = db.path().to_str().ok_or_else(|| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "❌ DB path could not be converted to string",
            )) as Box<dyn Error>
        })?;

        // Check if the DB path exists after initialization
>>>>>>> parent of 6c9614f (Revert "Fixed methods")
        assert!(
            Path::new(path).exists(),
            "❌ DB path '{}' does not exist after initialization!",
            path
        );
<<<<<<< HEAD
        let expected_path = DB_PATH.as_str();
        assert!(
            path == expected_path,
=======

        // Verify the path matches the expected value
        assert_eq!(
            path, DB_PATH,
>>>>>>> parent of 6c9614f (Revert "Fixed methods")
            "❌ Expected DB path '{}', but got '{}'",
            DB_PATH, path
        );

        Ok(())
    }
}