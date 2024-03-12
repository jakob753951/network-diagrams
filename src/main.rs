use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

use clap::{Parser, ValueEnum};
use graphviz_rust::cmd::Format;
use graphviz_rust::exec_dot;
use serde::{Deserialize, Serialize};
use color_eyre::Result;

use graph::Graph;
use task::Task;
use cli::Cli;

mod graph;
mod task;
mod cli;


#[derive(Serialize, Deserialize)]
struct Config {
    tasks: Vec<Task>,
    connections: Vec<(String, String)>,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let config = read_config(cli.config_file_path)?;

    let mut graph: Graph<String, Task> = Graph::new();

    config.tasks.iter().for_each(|task| graph.add_vertex(task.id.clone(), task.clone()));

    config.connections.iter().for_each(|(from, to)| graph.connect_vertices(from.clone(), to.clone()));

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

    Ok(())
}

fn read_config(path: String) -> Result<Config> {
    let mut file = File::open(path).expect("couldn't open the input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("couldn't read the input file");
    let config: Config = serde_json::from_str(contents.as_str()).expect("couldn't parse the contents of the input file");
    Ok(config)
}
