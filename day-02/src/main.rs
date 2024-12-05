use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;
    solve1(&input);
    solve2(&input);

    Ok(())
}

fn solve1(raports: &Vec<Vec<usize>> ) {
    let res: usize = raports.iter().filter(|rapport| is_safe(rapport)).count();
    println!("{} reports are safe", res);
}

fn solve2(raports: &Vec<Vec<usize>> ) {
    let res: usize = raports.iter().filter(|rapport| is_safe_with_a_single_removal(rapport)).count();
    println!("{} reports are now safe", res);
}

fn is_safe(rapport: &Vec<usize>) -> bool {
    (1..rapport.len()).all(|i| {
        (rapport[i - 1] != rapport[i])
            && ((rapport[i] < rapport[i - 1]) == (rapport[1] < rapport[0]))
            && rapport[i].abs_diff(rapport[i - 1]) <= 3
    })
}

fn is_safe_with_a_single_removal(rapport: &Vec<usize>) -> bool {
    is_safe(rapport)
        || (0..rapport.len()).any(|i| is_safe(&vector_from_vector_with_missing_index(rapport,
                                                                                            i)))
}

fn vector_from_vector_with_missing_index(rapport: &Vec<usize>, index: usize) -> Vec<usize> {
    let (left, right) = rapport.split_at(index);
    [left, &right[1..]].concat()
}

fn read_input(filename: &String) ->  io::Result<Vec<Vec<usize>>> {
    let file_in = File::open(filename)?;
    let rvalue: Vec<Vec<usize>> = BufReader::new(file_in)
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    Ok(rvalue)
}
