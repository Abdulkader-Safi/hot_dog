/// Server-only utilities module
///
/// This module contains helper functions and constants that should only
/// be available on the server. By placing this code in a server_utils module
/// and gating it with #[cfg(feature = "server")], we ensure that sensitive
/// information never leaks to the client.

#[cfg(feature = "server")]
pub mod server_only {
    use rusqlite::Connection;
    use std::cell::RefCell;

    // Thread-local database connection
    //
    // SQLite connections are not thread-safe, so we use thread_local
    // to ensure each thread has its own connection.
    thread_local! {
        pub static DB: RefCell<Connection> = RefCell::new({
            // Open the database from the persisted "hotdog.db" file
            let conn = Connection::open("hotdog.db")
                .expect("Failed to open database");

            // Create the "dogs" table if it doesn't already exist
            conn.execute(
                "CREATE TABLE IF NOT EXISTS dogs (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    url TEXT NOT NULL UNIQUE,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                )",
                [],
            )
            .expect("Failed to create dogs table");

            log_server_event("Database initialized successfully");

            // Return the connection
            conn
        });
    }

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
