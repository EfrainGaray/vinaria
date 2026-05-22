//! Vinaria core: bottle manager, recipe runner, process spawner.
//!
//! No Tauri or UI dependencies live here — this crate is a pure-Rust library
//! consumed by the Tauri shell in `src-tauri/`, and could also be reused by
//! a future CLI or alternate frontend.

pub mod bottle;
pub mod recipe;
pub mod spawner;

pub use bottle::{Bottle, BottleId, BottleManager};
pub use recipe::{Recipe, RecipeError, RecipeRunner};
pub use spawner::{ProcessSpawner, SpawnError};
