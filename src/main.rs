use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::fs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::hash::Hash;
use std::io::{Read, Write};
use graphviz_rust::cmd::Format;

use graphviz_rust::{exec, parse};
use graphviz_rust::printer::PrinterContext;

mod graph;

use graph::Graph;

#[derive(Serialize, Deserialize)]
struct Config {
    tasks: Vec<Task>,
    connections: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize)]
#[derive(Hash)]
#[derive(Clone)]
struct Task {
    id: String,
    description: String,
    duration: i32,
}

impl<VId> Graph<VId, Task>
    where
        VId: Eq + Hash + Clone
{
    fn early_start(&self, node: &VId) -> i32 {
        self.predecessor_edges.get(&node)
            .unwrap_or(&HashSet::new())
            .iter()
            .map(|predecessor| self.early_finish(&predecessor))
            .max()
            .unwrap_or(0)
    }

    fn early_finish(&self, node: &VId) -> i32 {
        self.early_start(&node) + self.vertices[&node].duration
    }

    fn late_start(&self, node: &VId) -> i32 {
        self.late_finish(&node) - self.vertices[&node].duration
    }

    fn late_finish(&self, node: &VId) -> i32 {
        self.successor_edges.get(&node)
            .unwrap_or(&HashSet::new())
            .iter()
            .map(|successor| self.late_start(&successor))
            .min()
            .unwrap_or(self.early_finish(node))
    }

    fn slack(&self, node: &VId) -> i32 {
        self.late_start(&node) - self.early_start(node)
    }
}

impl Display for Graph<String, Task>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "strict digraph network_diagram {{")?;
        writeln!(f, "   node [shape=plaintext]")?;

        self.vertices.iter().try_for_each(|(id, node)| {
            writeln!(f, r#"    node_{replaced_id} [label=<
                    <table border="0" cellborder="1" cellspacing="0">
                        <tr height="30px"><td width="40px">{early_start}</td><td>{id}</td><td>{early_finish}</td></tr>
                        <tr height="30px"><td width="40px">{slack}</td><td colspan="2">{description}</td></tr>
                        <tr height="30px"><td width="40px">{late_start}</td><td>{duration}</td><td>{late_finish}</td></tr>
                    </table>
                >];"#,
                replaced_id = id.replace(".", "_"),
                early_start = self.early_start(&id),
                id = id,
                early_finish = self.early_finish(&id),
                slack = self.slack(&id),
                description = node.description,
                late_start = self.late_start(&id),
                duration = node.duration,
                late_finish = self.late_finish(&id),
            )
        })?;

        writeln!(f)?;

        self.successor_edges.iter().try_for_each(|(from, successors)| {
            successors.iter().try_for_each(|to| {
                if self.slack(from) == 0 && self.slack(to) == 0 {

                    writeln!(f, r#"   node_{from} -> node_{to} [color="red" penwidth="2"]"#, from = from.replace(".", "_"), to = to.replace(".", "_"))
                } else {
                    writeln!(f, "   node_{from} -> node_{to}", from = from.replace(".", "_"), to = to.replace(".", "_"))
                }
            })
        })?;

        writeln!(f, "}}")?;
        Ok(())
    }
}

fn main() {
    let mut file = File::open("config.json").expect("couldn't open config.json");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("couldn't read config.json");
    let val: Config = serde_json::from_str(contents.as_str()).expect("couldn't parse the contents of config.json");

    let mut graph: Graph<String, Task> = Graph::new();

    val.tasks.iter().for_each(|task| graph.add_vertex(task.id.clone(), task.clone()));

    val.connections.iter().for_each(|(from, to)| graph.connect_vertices(from.clone(), to.clone()));

    let graph_string = format!("{graph}");
    let Ok(dot_graph) = parse(graph_string.as_str()) else {
        println!("Something went wrong when parsing your graph. Exiting program...");
        return;
    };
    let graph_png = exec(
        dot_graph,
        &mut PrinterContext::default(),
        vec![Format::Png.into()],
    ).expect("Couldn't make png");

    let mut file = fs::OpenOptions::new()
        .create(true) // To create a new file
        .write(true)
        // either use the ? operator or unwrap since it returns a Result
        .open("graph.png").expect("Couldn't open graph.png");

    file.write_all(&graph_png).expect("Writing to file failed");

    println!("graph.png has been created.");
}
