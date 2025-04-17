//! # Deriv API Client Library
//!
//! Provides asynchronous WebSocket connection and API call wrappers
//! for the Deriv API.

pub mod api;
pub mod client;
pub mod error;
pub mod subscription;
pub mod types;
mod utils;

pub use client::DerivClient;
pub use error::{DerivError, Result};

// Potentially re-export schema types if needed
pub use deriv_api_schema as schema;
