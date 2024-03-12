use std::fmt::{Display, Formatter};
use clap::{Parser, ValueEnum};
use graphviz_rust::cmd::Format;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(value_name = "FILE")]
    pub(crate) config_file_path: String,
    #[arg(short, long, default_value = "graph")]
    pub(crate) output_path: String,
    #[arg(short = 'f', long = "format", value_enum, value_name = "FORMAT", default_value_t = OutputFormat::Png)]
    pub(crate) output_format: OutputFormat,
}

#[derive(Copy, Clone, ValueEnum)]
pub(crate) enum OutputFormat {
    Png,
    Svg,
    Dot,
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Png => write!(f, "png"),
            OutputFormat::Svg => write!(f, "svg"),
            OutputFormat::Dot => write!(f, "dot"),
        }
    }
}

impl Into<Format> for OutputFormat {
    fn into(self) -> Format {
        match self {
            OutputFormat::Png => Format::Png,
            OutputFormat::Svg => Format::Svg,
            OutputFormat::Dot => Format::Dot,
        }
    }
}

impl Into<String> for OutputFormat {
    fn into(self) -> String {
        match self {
            OutputFormat::Png => "png".to_string(),
            OutputFormat::Svg => "svg".to_string(),
            OutputFormat::Dot => "dot".to_string(),
        }
    }
}