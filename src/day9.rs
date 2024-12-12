use std::{fs, isize, usize};

const FREE: isize = -69;

pub fn day_ninex() -> usize {
    let input = fs::read_to_string("./src/day9.txt");
    let mut id = -1;
    let mut reading_file = true;
    let mut inputx: Vec<isize> = Vec::new();

    if let Ok(mut x) = input {
        //hapus \n
        let len = x.len();
        x.truncate(len - 1);

        inputx = x
            .chars()
            .flat_map(|c| {
                //println!("{}", c);
                let n: usize = c.to_digit(10).unwrap() as usize;
                if reading_file {
                    reading_file = false;
                    id += 1;
                    vec![id; n]
                } else {
                    reading_file = true;
                    vec![FREE; n]
                }
            })
            .collect();
    }

    let inputxx = inputx.clone();

    println!("{:?}", inputx);

    let mut head = 0;
    let mut tail = inputx.len() - 1;

    while head <= tail {
        if inputx[head] != FREE {
            head += 1;
        } else if inputx[tail] == FREE {
            tail -= 1;
        } else {
            inputx[head] = inputx[tail];
            inputx[tail] = FREE;
            head += 1;
            tail -= 1;
        }
    }
    let mut total: usize = 0;
    for i in 0..inputx.len() {
        if inputx[i] != FREE {
            total += i * inputx[i] as usize;
        }
    }

    //todo chapter 2
    total
}
