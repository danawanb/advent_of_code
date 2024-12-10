use std::fs;

pub fn day_eight() -> i32 {
    let txt = fs::read_to_string("./src/day8.txt");

    let mut txt_str: Vec<String> = Vec::new();
    if let Ok(val) = txt {
        for (_, s) in val.lines().enumerate() {
            txt_str.push(s.to_string());
        }
    }

    let columns = txt_str.len();
    let rows = txt_str[0].len();

    //println!("columns {columns} rows {rows}");

    let mut matrix: Vec<Vec<char>> = vec![vec!['.'; columns]; rows];

    for (ids, s) in txt_str.iter().enumerate() {
        let char_vec: Vec<char> = s.chars().collect();
        for (idc, c) in char_vec.iter().enumerate() {
            matrix[ids][idc] = *c;
        }
    }

    let mut antena: Vec<Antena> = Vec::new();
    for (i, vali) in matrix.iter().enumerate() {
        for (idx, valx) in vali.iter().enumerate() {
            if *valx != '.' {
                let single_val = Antena {
                    x: i as i32,
                    y: idx as i32,
                    value: *valx,
                };

                antena.push(single_val);
            }
        }
    }

    println!("{:?}", antena);

    let txt = fs::read_to_string("./src/day8.txt");
    let input = txt.unwrap();
    let mut catchers = Vec::new();

    for (i, pos) in antena.iter().enumerate() {
        for (j, pos2) in antena.iter().enumerate() {
            //jika value characternya sama
            if pos.value == pos2.value && i != j {
                let pos_i_diff = pos.x as isize - pos2.x as isize;
                let pos_j_diff = pos.y as isize - pos2.y as isize;

                if pos_i_diff + pos.x as isize >= 0
                    && pos_j_diff + pos.y as isize >= 0
                    && pos_i_diff + (pos.x as isize) < input.lines().count() as isize
                    && pos_j_diff + (pos.y as isize)
                        < input.lines().nth(0).unwrap().chars().count() as isize
                    && !catchers
                        .contains(&(pos_i_diff + pos.x as isize, pos_j_diff + pos.y as isize))
                {
                    catchers.push((pos_i_diff + pos.x as isize, pos_j_diff + pos.y as isize));
                }

                let pos_i_diff2 = pos2.x as isize - pos.x as isize;
                let pos_j_diff2 = pos2.y as isize - pos.y as isize;

                if pos_i_diff2 + pos2.x as isize >= 0
                    && pos_j_diff2 + pos2.y as isize >= 0
                    && pos_i_diff2 + (pos2.x as isize) < input.lines().count() as isize
                    && pos_j_diff2 + (pos2.y as isize)
                        < input.lines().nth(0).unwrap().chars().count() as isize
                    && !catchers
                        .contains(&(pos_i_diff2 + pos2.x as isize, pos_j_diff2 + pos2.y as isize))
                {
                    catchers.push((pos_i_diff2 + pos2.x as isize, pos_j_diff2 + pos2.y as isize));
                }
            }
        }
    }

    println!("{}", catchers.len());

    catchers.len() as i32
}

#[derive(Debug)]
struct Antena {
    x: i32,
    y: i32,
    value: char,
}
