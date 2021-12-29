fn get_score(line: &str) -> (i64, i64) {
    let mut symbol_stack = Vec::new();
    for c in line.chars().into_iter() {
        match c {
            '(' | '[' | '{' | '<' => symbol_stack.push(c),
            ')' => {
                if symbol_stack.last() != Some(&'(') {
                    return (3, -1);
                } else {
                    symbol_stack.pop();
                }
            }
            ']' => {
                if symbol_stack.last() != Some(&'[') {
                    return (57, -1);
                } else {
                    symbol_stack.pop();
                }
            }
            '}' => {
                if symbol_stack.last() != Some(&'{') {
                    return (1197, -1);
                } else {
                    symbol_stack.pop();
                }
            }
            '>' => {
                if symbol_stack.last() != Some(&'<') {
                    return (25137, -1);
                } else {
                    symbol_stack.pop();
                }
            }
            _ => panic!("ouch"),
        };
    }
    let mut stack_score = 0;
    while let Some(c) = symbol_stack.pop() {
        stack_score = 5 * stack_score
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Ouch"),
            }
    }
    (0, stack_score)
}

fn part1(lines: &[&str]) -> i64 {
    lines.iter().map(|&l| get_score(l).0).sum()
}

fn part2(lines: &[&str]) -> i64 {
    let mut scores: Vec<i64> = lines
        .iter()
        .map(|&l| get_score(l).1)
        .filter(|&score| score >= 0)
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let lines: Vec<&str> = include_str!("../../data/day10.txt").split('\n').collect();
    println!("{:?}", part1(&lines));
    println!("{:?}", part2(&lines));
}
