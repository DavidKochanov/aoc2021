fn part1(numbers: &[i32], boards: &mut Vec<Board>) -> i32 {
    for &num in numbers {
        for board in boards.iter_mut() {
            board.call(num);
            if board.has_won() {
                return board.score() * num;
            }
        }
    }
    0
}

fn part2(numbers: &[i32], boards: Vec<Board>) -> i32 {
    let mut boards = boards;
    let mut left = boards.len();
    for &num in numbers {
        let mut new_boards = Vec::new();
        let l = boards.len();
        for board in boards.iter_mut() {
            board.call(num);
            if board.has_won() {
                if left == 1 {
                    return board.score() * num;
                }
                left -= 1;
            } else {
                new_boards.push(*board);
            }
        }
        boards = new_boards;
    }
    0
}

#[derive(Clone, Copy, Debug)]
struct Board {
    numbers: [i32; 25],
    called: [bool; 25],
}
impl Board {
    fn new(_numbers: &[i32]) -> Self {
        let mut b = Self {
            numbers: [0; 25],
            called: [false; 25],
        };
        b.numbers.clone_from_slice(_numbers);
        b
    }

    fn call(&mut self, num: i32) {
        for (number, called) in self.numbers.iter().zip(self.called.iter_mut()) {
            if *number == num {
                *called = true;
            }
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..5 {
            if (0..5).all(|j| self.called[j * 5 + i]) {
                return true;
            }
        }
        for i in 0..5 {
            if (0..5).all(|j| self.called[i * 5 + j]) {
                return true;
            }
        }
        false
    }

    fn score(&self) -> i32 {
        self.numbers
            .iter()
            .zip(self.called.iter())
            .filter(|&(_, called)| !called)
            .fold(0, |acc, (number, _)| acc + number)
    }
}

fn main() {
    let mut lines = include_str!("../../data/day4.txt").split('\n');
    let numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let boards = lines
        .flat_map(|x| x.split_whitespace())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards: Vec<Board> = boards.chunks(25).map(|chunk| Board::new(chunk)).collect();

    println!("{:?}", part1(&numbers, &mut boards));
    println!("{:?}", part2(&numbers, boards));
}
