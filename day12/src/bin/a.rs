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
        Graph { edges: HashMap::new(), empty: Vec::new() }
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        let entry = self.edges.entry(from.to_string());
        entry.or_default().push(to.to_string());
    }

    fn out_nodes(&self, from: &str) -> impl Iterator<Item = &String> {
        self.edges.get(from).map_or_else(|| self.empty.iter(), |v| v.iter())
    }
}

fn all_paths(g: &Graph) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let mut queue = vec![vec![String::from("start")]];
    while let Some(path) = queue.pop() {
        let head = path.last().expect("Cannot have an empty path");
        if head == "end" {
            paths.push(path);
            continue;
        }
        let tails = g.out_nodes(head)
            .filter(|s| has_uppercase(s) || !path.contains(s));
        for tail in tails {
            let mut new_path = path.clone();
            new_path.push(tail.to_string());
            queue.push(new_path);
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
