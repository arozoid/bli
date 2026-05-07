use anyhow::Result;

use crate::cli::LoadArgs;

pub fn run(args: LoadArgs) -> Result<()> {
    println!("{args:#?}");

    Ok(())
}
