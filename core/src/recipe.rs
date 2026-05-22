//! Recipe runner: declarative bottle configuration loaded from TOML.
//!
//! A recipe describes how to set up a bottle for one specific Windows app:
//! optional Windows version, registry tweaks, env vars for launch, and the
//! launch command. Recipes live in `recipes/` of this repo (the curated
//! starter set) or in `~/.vinaria/recipes/` (user-local overrides).
//!
//! Parsing is the only thing this module knows how to do. The Tauri shell
//! decides when to apply a recipe to a bottle.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;
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
    #[serde(default)]
    pub steam_app_id: Option<u32>,
    #[serde(default)]
    pub windows_version: Option<String>,
    #[serde(default)]
    pub registry: Vec<RegistryEntry>,
    pub launch: LaunchSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub key: String,
    pub name: String,
    pub kind: RegistryKind,
    pub value: toml::Value,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegistryKind {
    Dword,
    String,
    Qword,
    Binary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchSpec {
    pub executable: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

#[derive(Debug, Default)]
pub struct RecipeRunner;

impl RecipeRunner {
    pub fn parse(input: &str) -> Result<Recipe, RecipeError> {
        Ok(toml::from_str(input)?)
    }

    pub fn load_from_file(path: &Path) -> Result<Recipe, RecipeError> {
        let text = std::fs::read_to_string(path)?;
        Self::parse(&text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NORLAND_RECIPE: &str = r#"
id              = "norland"
name            = "Norland"
steam_app_id    = 1857090
windows_version = "win10"

[[registry]]
key   = 'HKCU\Software\Wine\Direct3D'
name  = 'cb_access_map_w'
kind  = 'dword'
value = 1

[launch]
executable = 'C:\Program Files (x86)\Steam\steamapps\common\Norland Story Generating Strategy\Norland.exe'

[launch.env]
WINEMSYNC = "1"
"#;

    #[test]
    fn parses_norland_starter_recipe() {
        let r = RecipeRunner::parse(NORLAND_RECIPE).expect("parse");
        assert_eq!(r.id, "norland");
        assert_eq!(r.name, "Norland");
        assert_eq!(r.steam_app_id, Some(1857090));
        assert_eq!(r.windows_version.as_deref(), Some("win10"));
        assert_eq!(r.registry.len(), 1);
        assert!(matches!(r.registry[0].kind, RegistryKind::Dword));
        assert_eq!(r.launch.env.get("WINEMSYNC"), Some(&"1".to_string()));
    }

    #[test]
    fn rejects_missing_required_fields() {
        let bad = r#"
id = "broken"
name = "Broken"
[launch]
# missing `executable`
"#;
        let err = RecipeRunner::parse(bad).unwrap_err();
        assert!(matches!(err, RecipeError::Parse(_)));
    }
}
