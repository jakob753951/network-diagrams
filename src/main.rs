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
    tasks: Vec<NaiveTask>,
    connections: Vec<(usize, usize)>,
}

#[derive(Serialize, Deserialize)]
#[derive(Hash)]
#[derive(Clone)]
struct NaiveTask {
    id: usize,
    description: String,
    duration: i32,
}

impl<VId> Graph<VId, NaiveTask>
    where
        VId: Eq + Hash + Clone
{
    fn early_start(&self, node: VId) -> i32 {
        self.predecessor_edges.get(&node)
            .unwrap_or(&HashSet::new())
            .iter()
            .map(|predecessor| self.early_finish(predecessor.clone()))
            .max()
            .unwrap_or(0)
    }

    fn early_finish(&self, node: VId) -> i32 {
        (self.early_start(node.clone()) + self.vertices[&node].duration) as i32
    }

    fn late_start(&self, node: VId) -> i32 {
        (self.late_finish(node.clone()) - self.vertices[&node].duration) as i32
    }

    fn late_finish(&self, node: VId) -> i32 {
        self.successor_edges.get(&node)
            .unwrap_or(&HashSet::new())
            .iter()
            .map(|successor| self.late_start(successor.clone()))
            .min()
            .unwrap_or(self.early_finish(node))
    }

    fn slack(&self, node: VId) -> i32 {
        self.late_start(node.clone()) - self.early_start(node)
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
// impl Task {
//     fn early_start(&self) -> i32 {
//         self.predecessors
//             .iter()
//             .map(|predecessor| predecessor.early_finish())
//             .max()
//             .unwrap_or(0)
//     }
//
//     fn early_finish(&self) -> i32 {
//         (self.early_start() + self.duration) as i32
//     }
//
//     fn late_start(&self) -> i32 {
//         (self.late_finish() - self.duration) as i32
//     }
//
//     fn late_finish(&self) -> i32 {
//         self.successors
//             .iter()
//             .map(|successor| successor.late_start())
//             .min()
//             .unwrap_or(self.early_finish())
//     }
//
//     fn slack(&self) -> i32 {
//         self.late_start() - self.early_start()
//     }
// }

impl Display for NodeData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "┌─────────┬─────────┬─────────┐")?;
        writeln!(
            f,
            "│ {:^7} │ {:^7} │ {:^7} │",
            self.early_start,
            self.id,
            self.early_finish
        )?;
        writeln!(f, "├─────────┼─────────┴─────────┤")?;
        writeln!(f, "│ {:^7} │ {:^17} │", self.slack, self.description)?;
        writeln!(f, "├─────────┼─────────┬─────────┤")?;
        writeln!(
            f,
            "│ {:^7} │ {:^7} │ {:^7} │",
            self.late_start,
            self.duration,
            self.late_finish
        )?;
        write!(f, "└─────────┴─────────┴─────────┘")?;
        Ok(())
    }
}

fn node_data(graph: &Graph<usize, NaiveTask>, id: usize) -> NodeData {
    let node = graph.vertices[&id].clone();

    NodeData {
        early_start: graph.early_start(id.clone()),
        id: node.id,
        early_finish: graph.early_finish(id.clone()),
        slack: graph.slack(id.clone()),
        description: node.description,
        late_start: graph.late_start(id.clone()),
        duration: node.duration,
        late_finish: graph.late_finish(id.clone()),
    }
}

fn main() {
    let mut file = File::open("config.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let val: Config = serde_json::from_str(contents.as_str()).unwrap();

    let mut graph: Graph<usize, NaiveTask> = Graph::new();

    val.tasks.iter().for_each(|task| graph.add_vertex(task.id, task.clone()));

    val.connections.iter().for_each(|(from, to)| graph.connect_vertices(*from, *to));

    let node_data = node_data(&graph, 6);
    println!("{node_data}");
}
