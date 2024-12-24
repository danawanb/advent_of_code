use std::{
    collections::{HashMap, HashSet},
    fs,
};
pub fn day_six_chapter_two() -> usize {
    let txt = fs::read_to_string("./src/day6.txt");
    let mut inputx: HashMap<(isize, isize), Tile> = HashMap::new();
    if let Ok(x) = txt {
        inputx = HashMap::from_iter(x.trim().lines().enumerate().flat_map(|(line_no, line)| {
            line.chars().enumerate().map(move |(tile_no, tile_char)| {
                let tile = match tile_char {
                    '.' => Tile::Empty,
                    '#' => Tile::Object,
                    '^' => Tile::Up,
                    '>' => Tile::Right,
                    '<' => Tile::Left,
                    'v' => Tile::Down,
                    _ => unreachable!(),
                };
                ((line_no as isize, tile_no as isize), tile)
            })
        }));
    }

    let mut guard_route = inputx;
    let mut obstacles = HashSet::new();
    let mut visited = HashSet::new();
    let (mut guard_pos, mut guard_facing) = guard_route
        .iter()
        .find_map(|(pos, tile)| match tile {
            Tile::Up | Tile::Down | Tile::Left | Tile::Right => Some((*pos, *tile)),
            _ => None,
        })
        .unwrap();
    guard_route.insert(guard_pos, Tile::Empty);
    let invalid_obstacle = guard_pos;

    while guard_route.contains_key(&guard_pos) {
        visited.insert(guard_pos);
        let (next_pos, next_face) = match guard_facing {
            Tile::Up => ((guard_pos.0 - 1, guard_pos.1), Tile::Right),
            Tile::Down => ((guard_pos.0 + 1, guard_pos.1), Tile::Left),
            Tile::Left => ((guard_pos.0, guard_pos.1 - 1), Tile::Up),
            Tile::Right => ((guard_pos.0, guard_pos.1 + 1), Tile::Down),
            _ => unreachable!(),
        };

        (guard_pos, guard_facing) = if let Some(tile) = guard_route.get(&next_pos) {
            if *tile == Tile::Empty {
                if guard_route.contains_key(&next_pos)
                    && next_pos != invalid_obstacle
                    && !visited.contains(&next_pos)
                {
                    guard_route.insert(next_pos, Tile::Object);
                    if test_loop((guard_pos, next_face), &guard_route) {
                        obstacles.insert(next_pos);
                    }
                    guard_route.insert(next_pos, Tile::Empty);
                }
                (next_pos, guard_facing)
            } else {
                (guard_pos, next_face)
            }
        } else {
            (next_pos, guard_facing)
        }
    }

    obstacles.len()
}
pub fn day_six_chapter_one() -> usize {
    let txt = fs::read_to_string("./src/day6.txt");
    let mut inputx: HashMap<(isize, isize), Tile> = HashMap::new();
    if let Ok(x) = txt {
        inputx = HashMap::from_iter(x.trim().lines().enumerate().flat_map(|(line_no, line)| {
            line.chars().enumerate().map(move |(tile_no, tile_char)| {
                let tile = match tile_char {
                    '.' => Tile::Empty,
                    '#' => Tile::Object,
                    '^' => Tile::Up,
                    '>' => Tile::Right,
                    '<' => Tile::Left,
                    'v' => Tile::Down,
                    _ => unreachable!(),
                };
                ((line_no as isize, tile_no as isize), tile)
            })
        }));
    }

    let mut guard_route = inputx;
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let (mut guard_pos, mut guard_facing) = guard_route
        .iter()
        .find_map(|(pos, tile)| match tile {
            Tile::Up | Tile::Down | Tile::Left | Tile::Right => Some((*pos, *tile)),
            _ => None,
        })
        .unwrap();
    guard_route.insert(guard_pos, Tile::Empty);

    while guard_route.contains_key(&guard_pos) {
        visited.insert(guard_pos);
        let next_pos = match guard_facing {
            Tile::Up => (guard_pos.0 - 1, guard_pos.1),
            Tile::Down => (guard_pos.0 + 1, guard_pos.1),
            Tile::Left => (guard_pos.0, guard_pos.1 - 1),
            Tile::Right => (guard_pos.0, guard_pos.1 + 1),
            _ => unreachable!(),
        };
        guard_pos = if let Some(tile) = guard_route.get(&next_pos) {
            if *tile != Tile::Empty {
                guard_facing = match guard_facing {
                    Tile::Up => Tile::Right,
                    Tile::Down => Tile::Left,
                    Tile::Left => Tile::Up,
                    Tile::Right => Tile::Down,
                    _ => unreachable!(),
                };
                guard_pos
            } else {
                next_pos
            }
        } else {
            next_pos
        }
    }

    visited.len()
}

fn test_loop(guard: ((isize, isize), Tile), guard_map: &HashMap<(isize, isize), Tile>) -> bool {
    let (mut guard_pos, mut guard_facing) = guard;
    let mut visited = HashSet::new();
    while guard_map.contains_key(&guard_pos) {
        if !visited.insert((guard_pos, guard_facing)) {
            return true;
        }
        let (next_pos, next_face) = match guard_facing {
            Tile::Up => ((guard_pos.0 - 1, guard_pos.1), Tile::Right),
            Tile::Down => ((guard_pos.0 + 1, guard_pos.1), Tile::Left),
            Tile::Left => ((guard_pos.0, guard_pos.1 - 1), Tile::Up),
            Tile::Right => ((guard_pos.0, guard_pos.1 + 1), Tile::Down),
            _ => unreachable!(),
        };

        (guard_pos, guard_facing) = if let Some(tile) = guard_map.get(&next_pos) {
            if *tile == Tile::Empty {
                (next_pos, guard_facing)
            } else {
                (guard_pos, next_face)
            }
        } else {
            (next_pos, guard_facing)
        }
    }
    false
}
pub fn day_six_chapter_onex() -> usize {
    let txt = fs::read_to_string("./src/day6.txt");
    let mut txt_str: Vec<String> = Vec::new();
    if let Ok(x) = txt {
        txt_str = x.split_ascii_whitespace().map(|f| f.to_string()).collect();
    }

    let rows = txt_str[0].len();
    let columns = txt_str.len();
    let mut matrix: Vec<Vec<char>> = vec![vec!['.'; columns]; rows];

    let mut pointx: Point = Point { x: 0, y: 0 };

    for (ids, s) in txt_str.iter().enumerate() {
        let char_vec: Vec<char> = s.chars().collect();
        for (idc, c) in char_vec.iter().enumerate() {
            matrix[ids][idc] = *c;
            if *c == '^' {
                pointx.y = ids;
                pointx.x = idc;
            }
        }
    }

    matrix[pointx.y][pointx.x] = '.';

    let mut direct = Direction::Atas;
    let mut seen: HashMap<Point, Point> = HashMap::new();
    seen.insert(pointx, pointx);

    let mut status = true;
    while status {
        status = walkx(&mut pointx, &matrix, &mut direct, &mut seen);
        if !status {
            if pointx.y < matrix.len() && pointx.x < matrix[0].len() {
                seen.insert(pointx, pointx);
            }
        }
    }
    for (_, point) in &seen {
        matrix[point.y][point.x] = 'X'
    }

    println!("Pointx: {:?}", pointx);

    seen.len()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}
fn walkx(
    curr: &mut Point,
    map: &Vec<Vec<char>>,
    direction: &mut Direction,
    seen: &mut HashMap<Point, Point>,
) -> bool {
    if curr.y >= map.len() || curr.x >= map[0].len() {
        return false;
    }

    match direction {
        Direction::Atas => {
            if curr.y > 0 && map[curr.y - 1][curr.x] == '#' {
                *direction = direction.rotate();
                return true;
            }
            if curr.y > 0 && map[curr.y - 1][curr.x] == '.' {
                curr.y -= 1;
                seen.insert(*curr, *curr);
                return true;
            }
        }
        Direction::Kanan => {
            if curr.x + 1 < map[0].len() && map[curr.y][curr.x + 1] == '#' {
                *direction = direction.rotate();
                return true;
            }
            if curr.x + 1 < map[0].len() && map[curr.y][curr.x + 1] == '.' {
                curr.x += 1;
                seen.insert(*curr, *curr);
                return true;
            }
        }
        Direction::Bawah => {
            if curr.y + 1 < map.len() && map[curr.y + 1][curr.x] == '#' {
                *direction = direction.rotate();
                return true;
            }
            if curr.y + 1 < map.len() && map[curr.y + 1][curr.x] == '.' {
                curr.y += 1;
                seen.insert(*curr, *curr);
                return true;
            }
        }
        Direction::Kiri => {
            if curr.x > 0 && map[curr.y][curr.x - 1] == '#' {
                *direction = direction.rotate();
                return true;
            }
            if curr.x > 0 && map[curr.y][curr.x - 1] == '.' {
                curr.x -= 1;
                seen.insert(*curr, *curr);
                return true;
            }
        }
        _ => {
            println!("{:?}", direction);
            *direction = direction.rotate();
            return true;
        }
    }
    false
}
#[derive(Debug)]
enum Direction {
    Atas,
    Kanan,
    Bawah,
    Kiri,
}

impl Direction {
    fn rotate(&mut self) -> Direction {
        match self {
            Direction::Atas => return Direction::Kanan,
            Direction::Kanan => return Direction::Bawah,
            Direction::Bawah => return Direction::Kiri,
            Direction::Kiri => return Direction::Atas,
        };
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Tile {
    Empty,
    Object,
    Up,
    Down,
    Left,
    Right,
}
