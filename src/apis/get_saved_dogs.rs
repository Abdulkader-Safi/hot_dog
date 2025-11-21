use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a saved dog image from the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedDog {
    pub id: i64,
    pub url: String,
    pub created_at: String,
}

/// Server function to retrieve all saved dog images from the database
///
/// Returns a list of all dog images that have been saved, ordered by most recent first.
///
/// # Security
/// - Server-only code is protected by #[cfg(feature = "server")]
/// - Read-only operation with no user input validation needed
#[get("/api/get_saved_dogs")]
pub async fn get_saved_dogs() -> Result<Vec<SavedDog>, ServerFnError> {
    // This code only runs on the server
    #[cfg(feature = "server")]
    {
        use crate::apis::server_utils::server_only;

        server_only::log_server_event("Retrieving saved dog images from database");

        // Query all dogs from the database
        let dogs = server_only::DB.with(|db| {
            let conn = db.borrow();
            let mut stmt = conn
                .prepare("SELECT id, url, created_at FROM dogs ORDER BY created_at DESC")
                .map_err(|e| ServerFnError::new(format!("Failed to prepare statement: {}", e)))?;

            let dogs = stmt
                .query_map([], |row| {
                    Ok(SavedDog {
                        id: row.get(0)?,
                        url: row.get(1)?,
                        created_at: row.get(2)?,
                    })
                })
                .map_err(|e| ServerFnError::new(format!("Failed to query dogs: {}", e)))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| ServerFnError::new(format!("Failed to collect results: {}", e)))?;

            Ok::<Vec<SavedDog>, ServerFnError>(dogs)
        })?;

        server_only::log_server_event(&format!("Retrieved {} saved dog images", dogs.len()));

        return Ok(dogs);
    }

    #[cfg(not(feature = "server"))]
    {
        // This should never be reached on client, but include for completeness
        Ok(vec![])
    }
}
