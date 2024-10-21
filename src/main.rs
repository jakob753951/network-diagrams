
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

use clap::Parser;
use graphviz_rust::cmd::Format;
use graphviz_rust::exec_dot;
use color_eyre::Result;

use graph::Graph;
use task::Task;
use cli::Cli;

mod graph;
mod task;
mod cli;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    let tasks = read_config(cli.config_file_path.clone())?;

    let mut graph: Graph<String, Task> = Graph::new();

    tasks.iter().for_each(|task| graph.add_vertex(task.id.clone(), task.clone()));

    tasks.iter()
        .for_each(|task| 
              task.predecessors.iter()
                  .for_each(|predecessor|
                      graph.connect_vertices(predecessor.clone(), task.clone().id)
                  )
        );

    let format: Format = cli.output_format.into();
    let graph_data = exec_dot(
        graph.to_string(),
        vec![format.into()],
    ).expect("Couldn't generate graph.\nPlease be mindful of special characters in IDs");

    let output_file_name = cli.output_path;
    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        .open(output_file_name.clone()).expect("Couldn't create/open output file");

    file.write_all(&graph_data).expect("Couldn't write to file");

    println!("{} has been created.", output_file_name);

    Ok(())
}

fn read_config(path: String) -> Result<Vec<Task>> {
    let mut file = File::open(path).expect("Couldn't open the input file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Couldn't read the input file");
    let tasks: Vec<Task> = serde_json::from_str(contents.as_str()).expect("Couldn't parse the contents of the input file");
    Ok(tasks)
}