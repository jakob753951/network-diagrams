use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[derive(Hash)]
#[derive(Clone)]
pub(crate) struct Task {
    pub(crate) id: String,
    pub(crate) description: String,
    pub(crate) duration: i32,
    pub(crate) predecessors: Vec<String>,
}

impl<VId> crate::graph::Graph<VId, Task>
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

impl Display for crate::graph::Graph<String, Task>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "strict digraph network_diagram {{")?;
        writeln!(f, "    bgcolor=transparent")?;
        writeln!(f, "    node [shape=plaintext bgcolor=white]")?;

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