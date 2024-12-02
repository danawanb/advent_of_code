use std::fs;

pub fn day_one() -> i32 {
    let txt = fs::read_to_string("./src/day1.txt");
    //let txt = fs::read_to_string("./src/day1_dummy.txt");
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

    let mut sleft = left.clone();
    let mut sright = right.clone();
    sleft.sort();
    sright.sort();

    //println!("{:?}", sleft);
    //println!("{:?}", sright);

    let mut res: i32 = 0;
    for i in 0..sleft.len() {
        let val_abs = sleft[i] - sright[i];
        //println!("{} {}", i, val_abs.abs());
        res += val_abs.abs();
    }

    let vecx: Vec<i32> = [4, 3, 5, 3, 9, 3].to_vec();
    let res2 = search_appears(4, &vecx);
    println!("{}", res2);

    let mut total: i32 = 0;
    for i in 0..left.len() {
        let tot = search_appears(left[i], &right);
        //println!("angka {} {}", i, left[i] * tot);
        total += left[i] * tot;
    }

    println!("total chapter 1 {}", res);
    println!("total chapter 2 {}", total);
    res
}

fn search_appears(x: i32, search_vec: &Vec<i32>) -> i32 {
    let mut res: i32 = 0;

    for i in search_vec {
        //println!("{}", i);
        if *i == x {
            res += 1;
        }
    }

    res
}
