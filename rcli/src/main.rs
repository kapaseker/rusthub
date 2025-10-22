use clap::Parser;
use rcli::opts::{Opt, SubCommand};
use rcli::process::process_csv;

fn main() -> anyhow::Result<()> {
    let opts = Opt::parse();
    println!("command is :{:?}", opts);

    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            process_csv(&csv_opts.input, &csv_opts.output)?;
        }
    }

    Ok(())
}
