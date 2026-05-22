//! Validate a recipe TOML file. Used by scripts/validate-recipes.sh.
//!
//! Usage: cargo run -p vinaria-core --example validate -- path/to/recipe.toml

use std::path::PathBuf;
use std::process::ExitCode;
use vinaria_core::RecipeRunner;

fn main() -> ExitCode {
    let path: PathBuf = match std::env::args().nth(1) {
        Some(p) => p.into(),
        None => {
            eprintln!("usage: validate <path/to/recipe.toml>");
            return ExitCode::from(2);
        }
    };
    match RecipeRunner::load_from_file(&path) {
        Ok(recipe) => {
            eprintln!("  id={} name={:?}", recipe.id, recipe.name);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("  parse failed: {e}");
            ExitCode::FAILURE
        }
    }
}
