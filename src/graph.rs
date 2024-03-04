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

    pub fn add_predecessor(&mut self, from: VId, to: VId) {
        self.predecessor_edges.entry(from).or_default().insert(to);
    }
    pub fn add_successor(&mut self, from: VId, to: VId) {
        self.successor_edges.entry(from).or_default().insert(to);
    }
}

impl<VId, V> Graph<VId, V>
    where
        VId: Eq + Hash + Clone,
        V: Hash,
{
    pub fn connect_vertices(&mut self, from: VId, to: VId) {
        self.add_successor(from.clone(), to.clone());
        self.add_predecessor(to, from);
    }
}
