use std::fs::read_to_string;
use regex::{Regex, RegexSet, Match, SetMatches};

pub(crate) fn part1(filename: &str) -> u32 {
    get_number_sum_in_file(filename)
}

fn get_number_sum_in_file(filename: &str) -> u32 {
    let mut y = 0;
    let patterns = [
        "\\d+",
        "[^\\d\\w\\s\\.]+"
    ];
    let set = RegexSet::new(patterns).unwrap();
    let regexes: Vec<Regex> = set.patterns().iter()
        .map(|s| Regex::new(s).unwrap())
        .collect();

    let lines: Vec<_> = read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| {
            let matches = set.matches(s);
            let line = parse_line(matches, &regexes, &s, y);
            y += 1;
            line
        }).collect();

    println!("{:?}", lines);

    0
}

fn parse_line(matches: SetMatches, regexes: &Vec<Regex>, txt: &str, y: usize) -> Line {
    let mut line = Line{
        symbols: vec![],
        numbers: vec![],
        y
    };

    matches
        .into_iter()
        .for_each(|i| {
            let r = &regexes[i];
            append_matches_to_line(&mut line, r, txt, i == 0);
        });

    line
}

fn append_matches_to_line(line: &mut Line, r: &Regex, s: &str, is_num: bool) {
    r.find_iter(s)
        .for_each(|m| {
            println!("{:?}", m);
            let pos = Position {
                x_start: m.start(),
                x_end: m.end()
            };

            if is_num {
                println!("{:?}", m);
                let value = m.as_str().parse::<u16>().unwrap();
                let num = Number {
                    value,
                    position: pos,
                };
                line.numbers.push(num);
            } else {
                let value = m.as_str().to_string();
                let sym = Symbol {
                    value,
                    position: pos
                };
                line.symbols.push(sym);
            }
        })
}

#[derive(Debug)]
struct Line {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
    y: usize
}

#[derive(Debug)]
struct Number {
    value: u16,
    position: Position,
}

#[derive(Debug)]
struct Symbol {
    value: String,
    position: Position
}

#[derive(Debug)]
struct Position {
    x_start: usize,
    x_end: usize
}