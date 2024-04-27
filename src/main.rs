use clap::Parser;
use rcli::process_csv;
use rcli::{Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(ops) => {
            let output = if let Some(output) = ops.output {
                output.clone()
            } else {
                format!("output.{}", ops.format)
            };
            process_csv(&ops.input, &output, ops.format)?
        }
    }
    Ok(())
}
