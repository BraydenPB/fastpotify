//! Spotify Web API client.

pub mod client;
pub mod models;

pub use client::{ApiClient, ApiError, NetActivity, PlayRequest, TokenProvider, WebTokens};
