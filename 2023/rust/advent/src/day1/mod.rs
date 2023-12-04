use std::fs::read_to_string;
use regex::{Match, Regex, RegexSet};

pub(crate) fn part1(filename: &str) -> u32 {
    sum_digits_from_lines(filename)
}

pub(crate) fn part2(filename: &str) -> u32 {
    sum_numbers_from_lines(filename)
}

fn sum_digits_from_lines(filename: &str) -> u32 {
    let input = read_to_string(filename)
        .unwrap();

    input
        .lines()
        .map(|s| {
            s.chars().filter_map(|c| {
                c.to_digit(10)
            }).collect::<Vec<u32>>()
        })
        .map(|i| {
            format!("{}{}", i.first().unwrap(), i.last().unwrap())
        })
        .map(|i| { i.parse::<u32>().unwrap() })
        .reduce(|sum, i| sum + i)
        .unwrap()
}

fn sum_numbers_from_lines(filename: &str) -> u32 {
    let input = read_to_string(filename)
        .unwrap();

    let patterns = [
        "\\d", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine"
    ];
    let set = RegexSet::new(patterns).unwrap();
    let regexes: Vec<_> = set.patterns().iter()
        .map(|p| Regex::new(p).unwrap())
        .collect();

    input.lines()
        .map(|s| {
            let matches: Vec<Vec<regex::Match>> = set.matches(s)
                .into_iter()
                .map(|i| &regexes[i])
                .map(|r| {
                    r.find_iter(s).collect()
                }).collect();
            let min_num = get_min_num(matches.clone());
            let max_num = get_max_num(matches.clone());

            (min_num * 10) + max_num
        }).reduce(|sum, i| sum + i).unwrap()
}

fn get_min_num(matches: Vec<Vec<Match>>) -> u32 {
    let mut min_match_vec = matches.clone()
        .into_iter()
        .min_by(|m, o| {
            let min_one = m.iter().min_by_key(|v| v.start()).unwrap();
            let min_two = o.iter().min_by_key(|v| v.start()).unwrap();

            min_one.start().cmp(&min_two.start())
        })
        .unwrap();
    min_match_vec.sort_by_key(|m| m.start());
    let min_match = min_match_vec.first().unwrap();

    get_num_from_match(min_match.as_str()).unwrap()
}

fn get_max_num(matches: Vec<Vec<Match>>) -> u32 {
    let mut max_match_vec = matches.clone()
        .into_iter()
        .max_by(|m, o| {
            let max_one = m.iter().max_by_key(|v| v.start()).unwrap();
            let max_two = o.iter().max_by_key(|v| v.start()).unwrap();

            max_one.start().cmp(&max_two.start())
        })
        .unwrap();
    max_match_vec.sort_by_key(|m| m.start());
    let max_match = max_match_vec.last().unwrap();

    get_num_from_match(max_match.as_str()).unwrap()
}

fn get_num_from_match(input: &str) -> Option<u32> {
    match input {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        _ => None
    }
}