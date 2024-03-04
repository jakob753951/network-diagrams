use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
pub struct Graph<VId, V = ()> {
    vertices: HashMap<VId, V>,
    edges: HashMap<VId, HashSet<VId>>,
}

impl<VId, V> Graph<VId, V>
where
    VId: Eq + Hash,
    V: Hash,
{
    pub fn new() -> Self {
        Graph {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: VId, value: V) {
        self.vertices.insert(vertex, value);
    }

    pub fn add_edge(&mut self, from: VId, to: VId) {
        self.edges.entry(from).or_default().insert(to);
    }
}

impl<VId, V> Graph<VId, V>
where
    VId: Eq + Hash + Clone,
    V: Hash,
{
    pub fn add_undirected_edge(&mut self, a: VId, b: VId) {
        self.add_edge(a.clone(), b.clone());
        self.add_edge(b, a);
    }
}
