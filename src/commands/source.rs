use anyhow::Result;

use crate::cli::SourceArgs;

pub fn run(args: SourceArgs) -> Result<()> {
    println!("{args}");

    Ok(())
}
