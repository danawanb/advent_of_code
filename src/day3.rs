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
        }
    }

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
            println!("{:?} {} {}", part, index, num);
            if index == 0 {
                single_sum.x = numy
            } else {
                single_sum.y = numy
            }
        }
        sum_col.push(single_sum);
    }

    println!("{:?}", sum_col);

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
