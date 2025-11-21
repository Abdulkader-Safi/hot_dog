use dioxus::prelude::*;

/// Server function to save a dog image URL to persistent storage
///
/// This endpoint accepts a dog image URL and saves it to a SQLite database.
///
/// # Security
/// - Validates image URL format before saving
/// - Server-only code is protected by #[cfg(feature = "server")]
/// - Uses prepared statements to prevent SQL injection
#[post("/api/save_dog")]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    // This code only runs on the server
    #[cfg(feature = "server")]
    {
        use crate::apis::server_utils::server_only;

        // Validate the URL before saving
        if !server_only::is_valid_dog_url(&image) {
            return Err(ServerFnError::new("Invalid image URL format"));
        }

        // Log the server event
        server_only::log_server_event(&format!("Saving dog image: {}", image));

        // Insert the dog image URL into the database
        let result = server_only::DB.with(|db| {
            db.borrow()
                .execute("INSERT INTO dogs (url) VALUES (?1)", [&image])
                .map_err(|e| {
                    // Log the error for debugging
                    server_only::log_server_event(&format!("Database error: {}", e));

                    // Handle duplicate URLs gracefully
                    if e.to_string().contains("UNIQUE constraint failed") {
                        ServerFnError::new("This dog image has already been saved")
                    } else {
                        ServerFnError::new(format!("Database error: {}", e))
                    }
                })
        });

        result?;

        server_only::log_server_event("Dog image saved successfully to database");
    }

    Ok(())
}
