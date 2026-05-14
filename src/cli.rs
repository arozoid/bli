use clap::{Args, Parser, Subcommand};
use std::fmt;

#[derive(Parser, Debug)]
#[command(name = "bli")]
#[command(version)]
#[command(about = "portable isolated package environments")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
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

    Source(SourceArgs),
}

#[derive(Args, Debug)]
pub struct IsolateArgs {
    pub folder: String,
}

#[derive(Args, Debug)]
pub struct LoadArgs {
    pub package: String,
}

#[derive(Args, Debug)]
pub struct UnloadArgs {
    pub package: String,
}

#[derive(Args, Debug)]
pub struct StashArgs {
    pub name: Option<String>,
}

#[derive(Args, Debug)]
pub struct CheckoutArgs {
    pub stash: String,
}

#[derive(Args, Debug)]
pub struct QueryArgs {
    pub package: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum SourceSubcommand {
    Add { repo: String },
    Remove { repo: String },
    List,
}

#[derive(Args, Debug)]
pub struct SourceArgs {
    #[command(subcommand)]
    pub command: SourceSubcommand,
}

macro_rules! display_args {
    ($name:ident) => {
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{self:?}")
            }
        }
    };
}

display_args!(IsolateArgs);
display_args!(LoadArgs);
display_args!(UnloadArgs);
display_args!(StashArgs);
display_args!(CheckoutArgs);
display_args!(QueryArgs);
display_args!(SourceSubcommand);
display_args!(SourceArgs);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_args_support_debug_and_display() {
        let isolate = IsolateArgs { folder: "pkg".into() };
        let load = LoadArgs { package: "pkg".into() };
        let unload = UnloadArgs { package: "pkg".into() };
        let stash = StashArgs { name: Some("stash-name".into()) };
        let checkout = CheckoutArgs { stash: "stash-name".into() };
        let query = QueryArgs { package: Some("pkg".into()) };
        let source = SourceArgs { command: SourceSubcommand::List };

        assert_eq!(format!("{isolate}"), format!("{isolate:?}"));
        assert_eq!(format!("{load}"), format!("{load:?}"));
        assert_eq!(format!("{unload}"), format!("{unload:?}"));
        assert_eq!(format!("{stash}"), format!("{stash:?}"));
        assert_eq!(format!("{checkout}"), format!("{checkout:?}"));
        assert_eq!(format!("{query}"), format!("{query:?}"));
        assert_eq!(format!("{source}"), format!("{source:?}"));
    }
}
