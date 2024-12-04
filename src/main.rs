mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let ha = day1::day_one();
    println!("{}", ha);

    let he = day2::day_two();
    println!("{}", he);

    //let three = day3::day_three();
    let three = day3::day_tree_part_two();

    println!("{}", three);

    let four = day4::day_four_chapter_one();
    let fourx = day4::count_xmas();

    println!("{} {}", four, fourx);
}
