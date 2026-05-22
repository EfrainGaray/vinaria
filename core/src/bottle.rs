//! Bottle manager: CRUD over Wine prefixes.
//!
//! A bottle is a Wine prefix with metadata: a unique ID, a friendly name, a
//! reference to the Wine build it uses, optional recipe applied, and a
//! launchable executable target.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BottleId(pub uuid::Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottle {
    pub id: BottleId,
    pub name: String,
    pub prefix_path: PathBuf,
    pub wine_path: PathBuf,
    pub recipe: Option<String>,
}

#[derive(Debug, Default)]
pub struct BottleManager {
    // Phase 4 will fill this with persistence (JSON or sqlite).
}

impl BottleManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn list(&self) -> Vec<Bottle> {
        // TODO(phase-4): scan ~/.vinaria/bottles/
        Vec::new()
    }
}
