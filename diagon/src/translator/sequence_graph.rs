use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct Node {
    actor: i32,
    message: i32,
}

impl Node {
    pub fn new(actor: i32, msg: Option<i32>) -> Self {
        Self {
            actor,
            message: msg.unwrap_or(0),
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        (self.actor == other.actor) && (self.message == other.message)
    }
}
impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if Ordering::Equal == self.actor.cmp(&other.actor) {
            self.message.cmp(&other.message)
        } else {
            self.actor.cmp(&other.actor)
        }
    }
}

#[derive(Debug, Clone)]
struct Message {
    id: i32,   // define sequence message
    from: i32, // define actor from node
    to: i32,   // define actor to node
}

#[derive(Debug)]
struct Edge {
    from: Node,
    to: Node,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        (self.from == other.from) && (self.to == other.to)
    }
}
impl Eq for Edge {}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.from == other.from {
            self.to.cmp(&other.to)
        } else {
            self.from.cmp(&other.from)
        }
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Edge {
    pub fn new(from: Node, to: Node) -> Self {
        Self { from, to }
    }

    pub fn new_from_message(msg: Message) -> Self {
        Self {
            from: Node {
                actor: msg.from,
                message: msg.id,
            },
            to: Node {
                actor: msg.to,
                message: msg.id,
            },
        }
    }
}

pub struct Graph(BTreeSet<Edge>);

impl Graph {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }
}

pub fn find_topological_order(graph: &Graph) -> Vec<Node> {
    let mut weight: BTreeMap<Node, i32> = BTreeMap::new();
    let mut work_to_do = true;
    let mut iteration_count = 0;
    while work_to_do {
        work_to_do = false;
        for vertex in &graph.0 {
            let mut to_weight = 0;
            let mut from_weight = 0;
            {
                to_weight = *weight.get(&vertex.to).unwrap_or(&0);
                from_weight = *weight.get(&vertex.from).unwrap_or(&0);
            }
            if to_weight <= from_weight {
                weight.insert(vertex.to.clone(), from_weight + 1);
                work_to_do = true;
            }
        }

        iteration_count += 1;
        if iteration_count >= 1000 {
            println!("There are cycles {}", iteration_count);
            break;
        }
    }

    let mut res: Vec<Node> = weight.iter().map(|(node, _)| node.clone()).collect();
    res.sort_by(|a, b| weight.get(a).unwrap_or(&0).cmp(weight.get(b).unwrap_or(&0)));
    res
}

#[cfg(test)]
mod tests {
    use super::{Edge, Graph, Node};

    #[test]
    fn test_topological_order() {
        let node1 = Node::new(1, None);
        let node2 = Node::new(2, None);
        let node3 = Node::new(3, None);
        let node4 = Node::new(4, None);
        let node5 = Node::new(5, None);

        let edge1 = Edge::new(node1.clone(), node2.clone());
        let edge2 = Edge::new(node2.clone(), node3.clone());
        let edge3 = Edge::new(node3.clone(), node4.clone());
        let edge4 = Edge::new(node4.clone(), node5.clone());

        let mut graph = Graph::new();
        graph.0.insert(edge1);
        graph.0.insert(edge2);
        graph.0.insert(edge3);
        graph.0.insert(edge4);

        let expected_vec = vec![node2, node3, node4, node5];

        let res_vec = super::find_topological_order(&graph);

        assert_eq!(res_vec, expected_vec);
    }
}
