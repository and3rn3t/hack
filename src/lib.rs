// Library interface for The Hack: Ghost Protocol
// This allows integration tests to access internal modules

pub mod challenges;
#[cfg(feature = "native")]
pub mod game;
#[cfg(feature = "native")]
pub mod narrative;
pub mod state;
#[cfg(feature = "native")]
pub mod tutorial;
pub mod ui;

// Web-specific modules and exports
#[cfg(feature = "web")]
pub mod web;

#[cfg(feature = "web")]
pub use web::*;
