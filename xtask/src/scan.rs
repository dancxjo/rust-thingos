use clap::Args;

use crate::common::Result;

#[derive(Args, Clone, Debug)]
pub struct ScanArgs {
    #[arg(long)]
    pub path: Option<String>,
}

pub fn run(args: ScanArgs) -> Result<()> {
    println!("xtask: scan requested: {args:?}");
    Ok(())
}
