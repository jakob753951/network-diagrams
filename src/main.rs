use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::hash::Hash;
use std::io::Read;

mod graph;

use graph::Graph;

#[derive(Serialize, Deserialize)]
struct Config {
    tasks: Vec<Task>,
    connections: Vec<(usize, usize)>,
}

#[derive(Serialize, Deserialize)]
#[derive(Hash)]
#[derive(Clone)]
struct Task {
    id: usize,
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
        (self.early_start(&node) + self.vertices[&node].duration) as i32
    }

    fn late_start(&self, node: &VId) -> i32 {
        (self.late_finish(&node) - self.vertices[&node].duration) as i32
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

impl<VId> Display for Graph<VId, Task>
    where
        VId: Eq + Hash + Clone + Display
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "strict digraph network_diagram {{")?;
        writeln!(f, "   node [shape=plaintext]")?;

        self.vertices.iter().try_for_each(|(id, node)| {
            writeln!(f, r#"    node_{id} [label=<
                    <table border="0" cellborder="1" cellspacing="0">
                        <tr height="30px"><td width="40px">{early_start}</td><td>{id}</td><td>{early_finish}</td></tr>
                        <tr height="30px"><td width="40px">{slack}</td><td colspan="2">{description}</td></tr>
                        <tr height="30px"><td width="40px">{late_start}</td><td>{duration}</td><td>{late_finish}</td></tr>
                    </table>
                >];"#,
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
                    writeln!(f, r#"   node_{from} -> node_{to} [color="red" penwidth="2"]"#, from = from, to = to)
                } else {
                    writeln!(f, "   node_{from} -> node_{to}", from = from, to = to)
                }
            })
        })?;

        writeln!(f, "}}")?;
        Ok(())
    }
}

struct NodeData {
    early_start: i32,
    id: usize,
    early_finish: i32,
    slack: i32,
    description: String,
    late_start: i32,
    duration: i32,
    late_finish: i32,
}

impl NodeData {
    fn new_from_graph_node(graph: &Graph<usize, Task>, id: usize) -> NodeData {
        let node = &graph.vertices[&id];

        NodeData {
            early_start: graph.early_start(&id),
            id: node.id,
            early_finish: graph.early_finish(&id),
            slack: graph.slack(&id),
            description: node.description.clone(),
            late_start: graph.late_start(&id),
            duration: node.duration,
            late_finish: graph.late_finish(&id),
        }
    }
}

impl Display for NodeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+{0:->10}{0:->15}{0:->15}", "+")?;
        writeln!(f, "| {:^7} | {:^12} | {:^12} |", self.early_start, self.id, self.early_finish)?;
        writeln!(f, "+{0:->10}{0:->15}{0:->15}", "+")?;
        writeln!(f, "| {:^7} | {:^17} |", self.slack, self.description)?;
        writeln!(f, "+{0:->10}{0:->15}{0:->15}", "+")?;
        writeln!(f, "| {:^7} | {:^12} | {:^12} |", self.late_start, self.duration, self.late_finish)?;
        writeln!(f, "+{0:->10}{0:->15}{0:->15}", "+")?;
        Ok(())
    }
}

fn main() {
    let mut file = File::open("config.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let val: Config = serde_json::from_str(contents.as_str()).unwrap();

    let mut graph: Graph<usize, Task> = Graph::new();

    val.tasks.iter().for_each(|task| graph.add_vertex(task.id, task.clone()));

    val.connections.iter().for_each(|(from, to)| graph.connect_vertices(*from, *to));

    // let node_data = NodeData::new_from_graph_node(&graph, 8);
    println!("{graph}");
}
