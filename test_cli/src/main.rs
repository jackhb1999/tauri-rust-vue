use clap::builder::TypedValueParser;
use clap::Parser;
use test_cli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // format!("{}{}","output.",String::from(opts.format))
                format!("{}{}","output.",Into::<&str>::into(opts.format))
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}
