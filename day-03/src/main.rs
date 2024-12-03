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
    // TODO Concatenate all input strings in var x
    let combined_input = input.join("");
    //
    let res =
        find_multiplications_in_string_and_execute_them_conditionally(combined_input.as_str());
    println!("{} is what you get if you add up all of the results of just enabled multiplications",
             res)
}

fn find_multiplications_in_string_and_execute_them_conditionally(input: &str) -> usize {
    let mut enabled = true; // Track whether multiplications are enabled
    let mut answer: usize = 0;

    // Match "mul(x, y)", "do()", and "don't()" patterns
    let re = Regex::new(r"mul\((\d+),\s*(\d+)\)|do\(\)|don't\(\)").unwrap();

    for cap in re.captures_iter(input) {
        if let Some(mul_match) = cap.get(1) {
            // If "mul(x, y)" is matched and enabled is true
            if enabled {
                let x: usize = mul_match.as_str().parse().unwrap();
                let y: usize = cap[2].parse().unwrap();
                answer += x * y;
            }
        } else if cap.get(0).map_or(false, |m| m.as_str() == "do()") {
            // If "do()" is matched, set enabled to true
            enabled = true;
        } else if cap.get(0).map_or(false, |m| m.as_str() == "don't()") {
            // If "don't()" is matched, set enabled to false
            enabled = false;
        }
    }

    answer
}


fn read_input(filename: &String) ->  io::Result<Vec<String>> {
    let file_in = File::open(filename)?;
    Ok(BufReader::new(file_in).lines().map(|x| x.unwrap()).collect())
}
