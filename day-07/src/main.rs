use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    solve1(&problem);
    solve2(&problem);

    Ok(())
}

fn solve1(problem: &Problem) {
    let res: isize = problem.equations.iter()
                                      .filter(|calibration| is_solvable(calibration.target,
                                                                               calibration.components[0],
                                                                                    &calibration.components[1..].to_vec()))
                                      .map(|calibration| {
                                          calibration.target
                                      })
                                      .sum();
    println!("{} is their total calibration result", res);
}

fn solve2(problem: &Problem) {
    let res: isize = problem.equations.iter()
        .filter(|calibration| is_solvable_with_third_operator(calibration.target,
                                                                          calibration.components[0],
                                                                          &calibration.components[1..].to_vec()))
        .map(|calibration| {
            calibration.target
        })
        .sum();
    println!("{} is their total calibration result", res);
}

fn is_solvable(target: isize,
               acc: isize,
               components: &Vec<isize>) -> bool {
    if target == acc && components.len() == 0 {
        return true;
    }
    if components.len() == 0 || target < acc {
        return false
    }
    let next_components = components[1..].to_vec();
    let l = acc + components[0];
    let r = acc * components[0];
    is_solvable(target, l, &next_components) || (is_solvable(target, r, &next_components))
}

fn is_solvable_with_third_operator(target: isize,
                                   acc: isize,
                                   components: &Vec<isize>) -> bool {
    if target == acc && components.len() == 0 {
        return true;
    }
    if components.len() == 0 || target < acc {
        return false
    }
    let next_components = components[1..].to_vec();
    let l = acc + components[0];
    let r = acc * components[0];
    let t = format!("{}{}", acc, components[0]).parse::<isize>().unwrap();
    is_solvable_with_third_operator(target, l, &next_components)
        || (is_solvable_with_third_operator(target, r, &next_components))
        || (is_solvable_with_third_operator(target, t, &next_components))
}

struct Problem {
    equations: Vec<Calibration>
}
impl Problem {
    fn new(equations: Vec<Calibration>) -> Self {
        Problem { equations }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Calibration {
    target: isize,
    components: Vec<isize>,
}

impl Calibration {
    fn new(target: isize, components: Vec<isize>) -> Self {
        Calibration { target, components }
    }
}

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    //         let words: Vec<&str> = line.split_whitespace().collect();
    let calibrations: Vec<Calibration> =
        BufReader::new(file_in).lines()
                               .map(|l| l.unwrap())
                               .map(|line|  {
            let words: Vec<&str> = line.split(": ").collect();
            let target= words[0].parse::<isize>().unwrap();
            let components: Vec<isize> = words[1].split_whitespace()
                                                 .map(|num| num.parse::<isize>().unwrap()).collect();
            Calibration::new(target, components)
        }).collect();

    Ok(Problem::new(calibrations))
}
