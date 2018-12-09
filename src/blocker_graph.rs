use goji::Issue;
use std::borrow::Cow;
use std::cmp::Eq;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::hash::Hasher;

pub struct BlockerGraph {
    edges: HashSet<Edge>,
    nodes: HashMap<u64, Node>,
}

impl BlockerGraph {
    pub fn new() -> Self {
        BlockerGraph {
            edges: HashSet::<Edge>::new(),
            nodes: HashMap::<u64, Node>::new(),
        }
    }
    pub fn add(&mut self, from: &Issue, to: &Issue) {
        let from_id: u64 = from.id.parse().unwrap();
        let to_id: u64 = to.id.parse().unwrap();
        if self.edges.insert(Edge {
            from: from_id,
            to: to_id,
        }) {
            self.nodes.insert(from_id, Node::from_issue(from));
            self.nodes.insert(to_id, Node::from_issue(to));
        }
    }
}

impl<'a> dot::Labeller<'a, Node, Edge> for BlockerGraph {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new("Blockers").unwrap()
    }

    fn node_id(&'a self, n: &Node) -> dot::Id<'a> {
        dot::Id::new(format!("N{}", n.id)).unwrap()
    }

    fn node_label<'b>(&'b self, n: &Node) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(Cow::Owned(n.key.clone()))
    }

    fn node_color(&'a self, n: &Node) -> Option<dot::LabelText<'a>> {
        let color = match n.status.as_str() {
            "Closed" => Some("green"),
            "In Progress" => Some("blue"),
            "Review" => Some("goldenrod"),
            "Cancelled" => Some("firebrick"),
            _ => None,
        };
        match color {
            Some(c) => Some(dot::LabelText::LabelStr(c.into())),
            None => None,
        }
    }
}

impl<'a> dot::GraphWalk<'a, Node, Edge> for BlockerGraph {
    fn nodes(&'a self) -> dot::Nodes<'a, Node> {
        let node_values: Vec<&Node> = self.nodes.values().collect();
        let nodes = node_values.iter().map(|n| (*n).clone()).collect();
        Cow::Owned(nodes)
    }

    fn edges(&'a self) -> dot::Edges<'a, Edge> {
        let edges: Vec<Edge> = self.edges.iter().cloned().collect();
        Cow::Owned(edges)
    }

    fn source(&self, e: &Edge) -> Node {
        self.nodes[&e.from].clone()
    }

    fn target(&self, e: &Edge) -> Node {
        self.nodes[&e.to].clone()
    }
}

#[derive(Clone)]
pub struct Node {
    id: u64,
    key: String,
    status: String,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.id == other.id
    }
}

impl Eq for Node {}

impl Node {
    pub fn from_issue(issue: &Issue) -> Self {
        Node {
            id: issue.id.parse().unwrap(),
            key: issue.key.clone(),
            status: issue.status().unwrap().name,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Edge {
    from: u64,
    to: u64,
}
