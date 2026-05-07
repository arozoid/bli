use anyhow::Result;

use crate::cli::StashArgs;

pub fn run(args: StashArgs) -> Result<()> {
    println!("{args:#?}");

    Ok(())
}
