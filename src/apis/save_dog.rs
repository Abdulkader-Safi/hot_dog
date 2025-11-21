use dioxus::prelude::*;

/// Server function to save a dog image URL to persistent storage
///
/// This endpoint accepts a dog image URL and saves it to a file called `dogs.txt`.
/// In production, you would replace this with a proper database.
///
/// # Security
/// - Validates image URL format before saving
/// - Server-only code is protected by #[cfg(feature = "server")]
#[post("/api/save_dog")]
pub async fn save_dog(image: String) -> Result<(), ServerFnError> {
    // This code only runs on the server
    #[cfg(feature = "server")]
    {
        use crate::apis::server_utils::server_only;
        use std::io::Write;

        // Validate the URL before saving
        if !server_only::is_valid_dog_url(&image) {
            return Err(ServerFnError::new("Invalid image URL format"));
        }

        // Log the server event
        server_only::log_server_event(&format!("Saving dog image: {}", image));

        // Get the file path from server utilities
        let file_path = server_only::get_dogs_file_path();

        // Open the `dogs.txt` file in append-only mode, creating it if it doesn't exist
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)
            .map_err(|e| ServerFnError::new(format!("Failed to open file: {}", e)))?;

        // Write a newline to it with the image URL
        file.write_fmt(format_args!("{image}\n"))
            .map_err(|e| ServerFnError::new(format!("Failed to write to file: {}", e)))?;

        server_only::log_server_event("Dog image saved successfully");
    }

    Ok(())
}
