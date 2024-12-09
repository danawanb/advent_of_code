use std::fs;

pub fn day_seven() -> i64 {
    //let txt = fs::read_to_string("./src/day7_dummy.txt");
    let txt = fs::read_to_string("./src/day7.txt");

    let mut texts: Vec<String> = Vec::new();
    if let Ok(val) = txt {
        let valx: Vec<String> = val.split("\n").map(|x| x.to_string()).collect();
        texts = valx;
    }

    let mut point: Vec<Point> = Vec::new();

    for (idx, i) in texts.iter().enumerate() {
        let mut sumx: i64 = 0;
        let mut numbers: Vec<i64> = Vec::new();

        if i.contains(":") {
            let sumxx: Vec<&str> = i.split(":").collect();
            if let Some(x) = sumxx.get(0) {
                sumx = x.parse().expect("gagal konvert sum {idx}");
            }

            if let Some(y) = sumxx.get(1) {
                let res: Vec<i64> = y
                    .split_whitespace()
                    .map(|x| x.parse().expect("gagal konvert {idx}"))
                    .collect();
                numbers = res;
            }
        }

        let single_point = Point {
            sum: sumx,
            numbers: numbers,
        };

        point.push(single_point);
    }

    let res = filter_point(point);

    let operation = [add, multiple];

    let mut point_final: Vec<Point> = Vec::new();

    let mut point_secod: Vec<Point> = Vec::new();
    for re in res {
        if recursive_operations(re.sum, re.numbers.clone(), &operation) {
            point_final.push(re);
        } else {
            point_secod.push(re);
        }
    }

    let mut sumnya: i64 = 0;

    for re in point_final {
        sumnya += re.sum;
    }

    let mut sumnya_dua: i64 = 0;
    let sec_operation = [add, multiple, concatenate];

    for re in point_secod {
        if recursive_operations(re.sum, re.numbers.clone(), &sec_operation) {
            //println!("{}", re.sum);
            //
            sumnya_dua += re.sum;
        }
    }
    sumnya + sumnya_dua
}

#[derive(Debug, Clone)]
struct Point {
    sum: i64,
    numbers: Vec<i64>,
}

fn filter_point(input: Vec<Point>) -> Vec<Point> {
    let mut inputx = input.clone();
    for (idx, id) in input.iter().enumerate() {
        if id.sum == 0 && id.numbers.len() == 0 {
            inputx.remove(idx);
        }
    }

    return inputx;
}

fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn multiple(a: i64, b: i64) -> i64 {
    a * b
}

fn concatenate(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse::<i64>().unwrap()
}

fn recursive_operations<F>(target: i64, operations: Vec<i64>, operators: &[F]) -> bool
where
    F: Fn(i64, i64) -> i64,
{
    if operations.len() == 1 {
        return operations[0] == target;
    }

    for operator in operators {
        let new_value = operator(operations[0], operations[1]);
        let mut new_list = vec![new_value];
        new_list.extend_from_slice(&operations[2..]);
        if recursive_operations(target, new_list, operators) {
            return true;
        }
    }

    false
}
