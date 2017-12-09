use std::cmp::{Ord, PartialOrd, Eq, PartialEq, Ordering};
use std::collections::BinaryHeap;

pub enum Node {
    Leaf(u8),
    Parent { left: Box<Node>, right: Box<Node> },
}

struct NodeFreqPair {
    freq: u64,
    node: Node,
}

impl PartialEq for NodeFreqPair {
    fn eq(&self, other: &NodeFreqPair) -> bool {
        self.freq == other.freq
    }
}

impl Eq for NodeFreqPair {}

impl PartialOrd for NodeFreqPair {
    fn partial_cmp(&self, other: &NodeFreqPair) -> Option<Ordering> {
        Some(other.freq.cmp(&self.freq))
    }
}

impl Ord for NodeFreqPair {
    fn cmp(&self, other: &NodeFreqPair) -> Ordering {
        other.freq.cmp(&self.freq)
    }
}

pub fn build_tree(freq_table: [u64; 256]) -> Option<Node> {
    let mut q = BinaryHeap::new();

    for (symbol, &freq) in freq_table.iter().enumerate() {
        if freq > 0 {
            q.push(NodeFreqPair {
                freq: freq,
                node: Node::Leaf(symbol as u8),
            });
        }
    }

    while q.len() > 1 {
        if let Some(x) = q.pop() {
            if let Some(y) = q.pop() {
                q.push(NodeFreqPair {
                    freq: x.freq + y.freq,
                    node: Node::Parent {
                        left: Box::new(x.node),
                        right: Box::new(y.node),
                    },
                });
            }
        }
    }

    q.pop().map(|x| x.node)
}
