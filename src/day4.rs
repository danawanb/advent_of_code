use std::{fmt::format, fs, isize, usize};

fn count_substring_occurrences(haystack: &str, needle: &str) -> usize {
    haystack.match_indices(needle).count()
}

pub fn day_four_chapter_two() -> i32 {
    let txt = fs::read_to_string("./src/day4.txt");

    let mut txt_str: Vec<String> = Vec::new();
    if let Ok(val) = txt {
        for (_, s) in val.lines().enumerate() {
            txt_str.push(s.to_string());
        }
    }

    let columns = txt_str.len();
    let rows = txt_str[0].len();

    println!("columns {columns} rows {rows}");

    let mut matrix: Vec<Vec<String>> = vec![vec![String::from("-"); columns]; rows];

    for (ids, s) in txt_str.iter().enumerate() {
        let char_vec: Vec<char> = s.chars().collect();
        for (idc, c) in char_vec.iter().enumerate() {
            //println!("{} {} {}", ids, idc, c);
            matrix[ids][idc] = c.to_string();
        }
    }

    let mut xmas_the_spot: Vec<Xmas> = Vec::new();

    for (i, vali) in matrix.iter().enumerate() {
        //println!("{:?}", i);
        for (j, valj) in vali.iter().enumerate() {
            //println!("{} {}", i, j);
            if i > 0 && j > 0 {
                //indeks terakhir tidak dianggap baik col terakhir atau row terakhir
                if j + 1 == vali.len() || i + 1 == vali.len() {
                    break;
                }

                //tentukan indeks maksimal agar tidak di luar bidang
                let maks = j + 1;
                //if j + 1 > vali.len() - 1 {
                //   maks = vali.len() - 1
                //}

                if matrix[i][j] == "A" {
                    let kiri_atas = matrix[i - 1][j - 1].clone();
                    let kanan_atas = matrix[i - 1][j + 1].clone();
                    let kiri_bawah = matrix[i + 1][j - 1].clone();
                    let kanan_bawah = matrix[i + 1][j + 1].clone();

                    if kiri_atas == "M" || kiri_atas == "S" {
                        //println!("{} {} kiri atas {:?} tengah{:?} kanan atas{:?} kiri bawah {:?} kanan bawah {:?}",
                        //i,
                        //j,
                        //kiri_atas,
                        //matrix[i][j],
                        //kanan_atas,
                        //matrix[i + 1][j - 1],
                        //matrix[i + 1][j + 1]
                        //);
                        let single_mat = Xmas {
                            kiri_atas: kiri_atas.clone(),
                            kanan_atas: kanan_atas.clone(),
                            kiri_bawah: kiri_bawah.clone(),
                            kanan_bawah: kanan_bawah.clone(),
                            point: format!("{:?} {i}{j}", matrix[i][j]),
                            store: vec![
                                kiri_atas.clone(),
                                kanan_atas.clone(),
                                kiri_bawah.clone(),
                                kanan_bawah.clone(),
                            ]
                            .to_vec(),
                        };
                        xmas_the_spot.push(single_mat);
                    }
                }
            }
        }
    }

    //println!("{:?}", xmas_the_spot);

    let mut countx = 0;
    for xmas in xmas_the_spot {
        if xmas.valid() {
            countx += 1;
            //println!("xmas {:?} point {:?}", xmas.store, xmas.point);
        }
    }
    countx
}

#[derive(Debug)]
struct Xmas {
    kiri_atas: String,
    kanan_atas: String,
    kiri_bawah: String,
    kanan_bawah: String,
    point: String,
    store: Vec<String>,
}

impl Xmas {
    fn valid(&self) -> bool {
        let mut s_count = 0;

        if self.kiri_atas == self.kanan_bawah {
            return false;
        }

        for r in &self.store {
            if r == "X" || r == "A" {
                return false;
            }

            if r == "S" {
                s_count += 1;
            }
        }

        if s_count == 2 {
            return true;
        } else {
            return false;
        };
    }
}
pub fn day_four_chapter_one() -> i32 {
    //dummy benar real salah

    //let txt = fs::read_to_string("./src/day4_dummy.txt");
    let txt = fs::read_to_string("./src/day4.txt");

    let mut txt_str: Vec<String> = Vec::new();
    if let Ok(val) = txt {
        for (_, s) in val.lines().enumerate() {
            //println!("{} {:?}", val.len(), s);
            txt_str.push(s.to_string());
        }
    }

    let columns = txt_str.len();
    let rows = txt_str[0].len();

    println!("columns {columns} rows {rows}");

    let mut matrix: Vec<Vec<String>> = vec![vec![String::from("-"); columns]; rows];

    for (ids, s) in txt_str.iter().enumerate() {
        let char_vec: Vec<char> = s.chars().collect();
        for (idc, c) in char_vec.iter().enumerate() {
            //println!("{} {} {}", ids, idc, c);
            matrix[ids][idc] = c.to_string();
        }
    }

    let mut count: i32 = 0;

    //hitung horizontal
    for i in &matrix {
        let ccount = count_substring_occurrences(&i.concat(), "XMAS");
        count += ccount as i32;

        let ccount = count_substring_occurrences(&i.concat(), "SAMX");
        count += ccount as i32;
    }
    //println!("horizontal {}", count);

    let iter = (0..rows).map(|row_idx| matrix.iter().flatten().skip(row_idx).step_by(columns));

    let mut matrixx: Vec<Vec<String>> = vec![vec![String::from("-"); columns]; rows];
    for (row_idx, row_values) in iter.enumerate() {
        for (column_idx, value) in row_values.enumerate() {
            matrixx[row_idx][column_idx] = value.to_string();
            //println!("[{}, {}] = {}", row_idx, column_idx, value);
        }
    }

    //hitung vertical
    let mut vert: i32 = 0;
    for i in &matrixx {
        let ccount = count_substring_occurrences(&i.concat(), "XMAS");
        count += ccount as i32;
        vert += ccount as i32;

        let ccount = count_substring_occurrences(&i.concat(), "SAMX");
        count += ccount as i32;
        vert += ccount as i32;
    }

    let mut diago: Vec<Vec<String>> = Vec::new();

    //tranpose rdiago
    for i in (1 - columns as isize)..rows as isize {
        let mut group: Vec<String> = Vec::new();

        for j in 0..columns {
            let index = i + j as isize;
            if index >= 0 && index < rows as isize {
                group.push(matrix[j][index as usize].clone());
            }
        }
        diago.push(group);
    }

    //hitung rdiagonal
    let mut rdiago: i32 = 0;

    for i in &diago {
        //println!("{:?}", i);

        let ccount = count_substring_occurrences(&i.concat(), "XMAS");
        count += ccount as i32;
        rdiago += ccount as i32;

        let ccount = count_substring_occurrences(&i.concat(), "SAMX");
        count += ccount as i32;
        rdiago += ccount as i32;
    }

    let mut leftdiago: Vec<Vec<String>> = Vec::new();

    //tranpose ldiago
    for i in 0..(rows + columns - 1) as isize {
        let mut group: Vec<String> = Vec::new();

        for j in 0..rows {
            let index = i - j as isize;
            if index >= 0 && index < rows as isize {
                group.push(matrix[j][index as usize].clone());
            }
        }
        leftdiago.push(group);
    }

    //println!("{:?}", diago);
    //println!("{:?}", leftdiago);

    //hitung ldiagonal
    let mut ldiago: i32 = 0;

    for i in &leftdiago {
        let ccount = count_substring_occurrences(&i.concat(), "XMAS");
        count += ccount as i32;
        ldiago += ccount as i32;

        let ccount = count_substring_occurrences(&i.concat(), "SAMX");
        count += ccount as i32;
        ldiago += ccount as i32;
    }

    println!("vertical {vert}");
    println!("rdiago {rdiago}");
    println!("ldiago {ldiago}");
    //println!("{:?}", matrixx);
    println!("total  : {count}");

    count
}

pub fn count_xmas() -> i32 {
    use std::fs;

    let txt = fs::read_to_string("./src/day4.txt").expect("File not found");
    let txt_str: Vec<String> = txt.lines().map(|line| line.to_string()).collect();

    //buat matrix dari txt str
    let matrix: Vec<Vec<char>> = txt_str.iter().map(|line| line.chars().collect()).collect();

    let mut count = 0;

    //hitung kata
    fn count_word(matrix: &[Vec<char>], word: &str, dx: isize, dy: isize) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let word_len = word.len();
        let word_chars: Vec<char> = word.chars().collect();
        let mut count = 0;

        for i in 0..rows {
            for j in 0..cols {
                let mut match_found = true;

                for k in 0..word_len {
                    let x = i as isize + k as isize * dx;
                    let y = j as isize + k as isize * dy;

                    if x < 0 || x >= rows as isize || y < 0 || y >= cols as isize {
                        match_found = false;
                        break;
                    }

                    if matrix[x as usize][y as usize] != word_chars[k] {
                        match_found = false;
                        break;
                    }
                }

                if match_found {
                    count += 1;
                }
            }
        }

        count
    }

    let directions = vec![
        (0, 1),  // Horizontal right
        (1, 0),  // Vertical down
        (1, 1),  // Diagonal down-right
        (1, -1), // Diagonal down-left
    ];

    for (dx, dy) in &directions {
        count += count_word(&matrix, "XMAS", *dx, *dy);
        count += count_word(&matrix, "SAMX", *dx, *dy); // Backward
    }

    count
}
