use std::fs;

pub fn day_one() -> i32 {
    let txt = fs::read_to_string("./src/day1.txt");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    if let Ok(val) = txt {
        for (_, s_val) in val.lines().enumerate() {
            let str_val: Vec<String> = s_val.split_whitespace().map(|s| s.to_string()).collect();
            let numx: Vec<i32> = str_val.iter().map(|x| x.parse().unwrap()).collect();
            left.push(numx[0]);
            right.push(numx[1]);
        }
    }

    left.sort();
    right.sort();

    println!("{:?}", left);
    println!("{:?}", right);

    let mut res: i32 = 0;
    for i in 0..left.len() {
        let val_abs = left[i] - right[i];
        println!("{} {}", i, val_abs.abs());
        res += val_abs.abs();
    }
    res
}
