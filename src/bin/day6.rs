fn simulate(fish: &[i32], days: i32) -> u64 {
    let mut current_fish: Vec<u64> = vec![0; 9];
    for &f in fish {
        current_fish[f as usize] += 1;
    }
    for _ in 0..days {
        let mut next_fish = vec![0; 9];
        next_fish[..8].clone_from_slice(&current_fish[1..]);
        next_fish[8] = current_fish[0];
        next_fish[6] += current_fish[0];
        current_fish = next_fish;
    }
    current_fish.iter().sum()
}

fn main() {
    let fish: Vec<i32> = include_str!("../../data/day6.txt")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{:?}", simulate(&fish, 80));
    println!("{:?}", simulate(&fish, 256));
}
