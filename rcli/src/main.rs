use clap::Parser;
use rcli::opts::{Opt, SubCommand};
use rcli::process::csv_process::process_csv;
use rcli::process::gen_pass_process::process_gen_pass;

fn main() -> anyhow::Result<()> {

    let opts = Opt::parse();

    println!("command is :{:?}", opts);

    match opts.cmd {
        SubCommand::Csv(csv_opts) => {
            let output_file = csv_opts
                .output
                .unwrap_or_else(|| format!("output.{}", csv_opts.format));
            process_csv(&csv_opts.input, &output_file, csv_opts.format)?;
        }

        SubCommand::GenPass(gen_pass_opt) => {
            println!("Generating passwords with option {:?}", gen_pass_opt);
            process_gen_pass(gen_pass_opt.length, gen_pass_opt.lowercase, gen_pass_opt.uppercase, gen_pass_opt.number, gen_pass_opt.symbol)?;
        }
    }

    Ok(())
}
