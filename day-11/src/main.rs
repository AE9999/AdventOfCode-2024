use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let input = read_input(input)?;
    solve(&input, 25);
    solve(&input, 75);

    Ok(())
}


fn solve(input: &Vec<usize>, amount: usize) {
    let mut cache:  HashMap<(usize, usize), usize> = HashMap::new();

    let res =
        input.iter()
            .map(|stone_number|
                do_solve(*stone_number, 0, amount, &mut cache))
            .sum::<usize>();

    println!("How many stones would you have after blinking a total of {} times? {}", amount, res);
}

fn do_solve(stone_number: usize,
            current_blink: usize,
            max_depth: usize,
            cache: &mut HashMap<(usize, usize), usize>) -> usize {

    let key = (stone_number, current_blink);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let stone_number_str = stone_number.to_string();
    let res =
        if current_blink == max_depth {
            1
        } else if stone_number == 0  {
            do_solve(1, current_blink + 1, max_depth, cache)
        } else if stone_number_str.len() % 2 == 0  {
            let midpoint = stone_number_str.len() / 2;
            let (first_half, second_half) = stone_number_str.split_at(midpoint);

            let first_half = first_half.parse::<usize>().unwrap();
            let second_half = second_half.parse::<usize>().unwrap();

            let first = do_solve(first_half, current_blink + 1, max_depth, cache);
            let second = do_solve(second_half, current_blink + 1, max_depth, cache);
            first + second

        }  else {
            do_solve(stone_number * 2024, current_blink + 1, max_depth, cache)
        };
    cache.insert(key, res);
    res
}


fn read_input(filename: &String) ->  io::Result<Vec<usize>> {
    let file_in = File::open(filename)?;

    let input =
        BufReader::new(file_in).lines()
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|amount| amount.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

    Ok(input)
}
