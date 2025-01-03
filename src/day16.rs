use core::panic;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
    ops::Add,
};

pub fn day_sixteen() -> usize {
    //let txt = fs::read_to_string("./src/day16_dummy.txt");
    let txt = fs::read_to_string("./src/day16.txt");
    if let Ok(x) = txt {
        let mazex = Maze::parse(x);
        println!("{:?}", mazex);
        let count = mazex.dijkstra();
        let countx = mazex.dijkstra_with_backtrack(count);
        println!("{}", countx);
        return count;
    }

    67
}

#[derive(Debug)]
struct Maze {
    matrix: Vec<Vec<Object>>,
    start: (usize, usize),
    end: (usize, usize),
    height: usize,
    width: usize,
}

impl Maze {
    fn parse(valx: String) -> Self {
        let mut matrix: Vec<Vec<Object>> = Vec::new();
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (row, line) in valx.lines().enumerate() {
            let mut mat_row = Vec::new();

            for (col, char) in line.char_indices() {
                if char == 'S' {
                    start = (row, col);
                }
                if char == 'E' {
                    end = (row, col)
                }

                mat_row.push(Object::from(char));
            }

            matrix.push(mat_row);
        }

        let height = matrix.len();
        let width = matrix[0].len();

        Self {
            matrix,
            start,
            end,
            height,
            width,
        }
    }

    fn dijkstra(&self) -> usize {
        let mut min_cost = usize::MAX;
        let mut dist = vec![vec![usize::MAX; self.width]; self.height];
        let mut prio = BinaryHeap::new();
        dist[self.start.0][self.start.1] = 0;
        prio.push(Reverse(Tile {
            position: self.start,
            direction: Direction::Kanan,
            cost: 0,
            history: None,
        }));
        while let Some(Reverse(Tile {
            position,
            direction,
            cost,
            history,
        })) = prio.pop()
        {
            let (row, col) = position;
            if position == self.end && cost < min_cost {
                min_cost = cost;
                continue;
            }
            if cost > dist[row][col] || cost >= min_cost {
                continue;
            }

            for dir in Direction::ALL {
                let next_dir = dir;
                let (next_row, next_col) = position + next_dir;
                let mut next_cost = cost;
                if next_dir == direction {
                    next_cost += 1;
                } else {
                    next_dir.rotate();
                    next_cost += 1001;
                }
                if self.matrix[next_row][next_col] == Object::Wall {
                    continue;
                }

                if next_cost < dist[next_row][next_col] {
                    dist[next_row][next_col] = next_cost;
                    prio.push(Reverse(Tile {
                        position: (next_row, next_col),
                        direction: next_dir,
                        cost: next_cost,
                        history: None,
                    }));
                }
            }
        }
        min_cost
    }
    fn dijkstra_with_backtrack(&self, min_cost: usize) -> usize {
        let mut to_visit = vec![vec![[min_cost, min_cost]; self.width]; self.height];
        let mut prio = BinaryHeap::new();
        let mut tiles = HashSet::new();
        to_visit[self.start.0][self.start.1][Direction::get_axis(&Direction::Kanan)] = 0;
        prio.push(Reverse(Tile {
            position: self.start,
            direction: Direction::Kanan,
            cost: 0,
            history: Some(vec![]),
        }));
        while let Some(Reverse(Tile {
            position,
            direction,
            cost,
            history,
        })) = prio.pop()
        {
            let mut history = history.unwrap();
            history.push(position);

            let (row, col) = position;
            if cost > to_visit[row][col][direction.get_axis()] || cost > min_cost {
                continue;
            }

            if position == self.end {
                if cost == min_cost {
                    tiles.extend(history);
                }

                continue;
            }
            for dir in Direction::ALL {
                let next_dir = dir;
                let (next_row, next_col) = position + next_dir;
                let mut next_cost = cost;
                if next_dir == direction {
                    next_cost += 1;
                } else {
                    next_dir.rotate();
                    next_cost += 1001;
                }
                if self.matrix[next_row][next_col] == Object::Wall {
                    continue;
                }

                if next_cost <= to_visit[next_row][next_col][direction.get_axis()] {
                    to_visit[next_row][next_col][direction.get_axis()] = next_cost;
                    prio.push(Reverse(Tile {
                        position: (next_row, next_col),
                        direction: next_dir,
                        cost: next_cost,
                        history: Some(history.clone()),
                    }));
                }
            }
        }

        tiles.len()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Object {
    Empty,
    Wall,
    Start,
    End,
}

impl From<char> for Object {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'S' => Self::Start,
            'E' => Self::End,
            _ => panic!("value tidak ada {:?}", value),
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Tile {
    position: (usize, usize),
    direction: Direction,
    cost: usize,
    history: Option<Vec<(usize, usize)>>,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Atas,
    Kiri,
    Kanan,
    Bawah,
}

impl Direction {
    const ALL: [Direction; 4] = [
        Direction::Atas,
        Direction::Kanan,
        Direction::Bawah,
        Direction::Kiri,
    ];
    fn rotate(&self) -> Self {
        match self {
            Self::Atas => Self::Kanan,
            Self::Kanan => Self::Bawah,
            Self::Bawah => Self::Kiri,
            Self::Kiri => Self::Atas,
        }
    }
    fn get_axis(&self) -> usize {
        match self {
            Self::Atas | Self::Bawah => 0,
            Self::Kiri | Self::Kanan => 1,
        }
    }
}
impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);

    fn add(self, rhs: Direction) -> Self::Output {
        let (row, col) = self;
        match rhs {
            Direction::Atas => (row - 1, col),
            Direction::Kanan => (row, col + 1),
            Direction::Bawah => (row + 1, col),
            Direction::Kiri => (row, col - 1),
        }
    }
}
