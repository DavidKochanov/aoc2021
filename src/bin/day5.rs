#[derive(Copy, Clone, Debug)]
struct Line(usize, usize, usize, usize);

fn draw_line(line: Line, board: &mut Vec<[i32; 1024]>) {
    let (dir_x, dir_y) = (
        (line.2 as i32 - line.0 as i32).signum(),
        (line.3 as i32 - line.1 as i32).signum(),
    );
    let mut c_x = line.0;
    let mut c_y = line.1;

    while c_x != line.2 || c_y != line.3 {
        board[c_x][c_y] += 1;
        c_x = (c_x as i32 + dir_x) as usize;
        c_y = (c_y as i32 + dir_y) as usize;
    }
    board[c_x][c_y] += 1;
}

fn part1(lines: &[Line]) -> i32 {
    let mut board = vec![[0; 1024]; 1024];
    for line in lines {
        if line.0 == line.2 || line.1 == line.3 {
            draw_line(*line, &mut board)
        }
    }
    board.iter().flatten().filter(|&x| x > &1).count() as i32
}

fn part2(lines: &[Line]) -> i32 {
    let mut board = vec![[0; 1024]; 1024];
    for line in lines {
        draw_line(*line, &mut board)
    }
    board.iter().flatten().filter(|&x| x > &1).count() as i32
}

fn main() {
    let parse_point = |pair: &str| {
        let (num1, num2) = pair.split_once(',').unwrap();
        let num1: usize = num1.parse().unwrap();
        let num2: usize = num2.parse().unwrap();
        (num1, num2)
    };
    let lines: Vec<Line> = include_str!("../../data/day5.txt")
        .split('\n')
        .map(|x| {
            let (point1, point2) = x.split_once(" -> ").unwrap();
            let (x1, y1) = parse_point(point1);
            let (x2, y2) = parse_point(point2);
            Line(x1, y1, x2, y2)
        })
        .collect();
    println!("{:?}", part1(&lines));
    println!("{:?}", part2(&lines));
}
