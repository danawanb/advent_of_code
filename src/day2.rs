use std::{collections::HashMap, fs};

pub fn day_two() -> i32 {
    let txt = fs::read_to_string("./src/day2.txt");
    //let txt = fs::read_to_string("./src/day2_dummy.txt");

    let mut input: Vec<Vec<i32>> = Vec::new();

    if let Ok(val) = txt {
        for (_, s_val) in val.lines().enumerate() {
            let valx: Vec<i32> = s_val
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            input.push(valx);
        }
    }

    //let resx = count_zero(safety_count);
    //println!("{}", resx);

    let mut count = 0;
    for i in input {
        let mut res_map = checksafety(&i);

        if res_map == false {
            let mut child_safe;

            for x in 0..i.len() {
                let mut temp = i.clone();
                temp.remove(x);
                //println!("{:?}", temp); // Cetak hasil

                child_safe = checksafety(&temp);
                if child_safe == true {
                    res_map = true;
                    break;
                }
                //println!("{:?} {}", temp, res_map); // Cetak hasil
            }
        }
        println!("{:?} {}", i, res_map);
        if res_map == true {
            count += 1
        }
    }

    println!("{}", count);
    return count;
}

fn checksafety(input: &Vec<i32>) -> bool {
    let mut unsafety: bool = true;

    let mut has_increase = false;
    let mut has_decrease = false;

    for x in input.windows(2) {
        let count: i32 = x[1] - x[0];

        //if increase or decrease more than 3 || neither an increase or decrease
        if count.abs() > 3 || count.abs() == 0 {
            unsafety = false;
            break;
        }

        //increase
        if count > 0 {
            has_increase = true
        } else if count < 0 {
            has_decrease = true
        }

        if has_increase && has_decrease {
            unsafety = false;
            break;
        }
    }

    return unsafety;
}

fn day_two_part1() -> i32 {
    let txt = fs::read_to_string("./src/day2.txt");
    //let txt = fs::read_to_string("./src/day2_dummy.txt");

    let mut input: Vec<Vec<i32>> = Vec::new();

    if let Ok(val) = txt {
        for (_, s_val) in val.lines().enumerate() {
            let valx: Vec<i32> = s_val
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            input.push(valx);
        }
    }
    let mut safex: HashMap<Vec<i32>, i32> = HashMap::new();
    let mut safety_count: Vec<i32> = Vec::new();
    for i in input {
        //println!("{:?}", i);
        let mut has_increase = false;
        let mut has_decrease = false;

        let mut safety: i32 = 0;
        for x in 0..i.len() - 1 {
            let count: i32 = i[x] - i[x + 1];

            //if increase or decrease more than 3
            if count.abs() > 3 {
                safety += 1;
                break;
            }

            //if neither an increase of decrease
            if count.abs() == 0 {
                safety += 1;
                break;
            }

            //increase
            if i[x] < i[x + 1] {
                has_increase = true
            } else if i[x] > i[x + 1] {
                has_decrease = true
            }

            if has_increase && has_decrease {
                safety += 1;
                break;
            }

            //if unsafex == true {
            //   safety += 1;
            //   break;
            //}
        }
        safety_count.push(safety.clone());
        safex.insert(i.clone(), safety.clone());
        //println!("{}", safety);
    }

    for safe in safex {
        println!("{:?}", safe);
    }

    let resx = count_zero(safety_count);
    println!("{}", resx);
    return resx;
}

fn count_zero(target: Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    for i in target {
        if i == 0 {
            res += 1
        }
    }
    res
}
