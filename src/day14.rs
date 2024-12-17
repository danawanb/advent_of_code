use ::std::fs;

pub fn day_fourteen() -> usize {
    //let txt = fs::read_to_string("./src/day14_dummy.txt");
    let txt = fs::read_to_string("./src/day14.txt");

    let mut txt_str: Vec<String> = Vec::new();
    if let Ok(x) = txt {
        txt_str = x
            .split_ascii_whitespace()
            .map(|st| st.to_string())
            .collect();
    }

    let mut robotx: Vec<Robot> = Vec::new();
    let mut current = 0;

    let mut robot: Robot = Robot {
        px: 0,
        py: 0,
        vx: 0,
        vy: 0,
    };

    for i in txt_str {
        if i.contains("p") {
            let res = split_to_int(&i, 0);
            let res2 = split_to_int(&i, 1);
            robot.px = res.unwrap();
            robot.py = res2.unwrap();
            current += 1;
        }

        if i.contains("v") {
            let res = split_to_int(&i, 0);
            let res2 = split_to_int(&i, 1);
            robot.vx = res.unwrap();
            robot.vy = res2.unwrap();
            current += 1;
        }

        if current == 2 {
            robotx.push(robot);
            current = 0;
        }
    }

    let mut robotj: Vec<Robot> = Vec::new();
    for rob in robotx {
        let mut blink = rob.clone();
        for _ in 0..100 {
            blink = blink.blink(101, 103);
        }

        robotj.push(blink);
    }

    let columns = 101;
    let rows = 103;
    let mut matrix: Vec<Vec<isize>> = vec![vec![0; columns]; rows];

    for robt in robotj {
        matrix[robt.py as usize][robt.px as usize] += 1;
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for (id, mat) in matrix.iter().enumerate() {
        if id != rows / 2 {
            for (idx, matx) in mat.iter().enumerate() {
                if idx != columns / 2 {
                    if *matx != 0 {
                        //quadran 1
                        if id < rows / 2 && idx < columns / 2 {
                            q1 += matx;
                        }

                        //quadran 2
                        if id < rows / 2 && idx > columns / 2 {
                            q2 += matx;
                        }

                        //id quadran 3
                        if id > rows / 2 && idx < columns / 2 {
                            q3 += matx;
                        }

                        //id quadran 4
                        if id > rows / 2 && idx > columns / 2 {
                            q4 += matx;
                        }
                    }
                }
            }
        }
    }

    println!("{} {} {} {}", q1, q2, q3, q4);
    let totalx = q1 * q2 * q3 * q4;

    totalx as usize
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    px: isize,
    py: isize,
    vx: isize,
    vy: isize,
}

impl Robot {
    fn blink(&mut self, columns: isize, rows: isize) -> Self {
        self.px = self.px + self.vx;
        self.py = self.py + self.vy;
        if self.px < 0 {
            self.px = self.px + columns;
        }

        //jika lebih
        if self.px > columns - 1 {
            self.px = self.px - columns;
        }
        if self.py < 0 {
            self.py = self.py + rows;
        }
        if self.py > rows - 1 {
            self.py = self.py - rows;
        }

        return *self;
    }
}

fn split_to_int(input: &String, idx: usize) -> Option<isize> {
    let ress: Vec<String> = input.split(',').map(|x| x.to_string()).collect();
    let res = &ress[idx];
    if res.contains("-") {
        let res = parse_to_int(&res).unwrap() * -1;
        Some(res)
    } else {
        let res = parse_to_int(&res);
        Some(res.unwrap())
    }
}

fn parse_to_int(input: &String) -> Option<isize> {
    let res: Vec<char> = input.chars().filter(|c| c.is_numeric()).collect();
    let ress: Vec<isize> = res.iter().map(|a| a.to_string().parse().unwrap()).collect();
    Some(concat(&ress))
}

fn concat(vec: &[isize]) -> isize {
    let mut acc = 0;
    for elem in vec {
        acc *= 10;
        acc += elem;
    }
    acc
}
