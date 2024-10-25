use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
pub struct Graph<VId, V = ()> {
    pub(crate) vertices: HashMap<VId, V>,
    pub(crate) successor_edges: HashMap<VId, HashSet<VId>>,
    pub(crate) predecessor_edges: HashMap<VId, HashSet<VId>>,
}

impl<VId, V> Graph<VId, V>
    where
        VId: Eq + Hash,
        V: Hash,
{
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new(),
            successor_edges: HashMap::new(),
            predecessor_edges: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: VId, value: V) {
        self.vertices.insert(vertex, value);
    }
}

impl<VId, V> Graph<VId, V>
where
    VId: Eq + Hash + Clone,
    V: Hash + Clone,
{
    pub fn is_acyclic(&self) -> bool {
        let Some(first_node) = self.vertices.iter().next() else {
            return true;
        };

        let all_nodes = self.vertices.clone().into_keys().collect::<HashSet<VId>>();

        self.connected_nodes(first_node.0.clone(), HashSet::new()) == all_nodes
    }

    fn connected_nodes(&self, id: VId, visited_nodes: HashSet<VId>) -> HashSet<VId> {
        if visited_nodes.contains(&id) {
            return visited_nodes;
        }
        // add id to visited_nodes
        let visited_nodes: HashSet<VId> = visited_nodes.union(&HashSet::from([id.clone()])).cloned().collect();

        let default_set = HashSet::new();
        let predecessor_ids = self.predecessor_edges.get(&id).unwrap_or(&default_set);
        let default_set = HashSet::new();
        let successor_ids = self.successor_edges.get(&id).unwrap_or(&default_set);
        let directly_connected_nodes = predecessor_ids.union(&successor_ids);

        directly_connected_nodes
            .map(|node| self.connected_nodes(node.clone(), visited_nodes.clone()))
            .fold(HashSet::new(), |a, b| a.union(&b).cloned().collect())
    }
}

impl<VId, V> Graph<VId, V>
    where
        VId: Eq + Hash + Clone,
        V: Hash,
{
    pub fn connect_vertices(&mut self, from: VId, to: VId) {
        self.successor_edges.entry(from.clone()).or_default().insert(to.clone());
        self.predecessor_edges.entry(to).or_default().insert(from);
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn normal_graph_works() {
        let mut graph = Graph::new();

        graph.add_vertex("a", 1);
        graph.add_vertex("b", 2);
        graph.add_vertex("c", 3);
        graph.add_vertex("d", 4);

        graph.connect_vertices("a", "b");
        graph.connect_vertices("a", "c");
        graph.connect_vertices("b", "d");
        graph.connect_vertices("c", "d");

        assert!(graph.is_acyclic());

        assert_eq!(graph.connected_nodes("a", HashSet::new()), HashSet::from(["a", "b", "c", "d"]));
    }
}