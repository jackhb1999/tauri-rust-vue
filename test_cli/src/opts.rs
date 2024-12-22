use std::path::Path;
use std::str::FromStr;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rtcli",version,author,about,long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV,or converts CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long,value_parser=verify_input_file)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg( long,value_parser=parse_format, default_value = "json")]
    pub format: OutputFormat,
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

impl From<OutputFormat> for &'static str {
   fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => { "json" }
            OutputFormat::Yaml => { "yaml" }
            OutputFormat::Toml => { "toml" }
        }
    }
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;
    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            v => anyhow::bail!("Unsupported format:{}",v),
        }
    }
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file not found.")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    format.parse::<OutputFormat>()
}