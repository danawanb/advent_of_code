use std::fs;

pub fn day_five_chapter_one() -> i32 {
    //let txt = fs::read_to_string("./src/day5_dummy.txt");
    let txt = fs::read_to_string("./src/day5.txt");

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

    //println!("{:?}", page_ordering);
    println!("-----------------------------");
    //println!("{:?}", page_update);

    let mut valid_page: Vec<Vec<i32>> = Vec::new();
    let mut in_valid_page: Vec<Vec<i32>> = Vec::new();
    for page in page_update {
        let lengx = page.len();
        let mut countx: i32 = 0;

        for (idxp, p) in page.iter().enumerate() {
            //indexks pertama
            if idxp == 0 {
                //println!("indeks pertama {} {}", page[idxp], page[idxp + 1]);
                let item: Vec<i32> = [page[idxp], page[idxp + 1]].to_vec();
                for i in &page_ordering {
                    if item == *i {
                        //println!("sama {:?}", item);
                        countx += 1;
                    }
                }

            //indeks terakhir
            } else if page.len() - 1 == idxp {
                //println!("indeks terakhir {} {}", page[idxp - 1], page[idxp]);
                let item: Vec<i32> = [page[idxp - 1], page[idxp]].to_vec();
                for i in &page_ordering {
                    if item == *i {
                        countx += 1;
                    }
                }
            } else {
                //println!("indeks setelah pertama {} {}", page[idxp], page[idxp + 1]);
                let item: Vec<i32> = [page[idxp], page[idxp + 1]].to_vec();
                for i in &page_ordering {
                    if item == *i {
                        //println!("sama {:?}", item);
                        countx += 1;
                    }
                }
            }
        }

        if lengx == countx as usize {
            //println!("{:?} is valid", page);
            valid_page.push(page);
        } else {
            in_valid_page.push(page);
        }
    }

    let mut sumx: i32 = 0;
    for v in valid_page {
        //println!("{:?}", v[v.len() / 2]);
        sumx += v[v.len() / 2];
    }

    //chapter 1
    //sumx

    let mut fixx_order: Vec<Vec<i32>> = Vec::new();

    for invalid in &in_valid_page {
        let res = validate_and_fix_order(&page_ordering, &invalid);
        fixx_order.push(res);
    }

    println!("{:?}", fixx_order);

    let mut sumxx: i32 = 0;
    for v in fixx_order {
        println!("{:?}", v[v.len() / 2]);
        sumxx += v[v.len() / 2];
    }

    //chapter 2
    sumxx
}

fn validate_and_fix_order(page_ordering: &Vec<Vec<i32>>, page_update: &Vec<i32>) -> Vec<i32> {
    let mut fixed_order = page_update.clone();
    let mut changed = true;

    while changed {
        changed = false;
        for idx in 0..fixed_order.len() - 1 {
            let pair = vec![fixed_order[idx], fixed_order[idx + 1]];
            if !page_ordering.contains(&pair) {
                let reversed_pair = vec![fixed_order[idx + 1], fixed_order[idx]];
                if page_ordering.contains(&reversed_pair) {
                    fixed_order.swap(idx, idx + 1);
                    changed = true;
                }
            }
        }
    }
    fixed_order
}
