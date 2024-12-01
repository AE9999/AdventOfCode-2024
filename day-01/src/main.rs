use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;
    solve1(input.0.clone(), input.1.clone());
    solve2(input.0, input.1);

    Ok(())
}

fn solve1(mut first: Vec<usize>, mut second: Vec<usize>) {
    first.sort();
    second.sort();
    let res: usize =
        (0..first.len()).fold(0, |acc, i| acc + first[i].abs_diff(second[i]));

    println!("{} is the total distance between your lists", res);
}

fn solve2(first: Vec<usize>, second: Vec<usize>) {

    let frequency_map =
        second.iter()
              .fold(HashMap::new(), |mut acc, &value| {
            *acc.entry(value).or_insert(0) += 1;
            acc
        });

    let res: usize = first.iter()
                          .map(|&value| value * frequency_map.get(&value).unwrap_or(&0))
                          .sum();

    println!("{} is their similarity score?", res);
}

fn read_input(filename: &String) -> io::Result<(Vec<usize>, Vec<usize>)> {
    let file_in = File::open(filename)?;
    let (mut first, mut second) = (Vec::new(), Vec::new());

    for line in BufReader::new(file_in).lines().map(|x| x.unwrap()) {
        let words: Vec<&str> = line.split_whitespace().collect();
        first.push(words[0].parse::<usize>().unwrap());
        second.push(words[1].parse::<usize>().unwrap());
    }

    Ok((first, second))
}
