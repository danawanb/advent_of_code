use std::{collections::HashSet, fs};

pub fn day_ten() -> usize {
    //let txt = fs::read_to_string("./src/day10_dummy.txt");
    let txt = fs::read_to_string("./src/day10.txt");

    let mut txt_i: Vec<String> = Vec::new();

    if let Ok(val) = txt {
        txt_i = val
            .split_whitespace()
            .map(|f| f.to_string())
            .into_iter()
            .collect();
    }

    let columns = txt_i.len();
    let rows = txt_i.len();

    let mut matrix: Vec<Vec<i32>> = vec![vec![0; columns]; rows];

    for (ids, s) in txt_i.iter().enumerate() {
        let char_vec: Vec<char> = s.chars().collect();
        for (idc, c) in char_vec.iter().enumerate() {
            matrix[ids][idc] = (*c as u8 - '0' as u8) as i32;
        }
    }

    let mut total = 0;
    for (id, i) in matrix.iter().enumerate() {
        for (idj, _) in i.iter().enumerate() {
            if matrix[id][idj] == 0 {
                let mut end_positions = HashSet::new();
                search_trail(idj, id, &matrix, &mut end_positions);
                total += end_positions.len();
            }
        }
    }

    println!("{}", total);

    let mut total2 = 0;

    for (id, i) in matrix.iter().enumerate() {
        for (idj, _) in i.iter().enumerate() {
            if matrix[id][idj] == 0 {
                total2 += search_trail_2(idj, id, &matrix);
            }
        }
    }

    println!("{}", total2);

    6
}

fn search_trail_2(x: usize, y: usize, matrix: &Vec<Vec<i32>>) -> usize {
    let height = matrix[y][x];
    if height == 9 {
        return 1;
    }
    let mut score = 0;

    //kiri
    if x > 0 && matrix[y][x - 1] == height + 1 {
        score += search_trail_2(x - 1, y, matrix);
    }

    //atas
    if y > 0 && matrix[y - 1][x] == height + 1 {
        score += search_trail_2(x, y - 1, matrix)
    }

    //kanan
    if x < matrix[y].len() - 1 && matrix[y][x + 1] == height + 1 {
        score += search_trail_2(x + 1, y, matrix)
    }

    //bawah
    if y < matrix.len() - 1 && matrix[y + 1][x] == height + 1 {
        score += search_trail_2(x, y + 1, matrix)
    }
    score
}

fn search_trail(
    x: usize,
    y: usize,
    matrix: &Vec<Vec<i32>>,
    end_positions: &mut HashSet<(usize, usize)>,
) {
    let height = matrix[y][x];
    if height == 9 {
        end_positions.insert((x, y));
        return;
    }

    // kiri
    if x > 0 && matrix[y][x - 1] == height + 1 {
        search_trail(x - 1, y, matrix, end_positions);
    }

    // atas
    if y > 0 && matrix[y - 1][x] == height + 1 {
        search_trail(x, y - 1, matrix, end_positions);
    }

    // kanan
    if x < matrix[y].len() - 1 && matrix[y][x + 1] == height + 1 {
        search_trail(x + 1, y, matrix, end_positions);
    }

    // bawah
    if y < matrix.len() - 1 && matrix[y + 1][x] == height + 1 {
        search_trail(x, y + 1, matrix, end_positions);
    }
}
