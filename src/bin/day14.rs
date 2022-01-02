use std::collections::HashMap;

fn apply_rules(s: String, rules: &HashMap<&str, &str>, n: u64) -> u64 {
    let mut counts = HashMap::new();
    for i in 0..s.len() - 1 {
        *counts.entry(s[i..i + 2].to_string()).or_insert(0) += 1;
    }

    for _ in 0..n {
        let mut new_counts = HashMap::new();
        for (pair, count) in counts.iter() {
            let left = format!("{}{}", rules.get(&pair[..]).unwrap(), &pair[1..2]);
            *new_counts.entry(left).or_insert(0) += count;
            let right = format!("{}{}", &pair[0..1], rules.get(&pair[..]).unwrap());
            *new_counts.entry(right).or_insert(0) += count;
        }
        counts = new_counts;
    }

    let mut letter_counts = HashMap::new();

    for (pair, count) in counts.iter() {
        *letter_counts.entry(&pair[0..1]).or_insert(0) += count;
        *letter_counts.entry(&pair[1..2]).or_insert(0) += count;
    }

    letter_counts.values().max().unwrap() / 2 - letter_counts.values().min().unwrap() / 2
}

fn main() {
    let rules: HashMap<&str, &str> = HashMap::from_iter(
        include_str!("../../data/day14.txt")
            .split('\n')
            .map(|line| line.split_once(" -> ").unwrap()),
    );
    // might be off by 1
    println!(
        "{:?}",
        apply_rules("NBOKHVHOSVKSSBSVVBCS".to_string(), &rules, 10) - 1
    );
    println!(
        "{:?}",
        apply_rules("NBOKHVHOSVKSSBSVVBCS".to_string(), &rules, 40) - 1
    );
}
