use std::collections::HashMap;

fn has_uppercase(s: &str) -> bool {
    s.chars().any(char::is_uppercase)
}

struct Graph {
    edges: HashMap<String, Vec<String>>,
    empty: Vec<String>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            empty: Vec::new(),
        }
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        let entry = self.edges.entry(from.to_string());
        entry.or_default().push(to.to_string());
    }

    fn out_nodes(&self, from: &str) -> impl Iterator<Item = &String> {
        self.edges
            .get(from)
            .map_or_else(|| self.empty.iter(), |v| v.iter())
    }
}

#[derive(Clone)]
struct Path {
    nodes: Vec<String>,
    has_dup: bool,
}

impl Path {
    fn new(s: &str) -> Self {
        Path {
            nodes: vec![s.to_string()],
            has_dup: false,
        }
    }

    fn head(&self) -> &str {
        self.nodes.last().expect("Cannot have an empty path")
    }

    fn visit(&self, node: &str) -> Option<Self> {
        if has_uppercase(node) || !self.nodes.iter().any(|n| n == node) {
            let mut p = self.clone();
            p.nodes.push(node.to_string());
            Some(p)
        } else if !self.has_dup && node != "start" {
            let mut p = self.clone();
            p.has_dup = true;
            p.nodes.push(node.to_string());
            Some(p)
        } else {
            None
        }
    }
}

fn all_paths(g: &Graph) -> Vec<Path> {
    let mut paths = Vec::new();
    let mut queue = vec![Path::new("start")];
    while let Some(path) = queue.pop() {
        let head = path.head();
        if head == "end" {
            paths.push(path);
            continue;
        }
        for tail in g.out_nodes(head) {
            if let Some(new_path) = path.visit(tail) {
                queue.push(new_path);
            }
        }
    }
    paths
}

fn main() {
    let lines: Vec<String> = app::read_lines(&app::input_arg()).collect();
    let mut graph = Graph::new();
    for line in lines {
        let mut parts = line.split('-');
        let start = parts.next().expect("Split should always produce something");
        let end = parts.next().expect("Missing right side");
        graph.add_edge(start, end);
        graph.add_edge(end, start);
    }

    let paths = all_paths(&graph);
    println!("Found {} paths", paths.len());
}
