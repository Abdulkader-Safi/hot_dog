pub mod get_saved_dogs;
pub mod save_dog;
pub mod server_utils;

// Re-export server functions for easy access
pub use get_saved_dogs::get_saved_dogs;
pub use save_dog::save_dog;
