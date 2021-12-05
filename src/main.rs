#![warn(clippy::pedantic)]

use anyhow::{anyhow, Context, Result};
use cargo_metadata::MetadataCommand;
use rand::{thread_rng, Rng};
use std::fs;
use structopt::StructOpt;
use syn::spanned::Spanned;
use walkdir::WalkDir;

#[derive(StructOpt)]
enum Args {
    Fox,
}

fn foxify(file: &syn::File, lines: &mut Vec<String>) {
    for item in &file.items {
        if thread_rng().gen_ratio(1, 3) {
            let line = item.span().start().line - 1;

            let ws = if let Some(whitespace) =
                lines[line].chars().position(|c| !c.is_ascii_whitespace())
            {
                whitespace
            } else {
                continue;
            };

            let mut fox = " ".repeat(ws);
            fox.push_str("// a fox.");

            lines.insert(line, fox);
        }
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();

    if matches!(args, Args::Fox) {
        let manifest = MetadataCommand::new()
            .no_deps()
            .exec()
            .context("Failed to retrieve Cargo metadata")?;

        // we currently don't support workspaces
        assert_eq!(manifest.packages.len(), 1);
        assert_eq!(manifest.packages[0].targets.len(), 1);

        let src_path = manifest.packages[0].targets[0]
            .src_path
            .parent()
            .ok_or(anyhow!("Failed to find source path"))?;

        for entry in WalkDir::new(src_path.as_std_path()) {
            let entry = entry.context("Failed to retrieve dir entry")?;

            if let Some(extension) = entry.path().extension() {
                if extension != "rs" {
                    continue;
                }

                let content = fs::read_to_string(entry.path())
                    .context(anyhow!("Failed to read {:?}", entry.path().as_os_str()))?;

                let file = syn::parse_file(&content).context("Failed to parse file")?;
                let mut lines: Vec<String> = content.lines().map(String::from).collect();

                foxify(&file, &mut lines);

                fs::write(entry.path(), lines.join("\n"))
                    .context(format!("Failed to write to {:?}", entry.path().to_str()))?;
            }
        }
    }

    Ok(())
}
