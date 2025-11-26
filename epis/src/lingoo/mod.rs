//! Lingoo module
//!
//! This module provides the language learning assistant functionality,
//! offering interactive conversations for language acquisition.

pub mod adapters;
pub mod handlers;
#[allow(clippy::module_inception)]
pub mod lingoo;
pub mod models;
pub mod repository;
pub mod router;
