use anyhow::Result;

use crate::cli::QueryArgs;

pub fn run(args: QueryArgs) -> Result<()> {
    println!("{args}");

    Ok(())
}
