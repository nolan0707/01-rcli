use clap::Parser;
use rcli::process_csv;
use rcli::{Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(ops) => process_csv(&ops.input, &ops.output)?,
    }
    Ok(())
}
