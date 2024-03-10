use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

use clap::{Parser, ValueEnum};
use graphviz_rust::cmd::Format;
use graphviz_rust::exec_dot;
use serde::{Deserialize, Serialize};

use graph::Graph;
use task::Task;

mod graph;
mod task;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "FILE")]
    config_file_path: String,
    #[arg(short, long, default_value = "graph")]
    output_path: String,
    #[arg(short = 'f', long = "format", value_enum, value_name = "FORMAT", default_value_t = OutputFormat::Png)]
    output_format: OutputFormat,
}

#[derive(Copy, Clone, ValueEnum)]
enum OutputFormat {
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

#[derive(Serialize, Deserialize)]
struct Config {
    tasks: Vec<Task>,
    connections: Vec<(String, String)>,
}

fn main() {
    let cli = Cli::parse();
    let mut file = File::open(cli.config_file_path).expect("couldn't open the input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("couldn't read the input file");
    let val: Config = serde_json::from_str(contents.as_str()).expect("couldn't parse the contents of the input file");

    let mut graph: Graph<String, Task> = Graph::new();

    val.tasks.iter().for_each(|task| graph.add_vertex(task.id.clone(), task.clone()));

    val.connections.iter().for_each(|(from, to)| graph.connect_vertices(from.clone(), to.clone()));

    let format: Format = cli.output_format.clone().into();
    let graph_data = exec_dot(
        graph.to_string(),
        // I'm sorry. We have to go from OutputFormat to Format to CommandArg. We're basically just doing output_format.into().into()
        vec![format.into()],
    ).expect("Couldn't generate graph");

    let output_file_name = format!("{}.{}", cli.output_path, cli.output_format);

    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        .open(output_file_name.clone()).expect("Couldn't create/open output file");

    file.write_all(&graph_data).expect("Writing to file failed");

    println!("{} has been created.", output_file_name);
}
