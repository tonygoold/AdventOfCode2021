use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Clone, Copy)]
enum Node {
    Leaf(u8),
    Branch,
    Empty,
}

impl Node {
    fn is_empty(&self) -> bool {
        matches!(*self, Node::Empty)
    }
}

#[derive(Clone)]
struct NodeRef<'a> {
    tree: &'a VecTree,
    index: usize,
}

impl<'a> NodeRef<'a> {
    fn node(&self) -> Node {
        self.tree.values[self.index]
    }

    fn left(&self) -> Option<Self> {
        match self.node() {
            Node::Branch => Some(Self {
                tree: self.tree,
                index: self.index * 2 + 1,
            }),
            _ => None,
        }
    }

    fn right(&self) -> Option<Self> {
        match self.node() {
            Node::Branch => Some(Self {
                tree: self.tree,
                index: self.index * 2 + 2,
            }),
            _ => None,
        }
    }

    fn magnitude(&self) -> usize {
        match self.node() {
            Node::Empty => panic!("Cannot get magnitude of empty node"),
            Node::Leaf(value) => value as usize,
            Node::Branch => {
                let l = self.left().expect("Branch must have left child");
                let r = self.right().expect("Branch must have right child");
                3 * l.magnitude() + 2 * r.magnitude()
            }
        }
    }
}

impl<'a> fmt::Display for NodeRef<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.node() {
            Node::Empty => Ok(()),
            Node::Leaf(n) => write!(f, "{}", n),
            Node::Branch => {
                write!(f, "[")?;
                if let Some(left) = self.left() {
                    left.fmt(f)?;
                }
                write!(f, ",")?;
                if let Some(right) = self.right() {
                    right.fmt(f)?;
                }
                write!(f, "]")
            }
        }
    }
}

// A binary tree stored in a vector, with values only at the leaf nodes.
struct VecTree {
    values: Vec<Node>,
}

impl VecTree {
    fn new() -> Self {
        Self { values: Vec::new() }
    }

    fn len(&self) -> usize {
        let mut len = self.values.len();
        while len > 0 && self.values[len - 1].is_empty() {
            len -= 1;
        }
        len
    }

    fn height(&self) -> usize {
        let mut rem = self.len();
        let mut height = 0;
        while rem > 0 {
            height += 1;
            rem >>= 1;
        }
        height
    }

    fn magnitude(&self) -> usize {
        if self.values.is_empty() {
            0
        } else {
            self.root_ref().magnitude()
        }
    }

    fn root_ref(&self) -> NodeRef {
        NodeRef {
            tree: self,
            index: 0,
        }
    }

    fn iter(&self) -> TreeIter {
        TreeIter::new(self.root_ref())
    }

    fn ensure_capacity(&mut self, capacity: usize) {
        let mut capacity = capacity;
        let mut required = 1;
        while capacity != 0 {
            capacity /= 2;
            required *= 2;
        }
        if self.values.len() < required {
            self.values.resize(required, Node::Empty);
        }
    }

    fn insert_value(&mut self, index: usize, value: u8) {
        // TODO: If we already have a branch here, clear its children too
        self.ensure_capacity(index + 1);
        self.values[index] = Node::Leaf(value);
        // Ensure all parent nodes are treated as branches
        let mut index = index;
        while index > 0 {
            index = (index - 1) / 2;
            self.values[index] = Node::Branch;
        }
    }

    fn insert_node(&mut self, index: usize, node: NodeRef) {
        match node.node() {
            Node::Empty => {}
            Node::Leaf(value) => self.insert_value(index, value),
            Node::Branch => {
                self.ensure_capacity(index * 2 + 3);
                self.values[index] = Node::Branch;
                if let Some(left) = node.left() {
                    self.insert_node(index * 2 + 1, left);
                } else {
                    self.delete_node(index * 2 + 1);
                }
                if let Some(right) = node.right() {
                    self.insert_node(index * 2 + 2, right);
                } else {
                    self.delete_node(index * 2 + 2);
                }
            }
        }
    }

    fn delete_node(&mut self, index: usize) {
        // To optimize: Shrink when a row is empty and the previous contains no branches.
        if index < self.values.len() {
            self.delete_node(index * 2 + 1);
            self.delete_node(index * 2 + 2);
            self.values[index] = Node::Empty;
        }
    }

    fn add(&self, rhs: &Self) -> Self {
        let mut tree = self.join(rhs);
        tree.reduce();
        tree
    }

    fn join(&self, rhs: &Self) -> Self {
        // let height = self.height() + rhs.height();
        let (h1, h2) = (self.height(), rhs.height());
        let height = 1 + if h1 > h2 { h1 } else { h2 };
        let capacity = 1 << height;
        let mut values = Vec::new();
        values.resize(capacity, Node::Empty);
        values[0] = Node::Branch;
        let mut tree = VecTree { values };
        tree.insert_node(1, self.root_ref());
        tree.insert_node(2, rhs.root_ref());
        tree
    }

    fn reduce(&mut self) {
        loop {
            if !self.explode() && !self.split() {
                break;
            }
        }
    }

    fn explode(&mut self) -> bool {
        let height = self.height();
        // A "pair nested inside four pairs" means we're looking for a branch node at the
        // fifth rank, which implies leaf nodes at the sixth rank.
        match height.cmp(&6) {
            Ordering::Less => return false,
            Ordering::Greater => panic!("Tree should never reach height of {}", height),
            _ => {}
        }
        // Stop at the first branch node
        for index in 15..31 {
            if let Node::Branch = &self.values[index] {
                self.explode_node(index);
                return true;
            }
        }
        false
    }

    fn explode_node(&mut self, index: usize) {
        let left_index = index * 2 + 1;
        let right_index = index * 2 + 2;
        let left = match self.values[left_index] {
            Node::Leaf(value) => value,
            _ => panic!("Expected leaf node as left child of exploding node"),
        };
        let right = match self.values[right_index] {
            Node::Leaf(value) => value,
            _ => panic!("Expected leaf node as right child of exploding node"),
        };
        let mut left_iter = self.iter().reverse();
        while let Some(node) = left_iter.next() {
            if node.index == left_index {
                if let Some(prev) = left_iter.next() {
                    let index = prev.index;
                    self.increase_node(index, left);
                }
                break;
            }
        }
        let mut right_iter = self.iter();
        while let Some(node) = right_iter.next() {
            if node.index == right_index {
                if let Some(next) = right_iter.next() {
                    let index = next.index;
                    self.increase_node(index, right);
                }
                break;
            }
        }
        self.delete_node(left_index);
        self.delete_node(right_index);
        self.values[index] = Node::Leaf(0);
    }

    fn increase_node(&mut self, index: usize, value: u8) {
        match self.values[index] {
            Node::Leaf(existing) => self.values[index] = Node::Leaf(existing + value),
            _ => panic!("Cannot increment non-leaf"),
        }
    }

    fn split(&mut self) -> bool {
        for node in self.iter() {
            if let Node::Leaf(value) = node.node() {
                if value >= 10 {
                    let index = node.index;
                    self.split_node(index);
                    return true;
                }
            }
        }
        false
    }

    fn split_node(&mut self, index: usize) {
        let value = match self.values[index] {
            Node::Leaf(value) => value,
            _ => panic!("Expected a leaf node to split"),
        };
        let left_value = value / 2;
        let right_value = if value == 2 * left_value {
            left_value
        } else {
            left_value + 1
        };
        self.insert_value(index * 2 + 1, left_value);
        self.insert_value(index * 2 + 2, right_value);
    }
}

impl fmt::Display for VecTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.root_ref().fmt(f)
    }
}

#[derive(Clone)]
struct TreeIter<'a> {
    stack: Vec<NodeRef<'a>>,
    reversed: bool,
}

impl<'a> TreeIter<'a> {
    fn new(root: NodeRef<'a>) -> Self {
        TreeIter {
            stack: vec![root],
            reversed: false,
        }
    }

    fn reverse(&self) -> Self {
        let mut copy = self.clone();
        copy.reversed = !self.reversed;
        copy
    }
}

impl<'a> Iterator for TreeIter<'a> {
    type Item = NodeRef<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let node = self.stack.pop()?;
            match node.node() {
                Node::Branch => {
                    let left = node.left().expect("Branch must have left child");
                    if left.node().is_empty() {
                        panic!(
                            "Branch should not have an empty left child for tree {}",
                            node.tree.to_string()
                        );
                    }
                    let right = node.right().expect("Branch must have right child");
                    if right.node().is_empty() {
                        panic!(
                            "Branch should not have an empty right child for tree {}",
                            node.tree.to_string()
                        );
                    }
                    if self.reversed {
                        self.stack.push(left);
                        self.stack.push(right);
                    } else {
                        self.stack.push(right);
                        self.stack.push(left);
                    }
                }
                Node::Leaf(_) => return Some(node),
                Node::Empty => panic!("Encountered empty node while walking tree"),
            }
        }
    }
}

fn parse_tree(s: &str) -> VecTree {
    let mut tree = VecTree::new();
    let mut index = 0;
    for c in s.chars() {
        match c {
            '[' => index = index * 2 + 1,
            ']' => index = (index - 1) / 2,
            ',' => index += 1,
            '0'..='9' => tree.insert_value(index, c.to_digit(10).unwrap() as u8),
            _ => panic!("Invalid character in input: {}", c),
        }
    }
    tree
}

fn main() {
    let trees = app::read_lines(&app::input_arg()).map(|line| parse_tree(&line));

    let tree = trees
        .reduce(|a, b| a.add(&b))
        .expect("Input did not contain any trees");
    println!("Final tree is {}", tree.to_string());
    println!("Its magnitude is {}", tree.magnitude());
}
