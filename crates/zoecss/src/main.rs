//! ZoeCSS CLI — scan sources, extract tokens, cache results, output CSS.

use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::Parser;

use zoecss_config::{CompiledConfig, Config};
use zoecss_core::{CssEngine, extract_tokens, generate};
use zoecss_presets::base;

#[derive(Parser)]
#[command(name = "zoecss", about = "Scan source files and generate CSS")]
struct Cli {
    /// Source files to scan for utility classes
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(&cli.files) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:#}");
            ExitCode::from(1)
        }
    }
}

fn run(files: &[PathBuf]) -> Result<()> {
    let mut seen = HashSet::new();
    let mut tokens: Vec<String> = Vec::new();

    for path in files {
        let content = fs::read_to_string(path)
            .with_context(|| format!("failed to read '{}'", path.display()))?;

        for token in extract_tokens(&content) {
            if seen.insert(token.to_owned()) {
                tokens.push(token.to_owned());
            }
        }
    }

    let mut config = Config::new();
    config.presets.push(base());
    let compiled =
        CompiledConfig::compile(config.merge()).context("failed to compile configuration")?;

    let css: Vec<String> = tokens
        .iter()
        .filter_map(|token| generate(&compiled, token))
        .collect();

    print!("@layer base, utilities;\n\n");

    let base = compiled.base_css();
    if base.is_empty() {
        print!("@layer base {{}}\n\n");
    } else {
        let indented: String = base.lines().map(|line| format!("  {line}\n")).collect();
        print!("@layer base {{\n{indented}}}\n\n");
    }

    if !css.is_empty() {
        let indented: String = css.iter().map(|rule| format!("  {rule}\n")).collect();
        print!("@layer utilities {{\n{indented}}}");
    } else {
        print!("@layer utilities {{}}");
    }

    Ok(())
}
