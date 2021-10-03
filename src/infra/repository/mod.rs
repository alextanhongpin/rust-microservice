pub mod db;
mod repository;
mod user_repository;

// Re-export is to it can be called as crate::infra::UserRepository;
pub use repository::Repository;
pub use user_repository::UserRepository;
