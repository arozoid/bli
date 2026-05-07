use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bli")]
#[command(version)]
#[command(about = "portable isolated package environments")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Isolate(IsolateArgs),
    Merge,

    Load(LoadArgs),
    Unload(UnloadArgs),

    Stash(StashArgs),
    Checkout(CheckoutArgs),

    Restock,
    Query(QueryArgs),
    Survey,
    Clean,

    Status,
    List,

    // Source(SourceArgs),
}

#[derive(Args)]
pub struct IsolateArgs {
    pub folder: String,
}

#[derive(Args)]
pub struct LoadArgs {
    pub package: String,
}

#[derive(Args)]
pub struct UnloadArgs {
    pub package: String,
}

#[derive(Args)]
pub struct StashArgs {
    pub name: Option<String>,
}

#[derive(Args)]
pub struct CheckoutArgs {
    pub stash: String,
}

#[derive(Args)]
pub struct QueryArgs {
    pub package: Option<String>,
}

// #[derive(Subcommand)]
// pub enum SourceSubcommand {
//     Add { repo: String },
//     Remove { repo: String },
//     List,
// }

// #[derive(Args)]
// pub struct SourceArgs {
//     #[command(subcommand)]
//     pub command: SourceSubcommand,
// }
