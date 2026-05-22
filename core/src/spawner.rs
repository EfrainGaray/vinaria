//! Process spawner: launch Wine processes with the right environment.
//!
//! This replaces CrossOver's Perl launcher. Built from scratch using Wine's
//! documented env contract — no code copied from CrossOver.

use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpawnError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct ProcessSpawner {
    pub wine_dir: PathBuf,
    pub prefix: PathBuf,
}

impl ProcessSpawner {
    pub fn new(wine_dir: PathBuf, prefix: PathBuf) -> Self {
        Self { wine_dir, prefix }
    }

    /// Build the Command with WINE env contract: WINEPREFIX, WINESERVER,
    /// WINELOADER. Caller adds further env (WINEMSYNC, WINEDEBUG, etc.).
    pub fn command(&self, exe: &str) -> Command {
        let mut cmd = Command::new(self.wine_dir.join("bin/wine"));
        cmd.env("WINEPREFIX", &self.prefix)
            .env("WINESERVER", self.wine_dir.join("bin/wineserver"))
            .env("WINELOADER", self.wine_dir.join("bin/wine"))
            .arg(exe);
        cmd
    }
}
