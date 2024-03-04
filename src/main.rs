use serde::{Deserialize, Serialize};
use std::fs::File;
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
//
// impl Display for Task {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         writeln!(f, "┌─────────┬─────────┬─────────┐")?;
//         writeln!(
//             f,
//             "│ {:^7} │ {:^7} │ {:^7} │",
//             self.early_start(),
//             self.id,
//             self.early_finish()
//         )?;
//         writeln!(f, "├─────────┼─────────┴─────────┤")?;
//         writeln!(f, "│ {:^7} │ {:^17} │", self.slack(), self.description)?;
//         writeln!(f, "├─────────┼─────────┬─────────┤")?;
//         writeln!(
//             f,
//             "│ {:^7} │ {:^7} │ {:^7} │",
//             self.late_start(),
//             self.duration,
//             self.late_finish()
//         )?;
//         write!(f, "└─────────┴─────────┴─────────┘")?;
//         Ok(())
//     }
// }

fn main() {
    let mut file = File::open("config.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let val: Config = serde_json::from_str(contents.as_str()).unwrap();

    let mut graph: Graph<usize, NaiveTask> = Graph::new();

    val.tasks.iter().for_each(|task| graph.add_vertex(task.id, task.clone()));

    val.connections.iter().for_each(|(from, to)| {
        graph.connect_vertices(*from, *to);
    });




    // let mut site_selection = Task::new(1, "Site Selection", 3);
    // let mut site_clearance = Task::new(2, "Site Clearance", 2);
    // let mut excavation = Task::new(3, "Excavation", 5);
    // let mut structural_framework = Task::new(4, "Structural Framework", 4);
    // let mut trap_installation = Task::new(5, "Trap Installation", 4);
    // let mut decorating = Task::new(6, "Decorating", 3);
    // let mut security_enhancement = Task::new(7, "Security Enhancement", 3);
    // let mut treasure_vault_construction = Task::new(8, "Treasure Vault Construction", 4);
    // let mut final_inspection = Task::new(9, "Final Inspection", 2);
    // let mut grand_opening = Task::new(10, "Grand Opening", 1);
    //
    // Task::connect(&mut site_selection, &mut site_clearance);
    // Task::connect(&mut site_clearance, &mut excavation);
    // Task::connect(&mut excavation, &mut structural_framework);
    // Task::connect(&mut structural_framework, &mut trap_installation);
    // Task::connect(&mut structural_framework, &mut decorating);
    // Task::connect(&mut structural_framework, &mut treasure_vault_construction);
    // Task::connect(&mut trap_installation, &mut security_enhancement);
    // Task::connect(&mut security_enhancement, &mut final_inspection);
    // Task::connect(&mut decorating, &mut final_inspection);
    // Task::connect(&mut treasure_vault_construction, &mut final_inspection);
    // Task::connect(&mut final_inspection, &mut grand_opening);

    // println!("{decorating}");
}
