use anyhow::Result;

use crate::cli::CheckoutArgs;

pub fn run(args: CheckoutArgs) -> Result<()> {
    println!("{args}");

    Ok(())
}
