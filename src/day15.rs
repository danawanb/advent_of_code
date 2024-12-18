use std::fs;

pub fn day_fifteen() -> usize {
    //let txt = fs::read_to_string("./src/day15_dummy.txt");
    let txt = fs::read_to_string("./src/day15.txt");
    let mut txt_str: Vec<String> = Vec::new();
    if let Ok(x) = txt {
        txt_str = x.split("\n\n").map(|x| x.to_string()).collect();
    }

    //mapnya
    let mut mapx: Vec<String> = Vec::new();
    if let Some(x) = txt_str.get(0) {
        mapx = x
            .split_ascii_whitespace()
            .map(|xx| xx.to_string())
            .collect();
    }

    //movenya
    let mut movex: Vec<char> = Vec::new();
    if let Some(x) = txt_str.get(1) {
        movex = x.chars().map(|a| a).collect();
    }

    let columns = mapx[0].len();
    let rows = mapx.len();
    let mut matrix: Vec<Vec<char>> = vec![vec!['-'; columns]; rows];

    let mut cur_pointx: Position = Position { x: 0, y: 0 };
    for (idy, valy) in mapx.iter().enumerate() {
        for (idx, valx) in valy.chars().into_iter().enumerate() {
            matrix[idy][idx] = valx;
            if valx == '@' {
                cur_pointx.x = idx;
                cur_pointx.y = idy;
            }
        }
    }

    for re in movex {
        move_position(&mut matrix, re, &mut cur_pointx);
    }

    let mut sumx = 0;
    for (idy, mat) in matrix.iter().enumerate() {
        //println!("{:?}", mat);
        for (idx, matx) in mat.iter().enumerate() {
            //println!("{} {} {}", idy, idx, matx);
            if *matx == 'O' {
                sumx += 100 * idy + idx;
            }
        }
    }
    sumx
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

fn move_position(matrix: &mut Vec<Vec<char>>, movex: char, postx: &mut Position) -> Vec<Vec<char>> {
    match movex {
        '>' => {
            let other = matrix[postx.y][postx.x + 1];

            //jika di depan adalah #
            if is_fence(other) {
                return matrix.to_vec();
            }

            //jika di depan adalah .
            if is_dot(other) {
                matrix[postx.y].swap(postx.x, postx.x + 1);
                postx.x += 1;
                return matrix.to_vec();
            };

            //jika di depan adalah O maka looping untuk mendapatkan valuenya di depan berapa kali
            let res = is_oo(matrix, *postx, '>');

            //jika di depan adalah 0 dan lebih dari 0
            if res > 0 {
                for i in (1..=res + 1).rev() {
                    //swap @ dengan .
                    if i <= 1 {
                        matrix[postx.y][postx.x] = '.';
                        matrix[postx.y][postx.x + 1] = '@';
                        postx.x += 1;
                    // swap urutan O
                    } else {
                        matrix[postx.y].swap(postx.x + i - 1, postx.x + i);
                    }
                }
                return matrix.to_vec();
            } else if res == 0 {
                return matrix.to_vec();
            }
        }

        '<' => {
            let other = matrix[postx.y][postx.x - 1];

            //jika di depan adalah #
            if is_fence(other) {
                return matrix.to_vec();
            }

            //jika di depan adalah .
            if is_dot(other) {
                matrix[postx.y].swap(postx.x, postx.x - 1);
                postx.x -= 1;
                return matrix.to_vec();
            };

            //jika di depan adalah O maka looping untuk mendapatkan valuenya di depan berapa kali
            let res = is_oo(matrix, *postx, '<');

            //jika di depan adalah 0 dan lebih dari 0
            if res > 0 {
                for i in (1..=res + 1).rev() {
                    //swap @ dengan .
                    if i <= 1 {
                        matrix[postx.y][postx.x] = '.';
                        matrix[postx.y][postx.x - 1] = '@';
                        postx.x -= 1;
                        // swap urutan O
                    } else {
                        matrix[postx.y].swap(postx.x - i, postx.x - i + 1);
                    }
                }
                return matrix.to_vec();
            } else if res == 0 {
                return matrix.to_vec();
            }
        }

        //atas
        '^' => {
            let other = matrix[postx.y - 1][postx.x];
            //jika di depan adalah #
            if is_fence(other) {
                return matrix.to_vec();
            }

            //jika di depan adalah .
            if is_dot(other) {
                matrix[postx.y - 1][postx.x] = '@';
                matrix[postx.y][postx.x] = '.';
                postx.y -= 1;
                return matrix.to_vec();
            };

            let res = is_oo(matrix, *postx, '^');
            if res > 0 {
                for i in (1..=res + 1).rev() {
                    //swap @ dengan .

                    if i <= 1 {
                        matrix[postx.y - 1][postx.x] = '@';
                        matrix[postx.y][postx.x] = '.';
                        postx.y -= 1;
                        // swap urutan O
                    } else {
                        matrix[postx.y - i][postx.x] = 'O';
                    }
                }
                return matrix.to_vec();
            } else if res == 0 {
                return matrix.to_vec();
            }
        }

        //bawah
        'v' => {
            let other = matrix[postx.y + 1][postx.x];
            //jika di depan adalah #
            if is_fence(other) {
                return matrix.to_vec();
            }

            //jika di depan adalah .
            if is_dot(other) {
                matrix[postx.y + 1][postx.x] = '@';
                matrix[postx.y][postx.x] = '.';
                postx.y += 1;
                return matrix.to_vec();
            };

            let res = is_oo(matrix, *postx, 'v');
            if res > 0 {
                for i in (1..=res + 1).rev() {
                    //swap @ dengan .
                    if i <= 1 {
                        matrix[postx.y + 1][postx.x] = '@';
                        matrix[postx.y][postx.x] = '.';
                        postx.y += 1;
                        // swap urutan O
                    } else {
                        matrix[postx.y + i][postx.x] = 'O';
                        //println!("{}", matrix[postx.y + i][postx.x]);
                        //postx.y += 1;
                    }
                }
                return matrix.to_vec();
            } else if res == 0 {
                return matrix.to_vec();
            }
        }
        _ => println!(),
    }

    return matrix.to_vec();
}

fn is_fence(next_to: char) -> bool {
    next_to == '#'
}
fn is_dot(next_to: char) -> bool {
    next_to == '.'
}

fn is_oo(matrix: &mut Vec<Vec<char>>, postx: Position, direct: char) -> usize {
    let mut num = 0;
    let postx = postx.clone();
    match direct {
        '>' => {
            while is_oo_check(matrix[postx.y][postx.x + 1 + num]) {
                num += 1;
            }

            //jika paling ujungnya ada #nya maka kembalikan 0
            if is_fence(matrix[postx.y][postx.x + 1 + num]) {
                return 0;
            }

            return num;
        }
        '<' => {
            while is_oo_check(matrix[postx.y][postx.x - 1 - num]) {
                num += 1;
            }

            //jika paling ujungnya ada #nya maka kembalikan 0
            if is_fence(matrix[postx.y][postx.x - 1 - num]) {
                return 0;
            }

            return num;
        }
        '^' => {
            while is_oo_check(matrix[postx.y - 1 - num][postx.x]) {
                num += 1;
            }

            //jika paling ujungnya ada #nya maka kembalikan 0
            if is_fence(matrix[postx.y - 1 - num][postx.x]) {
                return 0;
            }

            return num;
        }
        'v' => {
            while is_oo_check(matrix[postx.y + 1 + num][postx.x]) {
                num += 1;
            }

            //jika paling ujungnya ada #nya maka kembalikan 0
            if is_fence(matrix[postx.y + 1 + num][postx.x]) {
                return 0;
            }

            return num;
        }
        //v' => println!("bawah"),
        _ => {
            return num;
        }
    }
}

fn is_oo_check(next_to: char) -> bool {
    next_to == 'O'
}
