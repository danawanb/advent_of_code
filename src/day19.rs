use std::{collections::HashMap, fs};

pub fn day_nineteen() -> usize {
    let txt = fs::read_to_string("./src/day19.txt");
    let mut txt_str: Vec<String> = Vec::new();
    if let Ok(x) = txt {
        txt_str = x.split("\n\n").map(|f| f.to_string()).collect();
    }

    let mut towel: Vec<String> = Vec::new();
    if let Some(xx) = txt_str.get(0) {
        towel = xx
            .split(",")
            .map(|f| f.trim().to_string()) // Hilangkan whitespace di awal dan akhir string
            .filter(|f| !f.is_empty())
            .collect();
    }

    //towel to display
    let mut onsen_display: Vec<String> = Vec::new();
    if let Some(xx) = txt_str.get(1) {
        onsen_display = xx.lines().map(|x| x.to_string()).collect();
    }

    let mut countx = 0;

    //for design in &onsen_display {
    //if can_construct_design(&towel, design) {
    //countx += 1;
    //}
    //}

    let mut countx_part2 = 0;

    for design in &onsen_display {
        let cw = cw_to_construct(&towel, design);
        countx_part2 += cw;
    }

    countx_part2
}

fn can_construct_design(patterns: &Vec<String>, design: &str) -> bool {
    let mut memo: HashMap<String, bool> = HashMap::new();
    return can_construct(design.to_string(), patterns, &mut memo);
}

fn can_construct(target: String, patterns: &Vec<String>, memo: &mut HashMap<String, bool>) -> bool {
    //cek key jika ada ambil valuenya
    if memo.contains_key(&target) {
        return *memo.get(&target).unwrap();
    }

    //cek apakah sudah targetnya ada
    if target.is_empty() {
        return true;
    }

    //looping patternnya
    for pattern in patterns {
        if target.starts_with(pattern) {
            let suffix = target[pattern.len()..].to_string();
            if can_construct(suffix, patterns, memo) {
                memo.insert(target.clone(), true);
                return true;
            }
        }
    }

    //insert ke memo sementara
    memo.insert(target.clone(), false);
    return false;
}

fn cw_to_construct(patterns: &Vec<String>, design: &str) -> usize {
    let mut memo: HashMap<String, usize> = HashMap::new();
    return cw(design.to_string(), patterns, &mut memo);
}

fn cw(target: String, patterns: &Vec<String>, memo: &mut HashMap<String, usize>) -> usize {
    if memo.contains_key(&target) {
        return *memo.get(&target).unwrap();
    }

    if target.is_empty() {
        return 1;
    }

    let mut total_ways = 0;

    for pattern in patterns {
        if target.starts_with(pattern) {
            let suffix = target[pattern.len()..].to_string();
            total_ways += cw(suffix, patterns, memo);
        }
    }

    memo.insert(target.clone(), total_ways);
    return total_ways;
}
