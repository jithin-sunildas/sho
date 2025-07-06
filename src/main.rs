use std::fmt::format;
use anyhow::Context;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Error while reading file '{}'", args.path.display()))?;

    for line in content.lines(){
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
