use clap::Parser;
use todo_cli::{args::Args, Result};

fn main() -> Result<()> {
    let args = Args::parse();
    args.run()?;
    Ok(())
}
