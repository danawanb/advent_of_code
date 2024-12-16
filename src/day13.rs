use std::{fs, isize};

pub fn day_thirteen() -> usize {
    //let txt = fs::read_to_string("./src/day13_dummy.txt");
    let txt = fs::read_to_string("./src/day13.txt");

    let mut txt_v: Vec<String> = Vec::new();
    if let Ok(x) = txt {
        txt_v = x.split("\n").map(|a| a.to_string()).collect();
    }

    let mut pricex: Vec<Claw> = Vec::new();
    for i in txt_v {
        let mut button = Button {
            label: 'X',
            x: 0,
            y: 0,
        };

        let mut price = Price { x: 0, y: 0 };

        if i.contains("Button") {
            if i.contains("Button A") {
                let numx = parse_str(&i, 0);
                let numy = parse_str(&i, 1);
                let numxx: usize = numx.unwrap().parse().unwrap();
                let numyy: usize = numy.unwrap().parse().unwrap();
                button.label = 'A';
                button.x = numxx;
                button.y = numyy;
            } else {
                let numx = parse_str(&i, 0);
                let numy = parse_str(&i, 1);
                let numxx: usize = numx.unwrap().parse().unwrap();
                let numyy: usize = numy.unwrap().parse().unwrap();
                button.label = 'B';
                button.x = numxx;
                button.y = numyy;
            }
        }

        if i.contains("Prize:") {
            let numx = parse_str(&i, 0);
            let numy = parse_str(&i, 1);
            let numxx: usize = numx.unwrap().parse().unwrap();
            let numyy: usize = numy.unwrap().parse().unwrap();
            price.x = numxx;
            price.y = numyy;
        }

        let single_claw = Claw { button, price };
        pricex.push(single_claw);
    }

    let mut fix_claw: Vec<FixClaw> = Vec::new();
    let mut single_claws = FixClaw {
        button_a: Button {
            label: 'A',
            x: 0,
            y: 0,
        },
        button_b: Button {
            label: 'B',
            x: 0,
            y: 0,
        },
        price: Price { x: 0, y: 0 },
    };

    let mut current = 0;

    for (_, i) in pricex.iter().enumerate() {
        if i.button.label == 'A' {
            single_claws.button_a.x = i.button.x;
            single_claws.button_a.y = i.button.y;
            current += 1;
        }
        if i.button.label == 'B' {
            single_claws.button_b.x = i.button.x;
            single_claws.button_b.y = i.button.y;
            current += 1;
        }

        if current == 2 && i.button.label == 'X' && i.price.x > 0 && i.price.y > 0 {
            //part 2;
            single_claws.price.x = i.price.x;
            single_claws.price.y = i.price.y;

            fix_claw.push(single_claws);
            current = 0;
        }
    }

    //println!("{:?}", fix_claw);

    let mut totalx = 0;
    for fix in &fix_claw {
        for i in 1..=100 {
            for j in 1..=100 {
                //println!("{} {}", i, j);
                let res = fix.prize_press(i, j);
                if res {
                    //println!(
                    //   "press A : {} press B : {} token_total -> {} {:?}",
                    //    i,
                    //    j,
                    //    (3 * i) + (1 * j),
                    //    fix
                    //);

                    totalx += (3 * i) + (1 * j);
                }
            }
        }
    }
    println!("part1 {}", totalx);

    let totalxx: usize = fix_claw
        .iter()
        .filter_map(|claw| claw.min_cost_to_win(10000000000000))
        .sum();

    totalxx
}

#[derive(Debug, Clone, Copy)]
struct FixClaw {
    button_a: Button,
    button_b: Button,
    price: Price,
}

impl FixClaw {
    fn prize_press(&self, a: usize, b: usize) -> bool {
        let xx = (self.button_a.x * a) + (self.button_b.x * b);
        let yy = (self.button_a.y * a) + (self.button_b.y * b);

        if xx == self.price.x && yy == self.price.y {
            return true;
        }

        return false;
    }

    //part 2
    fn min_cost_to_win(&self, add_to_prize: isize) -> Option<usize> {
        let rx = self.price.x as isize + add_to_prize;
        let ry = self.price.y as isize + add_to_prize;
        let ax = self.button_a.x as isize;
        let ay = self.button_a.y as isize;
        let bx = self.button_b.x as isize;
        let by = self.button_b.y as isize;
        let top = bx * ry - by * rx;
        let bot = ay * bx - ax * by;
        if top % bot != 0 {
            return None;
        }
        let pa = top / bot;
        if (rx - (pa * ax)) % bx != 0 {
            return None;
        }
        let pb = (rx - (pa * ax)) / bx;
        Some(pa as usize * 3 + pb as usize * 4)
    }
}

#[derive(Debug, Clone, Copy)]
struct Claw {
    button: Button,
    price: Price,
}

#[derive(Debug, Clone, Copy)]
struct Button {
    label: char,
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy)]
struct Price {
    x: usize,
    y: usize,
}
fn parse_str(input: &String, index: usize) -> Option<String> {
    let ress: Vec<&str> = input.split(',').collect();
    let res: String = ress[index].chars().filter(|c| c.is_numeric()).collect();
    Some(res)
}
