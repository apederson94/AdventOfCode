// mod day1;

mod day1;
mod day2;
mod day3;

pub(crate) fn main() {

    let input_file1 = get_filepath("day1/input.txt");
    let input_file2 = get_filepath("day2/input.txt");
    let input_file3 = get_filepath("day3/input.txt");

    let day1_part1_result = day1::part1(input_file1.as_str());
    println!("Day 1 - Part 1 Result: {}", day1_part1_result);

    let day1_part2_result = day1::part2(input_file1.as_str());
    println!("Day 1 - Part 2 Result: {}", day1_part2_result);

    let day2_part1_result = day2::part1(input_file2.as_str());
    println!("Day 2 - Part 1 Result: {}", day2_part1_result);

    let day2_part2_result = day2::part2(input_file2.as_str());
    println!("Day 2 - Part 2 Result: {}", day2_part2_result);

    let day3_part1_result = day3::part1(input_file3.as_str());
    println!("Day 3 - Part 1 Result: {}", day3_part1_result);
}

fn get_filepath(local_path: &str) -> String {
    let base_path = "/users/austinpederson/Developer/AOC/2023/rust/advent/src/";
    format!("{}{}", base_path, local_path)
}