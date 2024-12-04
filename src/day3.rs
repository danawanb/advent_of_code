use std::{char, fs};

pub fn day_three() -> i32 {
    //let txt = fs::read_to_string("./src/day3_dummy.txt");
    let txt = fs::read_to_string("./src/day3.txt");

    let mut colx: Vec<String> = Vec::new();
    if let Ok(val) = txt {
        let char_val: Vec<char> = val.chars().collect();

        for (index, c) in char_val.iter().enumerate() {
            //println!("{} {}", index, c);

            if c.to_owned() == 'm' {
                //println!("{:?}", &char_val[index..index + 4]);

                let char_for: String = char_val[index..index + 4].iter().collect();
                println!("{:?}", char_for);

                if char_for == "mul(".to_string() {
                    let mut akhir = index + 12;
                    if akhir > char_val.len() {
                        akhir = char_val.len();
                    }
                    let char_mul: String = char_val[index..akhir].iter().collect();
                    //println!("{:?}", char_mul);
                    colx.push(char_mul);
                }
            }
        }
    }

    //println!("{:?}", colx);

    let mut colxx: Vec<String> = Vec::new();

    for (i, j) in colx.iter().enumerate() {
        let char_vec: Vec<char> = j.chars().collect();
        for (index, c) in char_vec.iter().enumerate() {
            //
            if *c == ')' {
                println!("index {}, index childnya {}", i, index);
                let colx_insert = colx[i].clone();
                colxx.push(colx_insert[0..index + 1].to_string());
                break;
            }
        }
    }

    //println!("{:?}", colxx);

    let mut sum_col: Vec<Point> = Vec::new();
    for i in colxx {
        let parts = i.split(",");
        //let partx: Vec<&str> = parts.collect();
        //println!("{:?}", partx);

        //println!("{}", num1);
        let mut single_sum = Point { x: 0, y: 0 };
        for (index, part) in parts.clone().into_iter().enumerate() {
            let num: String = part.chars().filter(|c| c.is_numeric()).collect();
            let numy: i32 = num.parse().expect("Gagal konversi");
            //println!("{:?} {} {}", part, index, num);
            if index == 0 {
                single_sum.x = numy
            } else {
                single_sum.y = numy
            }
        }
        sum_col.push(single_sum);
    }

    //println!("{:?}", sum_col);

    let mut hitung: i32 = 0;
    for i in sum_col {
        hitung += i.count_mul()
    }

    println!("{}", hitung);

    32
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn count_mul(&self) -> i32 {
        self.x * self.y
    }
}

pub fn day_tree_part_two() -> i32 {
    //let txt = fs::read_to_string("./src/day3_dummy.txt");
    let txt = fs::read_to_string("./src/day3.txt");

    let mut colx: Vec<String> = Vec::new();
    if let Ok(val) = txt {
        let char_val: Vec<char> = val.chars().collect();

        for (index, c) in char_val.iter().enumerate() {
            //println!("{} {}", index, c);

            if c.to_owned() == 'm' {
                //println!("{:?}", &char_val[index..index + 4]);

                let char_for: String = char_val[index..index + 4].iter().collect();
                //println!("{:?}", char_for);

                if char_for == "mul(".to_string() {
                    let mut akhir = index + 12;
                    if akhir > char_val.len() {
                        akhir = char_val.len();
                    }
                    let char_mul: String = char_val[index..akhir].iter().collect();
                    //println!("{:?}", char_mul);
                    colx.push(char_mul);
                }
            }

            if c.to_owned() == 'd' {
                let char_for: String = char_val[index..index + 2].iter().collect();
                if char_for == "do".to_string() {
                    let mut akhir = index + 7;
                    if akhir > char_val.len() {
                        akhir = char_val.len();
                    }
                    let char_mul: String = char_val[index..akhir].iter().collect();
                    //println!("{:?}", char_mul);
                    colx.push(char_mul);
                }
            }
        }
    }

    //println!("{:?}", colx);

    let mut colxx: Vec<String> = Vec::new();

    for (i, j) in colx.iter().enumerate() {
        let char_vec: Vec<char> = j.chars().collect();
        for (index, c) in char_vec.iter().enumerate() {
            //
            if *c == ')' {
                //println!("index {}, index childnya {}", i, index);
                let colx_insert = colx[i].clone();
                colxx.push(colx_insert[0..index + 1].to_string());
                break;
            }
            if *c == 'd' {
                let mut akhir = index + 7;
                let colx_insert = colx[i].clone();

                if akhir > colx[i].len() {
                    akhir = colx[i].len();
                }

                //println!("{}", colx[i]);

                if colx_insert[0..4].to_string() == "do()" {
                    colxx.push(colx_insert[0..4].to_string());
                    break;
                } else {
                    colxx.push(colx_insert[0..akhir].to_string());
                    break;
                }
            }
        }
    }

    //println!("{:?}", colxx);
    let mut sum_col_before: Vec<String> = Vec::new();
    let mut current_do = true;
    for (indexs, val) in colxx.clone().iter().enumerate() {
        //println!("{:?}", i);
        if val == "do()" {
            current_do = true;
        }
        if val == "don't()" {
            current_do = false;
        }
        if current_do == true {
            if val != "do()" {
                sum_col_before.push(val.to_string())
            };
            //println!("{}", indexs);
        }
    }

    //println!("{:?}", sum_col_before);

    let mut sum_col: Vec<Point> = Vec::new();
    for i in sum_col_before {
        let parts = i.split(",");
        //let partx: Vec<&str> = parts.collect();
        //println!("{:?}", partx);

        //println!("{}", num1);
        let mut single_sum = Point { x: 0, y: 0 };
        for (index, part) in parts.clone().into_iter().enumerate() {
            let num: String = part.chars().filter(|c| c.is_numeric()).collect();
            let numy: i32 = num.parse().expect("Gagal konversi");
            //println!("{:?} {} {}", part, index, num);
            if index == 0 {
                single_sum.x = numy
            } else {
                single_sum.y = numy
            }
        }
        sum_col.push(single_sum);
    }

    //println!("{:?}", sum_col);

    let mut hitung: i32 = 0;
    for i in sum_col {
        hitung += i.count_mul()
    }

    println!("{}", hitung);

    return hitung;
}
