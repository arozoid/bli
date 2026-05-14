mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Isolate(args) => commands::isolate::run(args)?,
        Commands::Merge => commands::merge::run()?,

        Commands::Load(args) => commands::load::run(args)?,
        Commands::Unload(args) => commands::unload::run(args)?,

        Commands::Stash(args) => commands::stash::run(args)?,
        Commands::Checkout(args) => commands::checkout::run(args)?,

        Commands::Restock => commands::restock::run()?,
        Commands::Query(args) => commands::query::run(args)?,
        Commands::Survey => commands::survey::run()?,
        Commands::Clean => commands::clean::run()?,

        Commands::Status => commands::status::run()?,

        Commands::Source(args) => commands::source::run(args)?,
    }

    Ok(())
}
