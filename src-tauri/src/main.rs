#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use vinaria_core::{paths, Bottle, BottleId, BottleManager, ProcessSpawner, Recipe, RecipeRunner};

#[derive(Debug, Serialize)]
struct VinariaError {
    message: String,
}

impl<E: std::fmt::Display> From<E> for VinariaError {
    fn from(e: E) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

type CmdResult<T> = Result<T, VinariaError>;

#[tauri::command]
fn list_bottles() -> CmdResult<Vec<Bottle>> {
    Ok(BottleManager::new().list()?)
}

#[tauri::command]
fn create_bottle(name: String, recipe: Option<String>) -> CmdResult<Bottle> {
    Ok(BottleManager::new().create(name, recipe)?)
}

#[tauri::command]
fn delete_bottle(id: String) -> CmdResult<()> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    BottleManager::new().delete(BottleId(uuid))?;
    Ok(())
}

#[tauri::command]
fn list_recipes() -> CmdResult<Vec<Recipe>> {
    // Phase 4 scaffold: read TOML files from <repo>/recipes/ next to the binary
    // in dev. Phase 6 expands to merge with ~/.vinaria/recipes/ user overrides.
    let mut out = Vec::new();
    let dir = paths::recipes_dir()?;
    if dir.exists() {
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) != Some("toml") {
                continue;
            }
            if let Ok(r) = RecipeRunner::load_from_file(&entry.path()) {
                out.push(r);
            }
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

#[derive(Debug, Serialize)]
struct WineState {
    installed: bool,
    version: Option<String>,
    dll_count: Option<usize>,
    install_dir: String,
}

#[tauri::command]
fn wine_state() -> CmdResult<WineState> {
    let dir = paths::wine_dir()?;
    let bin = dir.join("bin/wine");
    if !bin.exists() {
        return Ok(WineState {
            installed: false,
            version: None,
            dll_count: None,
            install_dir: dir.display().to_string(),
        });
    }
    // Read version from a `wine --version` invocation. Cheap enough to do
    // synchronously on a settings page open.
    let version = std::process::Command::new(&bin)
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string());
    let dll_count = std::fs::read_dir(dir.join("lib/wine/x86_64-windows"))
        .ok()
        .map(|it| it.filter_map(|e| e.ok()).count());
    Ok(WineState {
        installed: true,
        version,
        dll_count,
        install_dir: dir.display().to_string(),
    })
}

#[tauri::command]
fn launch_bottle(id: String, recipe_id: Option<String>) -> CmdResult<()> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let mgr = BottleManager::new();
    let bottle = mgr.get(BottleId(uuid))?;
    let spawner = ProcessSpawner::standard()?;

    // If a recipe is given, use its launch spec. Otherwise the caller has to
    // pass an executable separately (not exposed here yet).
    if let Some(rid) = recipe_id {
        let recipes = list_recipes()?;
        let recipe = recipes
            .into_iter()
            .find(|r| r.id == rid)
            .ok_or_else(|| format!("recipe {rid} not found"))?;
        let mut cmd = spawner.command_for_recipe(&bottle, &recipe)?;
        cmd.spawn().map_err(|e| e.to_string())?;
    } else {
        return Err("launching without a recipe is not implemented yet".into());
    }
    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vinaria=debug,info".into()),
        )
        .init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_bottles,
            create_bottle,
            delete_bottle,
            list_recipes,
            wine_state,
            launch_bottle,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
