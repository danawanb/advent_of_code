use std::collections::VecDeque;
use std::fs;

pub fn day_eighteen() -> usize {
    //let txt = fs::read_to_string("./src/day18_dummy.txt");
    let txt = fs::read_to_string("./src/day18.txt");
    let mut coord: Vec<String> = Vec::new();
    let mut coordx: Vec<Coord> = Vec::new();
    if let Ok(xx) = txt {
        coord = xx.split_ascii_whitespace().map(|f| f.to_string()).collect();
    }

    for cor in coord {
        let co: Vec<&str> = cor.split(",").collect();
        let mut corx = Coord { x: 0, y: 0 };
        if let Some(x) = co.get(0) {
            corx.x = x.parse().unwrap();
        }
        if let Some(y) = co.get(1) {
            corx.y = y.parse().unwrap();
        }
        coordx.push(corx);
    }

    let mut gridx: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];

    for (idc, cor) in coordx.iter().enumerate() {
        if idc > 1023 {
            gridx[cor.y][cor.x] = '.';
        } else {
            gridx[cor.y][cor.x] = '#';
        }
    }

    let start = Coord { x: 0, y: 0 };
    let end = Coord { x: 70, y: 70 };

    let mut stepx = 0;

    //part1
    //if let Some(steps) = bfs(&gridx, start, end, '#') {
    //    stepx = steps;
    //}

    //part2
    for (_, cor) in coordx.iter().enumerate() {
        gridx[cor.y][cor.x] = '#';

        if !bfs2(&gridx, Coord { x: 0, y: 0 }, Coord { x: 70, y: 70 }, '#') {
            println!("{} {}", cor.x, cor.y);
            return 0;
        }
    }

    stepx
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: usize,
    y: usize,
}

//xy
const DIRECTION: [[i32; 2]; 4] = [
    [1, 0],  //kanan
    [0, 1],  //bawah
    [0, -1], //atas
    [-1, 0], //kiri
];

fn bfs(grid: &Vec<Vec<char>>, start: Coord, end: Coord, wall: char) -> Option<usize> {
    let mut queue: VecDeque<(Coord, usize)> = VecDeque::new();
    let mut seen: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    queue.push_back((start, 0));
    seen[start.y][start.x] = true;

    while let Some((curr, steps)) = queue.pop_front() {
        if curr.x == end.x && curr.y == end.y {
            return Some(steps);
        }

        for dir in &DIRECTION {
            let nx = curr.x as i32 + dir[0];
            let ny = curr.y as i32 + dir[1];

            if nx >= 0 && nx < grid[0].len() as i32 && ny >= 0 && ny < grid.len() as i32 {
                let next = Coord {
                    x: nx as usize,
                    y: ny as usize,
                };

                if !seen[next.y][next.x] && grid[next.y][next.x] != wall {
                    seen[next.y][next.x] = true;
                    queue.push_back((next, steps + 1));
                }
            }
        }
    }

    None
}

fn bfs2(grid: &Vec<Vec<char>>, start: Coord, end: Coord, wall: char) -> bool {
    let mut queue: VecDeque<Coord> = VecDeque::new();
    let mut seen: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];

    queue.push_back(start);
    seen[start.y][start.x] = true;

    while let Some(curr) = queue.pop_front() {
        if curr.x == end.x && curr.y == end.y {
            return true;
        }

        for dir in &DIRECTION {
            let nx = curr.x as i32 + dir[0];
            let ny = curr.y as i32 + dir[1];

            if nx >= 0 && nx < grid[0].len() as i32 && ny >= 0 && ny < grid.len() as i32 {
                let next = Coord {
                    x: nx as usize,
                    y: ny as usize,
                };

                if !seen[next.y][next.x] && grid[next.y][next.x] != wall {
                    seen[next.y][next.x] = true;
                    queue.push_back(next);
                }
            }
        }
    }

    false
}
