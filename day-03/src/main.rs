use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use regex::Regex;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;
    solve1(&input);
    solve2(&input);

    Ok(())
}

fn solve1(input: &Vec<String>) {
    let res: usize = input.iter()
                   .map(|input| find_multiplications_in_string_and_execute_them(input))
                   .sum();
    println!("{} is what you get if you add up all of the results of the multiplications", res)
}


fn find_multiplications_in_string_and_execute_them(input: &str) -> usize {
    // find all patters of mul(\d+, \d+) in input
    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|cap| {
            cap[1].parse::<usize>().unwrap() * cap[2].parse::<usize>().unwrap()
        })
        .sum()
}

fn solve2(input: &Vec<String>) {

    let combined_input = input.join("");

    let res =
        find_multiplications_in_string_and_execute_them_conditionally(combined_input.as_str());
    println!("{} is what you get if you add up all of the results of just enabled multiplications",
             res)
}

fn find_multiplications_in_string_and_execute_them_conditionally(input: &str) -> usize {
    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)|do\(\)|don't\(\)").unwrap();

    re.captures_iter(input).fold((true, 0), |(enabled, answer), cap| {
        if let Some(mul_match) = cap.get(1) {
            // If "mul(x, y)" is matched and enabled is true
            if enabled {
                let x: usize = mul_match.as_str().parse().unwrap();
                let y: usize = cap[2].parse().unwrap();
                (enabled, answer + x * y)
            } else {
                (enabled, answer)
            }
        } else if cap.get(0).map_or(false, |m| m.as_str() == "do()") {
            // If "do()" is matched, set enabled to true
            (true, answer)
        } else if cap.get(0).map_or(false, |m| m.as_str() == "don't()") {
            // If "don't()" is matched, set enabled to false
            (false, answer)
        } else {
            (enabled, answer)
        }
    }).1 // Return only the answer from the fold
}


fn read_input(filename: &String) ->  io::Result<Vec<String>> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in).lines().map(|x| x.unwrap()).collect())
}
