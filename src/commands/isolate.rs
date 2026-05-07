use anyhow::Result;

use crate::cli::IsolateArgs;

pub fn run(args: IsolateArgs) -> Result<()> {
    println!("{args:#?}");

    Ok(())
}
