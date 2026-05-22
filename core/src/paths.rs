//! Filesystem layout helpers.
//!
//! All Vinaria state lives under `~/.vinaria/`:
//!   wine/          — compiled Wine install (single shared install)
//!   bottles/<id>/  — one directory per bottle (meta.toml + prefix/)
//!   recipes/       — user-local recipe overrides
//!   logs/          — per-bottle launch logs
//!
//! This module centralizes path computation so the rest of the code doesn't
//! sprinkle `home_dir()` calls.

use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn home() -> Result<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("HOME environment variable not set"))
}

pub fn vinaria_root() -> Result<PathBuf> {
    Ok(home()?.join(".vinaria"))
}

pub fn wine_dir() -> Result<PathBuf> {
    Ok(vinaria_root()?.join("wine"))
}

pub fn bottles_dir() -> Result<PathBuf> {
    Ok(vinaria_root()?.join("bottles"))
}

pub fn recipes_dir() -> Result<PathBuf> {
    Ok(vinaria_root()?.join("recipes"))
}

pub fn logs_dir() -> Result<PathBuf> {
    Ok(vinaria_root()?.join("logs"))
}

pub fn ensure_layout() -> Result<()> {
    for d in [bottles_dir()?, recipes_dir()?, logs_dir()?] {
        std::fs::create_dir_all(&d)?;
    }
    Ok(())
}
