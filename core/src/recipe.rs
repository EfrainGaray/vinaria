//! Recipe runner: declarative bottle configuration loaded from TOML.
//!
//! A recipe describes how to set up a bottle for a specific Windows app:
//! Wine version, DLL overrides, registry tweaks, runtime files, env vars,
//! and the launch command. Recipes live in the `recipes/` directory of the
//! main repo (community-contributed) or in `~/.vinaria/recipes/` (user-local).

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecipeError {
    #[error("recipe parse failed: {0}")]
    Parse(#[from] toml::de::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub steam_app_id: Option<u32>,
    pub windows_version: Option<String>,
    #[serde(default)]
    pub dll_overrides: std::collections::BTreeMap<String, String>,
    #[serde(default)]
    pub registry: Vec<RegistryEntry>,
    pub launch: LaunchSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub key: String,
    pub name: String,
    pub value: toml::Value,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchSpec {
    pub executable: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: std::collections::BTreeMap<String, String>,
}

#[derive(Debug, Default)]
pub struct RecipeRunner;

impl RecipeRunner {
    pub fn parse(input: &str) -> Result<Recipe, RecipeError> {
        Ok(toml::from_str(input)?)
    }
}
