use anyhow::Result;

use crate::cli::UnloadArgs;

pub fn run(args: UnloadArgs) -> Result<()> {
    println!("{args:#?}");

    Ok(())
}
