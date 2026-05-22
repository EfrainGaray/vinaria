//! Process spawner: launch Wine processes with the right environment.
//!
//! This replaces CrossOver's Perl launcher. Built from scratch using Wine's
//! documented env contract — no code copied from CrossOver.

use crate::bottle::Bottle;
use crate::paths;
use crate::recipe::Recipe;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpawnError {
    #[error("Wine not installed at {0}. Build it with scripts/build-wine.sh.")]
    WineNotInstalled(PathBuf),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("anyhow: {0}")]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct ProcessSpawner {
    wine_dir: PathBuf,
}

impl ProcessSpawner {
    /// Use the standard ~/.vinaria/wine/ install.
    pub fn standard() -> Result<Self, SpawnError> {
        let dir = paths::wine_dir()?;
        if !dir.join("bin/wine").exists() {
            return Err(SpawnError::WineNotInstalled(dir));
        }
        Ok(Self { wine_dir: dir })
    }

    /// Override the Wine install path (for tests or custom builds).
    pub fn with_wine_dir(dir: impl Into<PathBuf>) -> Self {
        Self {
            wine_dir: dir.into(),
        }
    }

    /// Build a Command running an arbitrary executable inside the bottle.
    /// Caller adds args before spawning.
    pub fn command_in_bottle(
        &self,
        bottle: &Bottle,
        executable: &Path,
    ) -> Result<Command, SpawnError> {
        let prefix = bottle.prefix()?;
        let mut cmd = Command::new(self.wine_dir.join("bin/wine"));
        cmd.env_clear()
            .env("HOME", paths::home()?)
            .env("PATH", "/usr/local/bin:/usr/bin:/bin")
            .env("WINEPREFIX", &prefix)
            .env("WINESERVER", self.wine_dir.join("bin/wineserver"))
            .env("WINELOADER", self.wine_dir.join("bin/wine"))
            .env("WINEDEBUG", "-all")
            // macOS dyld doesn't search Homebrew's /usr/local/lib by default
            // for dlopen calls. Wine dlopen's libfreetype, libgnutls etc. at
            // runtime — this fallback gets them found without needing the
            // user to set the variable themselves.
            .env(
                "DYLD_FALLBACK_LIBRARY_PATH",
                "/usr/local/lib:/usr/lib",
            )
            .arg(executable);
        Ok(cmd)
    }

    /// Build a Command using a recipe's launch spec against a bottle.
    /// Applies the recipe's env vars on top of the base Wine env.
    pub fn command_for_recipe(
        &self,
        bottle: &Bottle,
        recipe: &Recipe,
    ) -> Result<Command, SpawnError> {
        let exe = PathBuf::from(&recipe.launch.executable);
        let mut cmd = self.command_in_bottle(bottle, &exe)?;
        for arg in &recipe.launch.args {
            cmd.arg(arg);
        }
        for (k, v) in &recipe.launch.env {
            cmd.env(k, v);
        }
        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bottle::{BottleId, Bottle};

    fn fake_bottle() -> Bottle {
        Bottle {
            id: BottleId::new(),
            name: "fake".into(),
            recipe: None,
            prefix_subdir: "prefix".into(),
        }
    }

    #[test]
    fn command_in_bottle_sets_wine_env() {
        let spawner = ProcessSpawner::with_wine_dir("/tmp/fake-wine");
        let b = fake_bottle();
        let cmd = spawner
            .command_in_bottle(&b, Path::new("C:/notepad.exe"))
            .unwrap();
        let env: std::collections::HashMap<String, String> = cmd
            .get_envs()
            .filter_map(|(k, v)| {
                Some((k.to_string_lossy().into_owned(), v?.to_string_lossy().into_owned()))
            })
            .collect();
        assert_eq!(env.get("WINEDEBUG").map(|s| s.as_str()), Some("-all"));
        assert!(env.get("WINEPREFIX").unwrap().contains(&b.id.to_string()));
        assert_eq!(
            env.get("DYLD_FALLBACK_LIBRARY_PATH").map(|s| s.as_str()),
            Some("/usr/local/lib:/usr/lib")
        );
    }
}
