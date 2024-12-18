use std::collections::{HashSet, HashMap};
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
    let res = problem.designs.iter()
                                              .filter(|design| {
                                                  let mut infeasible: HashSet<String> = HashSet::new();
                                                  is_design_feasible(design,
                                                                     "".to_string(),
                                                                     &problem.towels,
                                                                     &mut infeasible)
                                              })
                                              .count();
    println!("How many designs are possible? {}", res);
}

fn solve2(problem: &Problem) {
    let res = problem.designs.iter()
        .map(|design| {
            let mut infeasible: HashMap<String, usize> = HashMap::new();
            count_nr_of_feasible_designs(design,
                                         "".to_string(),
                                         &problem.towels,
                                         &mut infeasible)
        })
        .sum::<usize>();
    println!("What do you get if you add up the number of different ways you could make each design? {}",
    res)
}

fn is_design_feasible(target: &String,
                      current: String,
                      towels: &Vec<String>,
                      infeasible: &mut  HashSet<String>) -> bool {
    let r =
        target == &current
        || (!infeasible.contains(&current)
            && target.starts_with(current.as_str()) && towels.iter().any(|towel| {
            let current =  current.to_string() + towel;
            is_design_feasible(target, current, towels, infeasible)
        }));

    if !r  {
        infeasible.insert(current);
    }

    r
}

fn count_nr_of_feasible_designs(target: &String,
                                current: String,
                                towels: &Vec<String>,
                                cache: &mut  HashMap<String, usize>) -> usize {
    if target == &current {
        1
    } else if cache.contains_key(&current) {
        return *cache.get(&current).unwrap();
    } else {
        let nr_of_feasible_designs =
            if target.starts_with(current.as_str()) {
                towels.iter()
                      .map(|towel| {
                            let current =  current.to_string() + towel;
                            count_nr_of_feasible_designs(target, current, towels, cache)
                        })
                     .sum()
            } else {
                0
            };
        cache.insert(current, nr_of_feasible_designs);
        nr_of_feasible_designs
    }


}

#[derive(Debug)]
struct Problem {
    towels: Vec<String>,
    designs: Vec<String>,
}


impl Problem {
    fn new(towels: Vec<String>, designs: Vec<String>) -> Self {
        Self { towels, designs }
    }
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;

    let mut it = BufReader::new(file_in).lines();

    let towels: Vec<String> = it.next().unwrap()?.split(", ").map(|s| s.to_owned()).collect();

    it.next();

    let designs = it.map(|towel| { towel.unwrap()}).collect();

    Ok(Problem::new(towels, designs))
}
