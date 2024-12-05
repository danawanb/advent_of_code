use std::fs;

pub fn day_five_chapter_one() -> i32 {
    let txt = fs::read_to_string("./src/day5_dummy.txt");

    let mut valx: Vec<String> = Vec::new();

    if let Ok(val) = txt {
        valx = val.split_whitespace().map(|f| f.to_string()).collect();
    }

    let mut page_ordering: Vec<Vec<i32>> = Vec::new();
    let mut page_update: Vec<Vec<i32>> = Vec::new();

    for val in valx {
        if val.contains("|") {
            let num: Vec<i32> = val.split("|").map(|x| x.parse().unwrap()).collect();
            page_ordering.push(num);
            //println!("{}", val);
        } else {
            let num: Vec<i32> = val.split(",").map(|x| x.parse().unwrap()).collect();
            page_update.push(num);
        }
    }

    println!("{:?}", page_ordering);
    println!("-----------------------------");
    println!("{:?}", page_update);
    32
}
