use cached::proc_macro::cached;
use std::fs;

pub fn day_eleven() -> usize {
    //let txt = fs::read_to_string("./src/day11_dummy.txt");
    let txt = fs::read_to_string("./src/day11.txt");

    let mut numx: Vec<usize> = Vec::new();
    if let Ok(x) = txt {
        numx = x
            .split_ascii_whitespace()
            .map(|f| f.parse().unwrap())
            .collect();
    }

    let numy = numx.clone();
    //println!("{:?}", numx);
    //let res = blinking(numx);

    let now1 = std::time::Instant::now();
    let mut res = Vec::new();
    res = blinking_1(numx);
    //println!("{:?}", res);

    for _ in 1..25 {
        res = blinking_1(res);
        //println!("{:?}", res);
    }

    println!("part 1 {} ({:?})", res.len(), now1.elapsed());

    let now = std::time::Instant::now();
    let p2 = numy.iter().map(|n| blink(*n, 75)).sum::<usize>();
    println!("part 2: {} ({:?})", p2, now.elapsed());
    66
}

fn blinking_1(numx: Vec<usize>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    for num in numx {
        let mut numbers = number_to_vec(num);
        let bagi2 = numbers.len() / 2;
        if num == 0 {
            res.push(1);
        } else if numbers.len() % 2 == 0 {
            if let Some((left, right)) = numbers.split_at_mut_checked(bagi2) {
                res.push(concat(left));
                res.push(concat(right));
            }
        } else {
            res.push(num * 2024);
        }
    }

    res
}

fn number_to_vec(n: usize) -> Vec<usize> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
}

fn concat(vec: &[usize]) -> usize {
    let mut acc = 0;
    for elem in vec {
        acc *= 10;
        acc += elem;
    }
    acc
}

#[cached]
fn blink(number: usize, remaining: usize) -> usize {
    if remaining == 0 {
        return 1;
    }

    let s = number.to_string();
    if number == 0 {
        blink(1, remaining - 1)
    } else if s.len() % 2 == 0 {
        let half = s.len() / 2;
        let left = s[..half].parse::<usize>().unwrap();
        let right = s[half..].parse::<usize>().unwrap();
        blink(left, remaining - 1) + blink(right, remaining - 1)
    } else {
        blink(number * 2024, remaining - 1)
    }
}
