//! Bottle manager: CRUD over Wine prefixes.
//!
//! A bottle is a Wine prefix with metadata: a unique ID, a friendly name, the
//! Wine install it uses, the recipe applied (if any), and a launch target.
//! Bottles persist under `~/.vinaria/bottles/<uuid>/`:
//!   meta.toml — bottle metadata
//!   prefix/   — the WINEPREFIX

use crate::paths;
use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BottleId(pub Uuid);

impl BottleId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::fmt::Display for BottleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// What ends up in meta.toml. Kept small on purpose — the prefix itself is
/// the source of truth for Windows-side state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottle {
    pub id: BottleId,
    pub name: String,
    /// Recipe ID (slug) applied to this bottle, if any.
    pub recipe: Option<String>,
    /// Subpath within the bottle dir holding the wine prefix. Always "prefix"
    /// today, kept as a field for future flexibility (e.g. shared prefixes).
    #[serde(default = "default_prefix_subdir")]
    pub prefix_subdir: String,
}

fn default_prefix_subdir() -> String {
    "prefix".to_string()
}

impl Bottle {
    /// Path on disk to this bottle's directory.
    pub fn dir(&self) -> Result<PathBuf> {
        Ok(paths::bottles_dir()?.join(self.id.to_string()))
    }

    /// Path to the wine prefix inside this bottle.
    pub fn prefix(&self) -> Result<PathBuf> {
        Ok(self.dir()?.join(&self.prefix_subdir))
    }

    /// Path to the meta.toml file.
    pub fn meta_path(&self) -> Result<PathBuf> {
        Ok(self.dir()?.join("meta.toml"))
    }
}

#[derive(Debug, Default)]
pub struct BottleManager;

impl BottleManager {
    pub fn new() -> Self {
        Self
    }

    /// Create a new bottle on disk with an empty prefix subdir. The caller is
    /// expected to follow up with wineboot to initialize the prefix.
    pub fn create(&self, name: impl Into<String>, recipe: Option<String>) -> Result<Bottle> {
        paths::ensure_layout()?;
        let bottle = Bottle {
            id: BottleId::new(),
            name: name.into(),
            recipe,
            prefix_subdir: default_prefix_subdir(),
        };
        let dir = bottle.dir()?;
        std::fs::create_dir_all(dir.join(&bottle.prefix_subdir))?;
        self.write_meta(&bottle)?;
        Ok(bottle)
    }

    pub fn list(&self) -> Result<Vec<Bottle>> {
        let dir = paths::bottles_dir()?;
        if !dir.exists() {
            return Ok(Vec::new());
        }
        let mut out = Vec::new();
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let meta = entry.path().join("meta.toml");
            if let Ok(text) = std::fs::read_to_string(&meta) {
                if let Ok(bottle) = toml::from_str::<Bottle>(&text) {
                    out.push(bottle);
                }
            }
        }
        out.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(out)
    }

    pub fn get(&self, id: BottleId) -> Result<Bottle> {
        let path = paths::bottles_dir()?.join(id.to_string()).join("meta.toml");
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("reading bottle meta at {}", path.display()))?;
        Ok(toml::from_str(&text)?)
    }

    pub fn delete(&self, id: BottleId) -> Result<()> {
        let dir = paths::bottles_dir()?.join(id.to_string());
        if !dir.exists() {
            return Err(anyhow!("bottle {} does not exist", id));
        }
        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }

    fn write_meta(&self, bottle: &Bottle) -> Result<()> {
        let text = toml::to_string_pretty(bottle)?;
        std::fs::write(bottle.meta_path()?, text)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Tests touch ~/.vinaria/ so serialize them. Each test cleans up its own
    /// bottle and asserts it doesn't leak.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn create_then_list_then_delete() {
        let _g = TEST_LOCK.lock().unwrap();
        let mgr = BottleManager::new();
        let b = mgr.create("test-bottle-xyz", None).unwrap();
        let list = mgr.list().unwrap();
        assert!(list.iter().any(|x| x.id == b.id));
        assert!(b.prefix().unwrap().exists());
        mgr.delete(b.id).unwrap();
        let after = mgr.list().unwrap();
        assert!(!after.iter().any(|x| x.id == b.id));
    }

    #[test]
    fn meta_roundtrip_through_toml() {
        let _g = TEST_LOCK.lock().unwrap();
        let mgr = BottleManager::new();
        let b = mgr
            .create("roundtrip-bottle", Some("norland".into()))
            .unwrap();
        let loaded = mgr.get(b.id).unwrap();
        assert_eq!(loaded.id, b.id);
        assert_eq!(loaded.name, "roundtrip-bottle");
        assert_eq!(loaded.recipe.as_deref(), Some("norland"));
        mgr.delete(b.id).unwrap();
    }
}
