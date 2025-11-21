/// Server-only utilities module
///
/// This module contains helper functions and constants that should only
/// be available on the server. By placing this code in a server_utils module
/// and gating it with #[cfg(feature = "server")], we ensure that sensitive
/// information never leaks to the client.

#[cfg(feature = "server")]
pub mod server_only {
    use std::path::PathBuf;

    /// Get the path to the dogs data file
    pub fn get_dogs_file_path() -> PathBuf {
        PathBuf::from("dogs.txt")
    }

    /// Example: In production, you would place database credentials here
    /// pub static DB_URL: &str = env!("DATABASE_URL");
    /// pub static API_KEY: &str = env!("API_KEY");

    /// Helper function to validate image URLs
    pub fn is_valid_dog_url(url: &str) -> bool {
        // Basic URL validation
        url.starts_with("http://") || url.starts_with("https://")
    }

    /// Log server events (only available on server)
    pub fn log_server_event(event: &str) {
        println!("[SERVER] {}", event);
    }
}
