use std::collections::HashMap;

fn is_lower(s: &str) -> bool {
    s == s.to_ascii_lowercase()
}

fn dfs<'a>(
    current_node: &'a str,
    graph: &HashMap<&str, Vec<&'a str>>,
    visited: &mut HashMap<&'a str, i64>,
    twice: bool,
) -> u64 {
    if current_node == "end" {
        return 1;
    }

    let mut total = 0;
    *visited.entry(current_node).or_insert(0) += 1;
    for &n in graph.get(current_node).unwrap() {
        if n != "start" {
            let count: i64 = *visited.get(n).unwrap_or(&0);
            let new_twice = twice || (is_lower(n) && count == 1);
            if !is_lower(n) || count == 0 || (count == 1 && !twice) {
                total += dfs(n, graph, visited, new_twice);
            }
        }
    }
    *visited.entry(current_node).or_insert(0) -= 1;
    total
}

fn part1(graph: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut visited = HashMap::new();
    dfs("start", graph, &mut visited, true)
}
fn part2(graph: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut visited = HashMap::new();
    dfs("start", graph, &mut visited, false)
}

fn main() {
    let mut cave_graph = HashMap::new();
    for line in include_str!("../../data/day12.txt").split('\n') {
        let (v1, v2) = line.split_once('-').unwrap();
        cave_graph.entry(v1).or_insert(Vec::new()).push(v2);
        cave_graph.entry(v2).or_insert(Vec::new()).push(v1);
    }
    println!("{:?}", part1(&cave_graph));
    println!("{:?}", part2(&cave_graph));
}
