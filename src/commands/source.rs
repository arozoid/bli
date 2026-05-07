use anyhow::Result;

use crate::cli::COMMANDArgs;

pub fn run(args: COMMANDArgs) -> Result<()> {
    println!("{args:#?}");

    Ok(())
}
