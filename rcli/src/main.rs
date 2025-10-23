use clap::Parser;
use rcli::opts::{Opt, OutputFormat, SubCommand};
use rcli::process::process_csv;

fn main() -> anyhow::Result<()> {
    println!("{}", OutputFormat::Json);
    let opts = Opt::parse();
    println!("command is :{:?}", opts);

    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let output_file = csv_opts
                .output
                .unwrap_or_else(|| format!("output.{}", csv_opts.format));
            process_csv(&csv_opts.input, &output_file, csv_opts.format)?;
        }
    }

    Ok(())
}
