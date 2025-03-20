use hgdb_core::db_config;
use std::error::Error;
use std::fs::remove_dir_all;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

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
        }
        Ok(())
    }

    #[test]
    fn test_get_db_path() -> Result<(), Box<dyn Error>> {
        // Clean up before running the test
        cleanup_db()?;

        let path = db_config::get_db_path();

        // Verify the path matches the expected value
        assert_eq!(
            path, DB_PATH,
            "❌ Expected DB path '{}', but got '{}'",
            DB_PATH, path
        );

        Ok(())
    }

    #[test]
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
        assert!(
            Path::new(path).exists(),
            "❌ DB path '{}' does not exist after initialization!",
            path
        );

        // Verify the path matches the expected value
        assert_eq!(
            path, DB_PATH,
            "❌ Expected DB path '{}', but got '{}'",
            DB_PATH, path
        );

        Ok(())
    }
}