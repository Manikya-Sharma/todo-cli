use clap::Parser;
use todo_cli_manikya::{args::Args, Result};

fn main() -> Result<()> {
    let args = Args::parse();
    args.run()?;
    Ok(())
}
