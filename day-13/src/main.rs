use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;
    solve(&problem);

    Ok(())
}

fn solve(problem: &Problem) {
    let res =
        problem.claw_machines.iter()
                             .filter_map(|claw_machine|solve_claw_machine(claw_machine))
                             .sum::<usize>();
    println!("What is the fewest tokens you would have to spend to win all possible prizes? {}",
             res);

    let res =
        problem.claw_machines.iter()
            .filter_map(|claw_machine| solve_claw_machine_extended(claw_machine))
            .sum::<i64>();

    println!("What is the fewest tokens you would have to spend to win all possible prizes? {}",
            res);
}

fn solve_claw_machine(claw_machine: &ClawMachine) -> Option<usize> {

    let cost_a: usize = 3;
    let cost_b: usize = 1;
    let mut found_min: Option<usize> = None;

    for i in 0..101 {
        for j in  0..(i+1) {

            let option_a =
                claw_machine.a.mul(i).add(&claw_machine.b.mul(j));

            let option_b =
                claw_machine.a.mul(j).add(&claw_machine.b.mul(i));

            if option_a  == claw_machine.prize_location {
                let candidate = cost_a * (i as usize) + cost_b * (j as usize);
                if found_min.is_none() || candidate < found_min.unwrap() {
                    found_min = Some(candidate);
                }

            }

            if option_b == claw_machine.prize_location {
                let candidate = cost_a * (j as usize) + cost_b * (i as usize);
                if found_min.is_none() || candidate < found_min.unwrap() {
                    found_min = Some(candidate);
                }
            }
        }
    }
    found_min
}

fn solve_claw_machine_extended(claw_machine: &ClawMachine) -> Option<i64> {
    // I really, really hate math, especially linear algebra.

    let increase = Point::new(10000000000000, 10000000000000);
    let target = claw_machine.prize_location.add(&increase);

    let determinant = determinant(claw_machine)?;

    let a = divide_if_divisable((claw_machine.b.y * target.x) - (claw_machine.b.x * target.y), determinant)?;
    let b  = divide_if_divisable(-claw_machine.a.y * target.x + claw_machine.a.x * target.y, determinant)?;

    if a < 0 || b < 0 {
        None
    } else {
        Some(3 * a + b)
    }
}

fn determinant(claw_machine: &ClawMachine) -> Option<i64> {
    let a  = claw_machine.a.x;
    let c = claw_machine.a.y;
    let b  = claw_machine.b.x;
    let d = claw_machine.b.y;

    let determinant  = (a * d) - (b * c);
    if  determinant == 0 {
        None
    } else {
        Some(determinant)
    }
}

fn divide_if_divisable(nominator: i64, divisor: i64) -> Option<i64> {
    if nominator % divisor == 0 {
        Some(nominator / divisor)
    } else {
        None
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Problem {
    claw_machines: Vec<ClawMachine>
}

impl Problem {
    fn new(claw_machines: Vec<ClawMachine>) -> Self {
        Problem { claw_machines }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct ClawMachine {
    a: Point,
    b: Point,
    prize_location: Point,
}

impl ClawMachine {
    fn new(a: Point, b: Point, prize_location: Point) -> Self {
        ClawMachine {
            a,
            b,
            prize_location,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn add(&self, other: &Point) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }

    fn mul(&self, factor: i64) -> Self {
        Point::new(self.x * factor, self.y * factor)
    }
}



fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let mut it = BufReader::new(file_in).lines();
    let mut claw_machines: Vec<ClawMachine> = Vec::new();
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();
    loop {
        let line = it.next().unwrap()?;
        let captures = re.captures(line.as_str()).unwrap();
        let a = Point::new(captures[1].parse().unwrap(),
                                 captures[2].parse().unwrap());

        let line = it.next().unwrap()?;
        let captures = re.captures(line.as_str()).unwrap();
        let b = Point::new(captures[1].parse().unwrap(),
                                  captures[2].parse().unwrap());

        let line = it.next().unwrap()?;
        let captures = re.captures(line.as_str()).unwrap();
        let prize_location = Point::new(captures[1].parse().unwrap(),
                                              captures[2].parse().unwrap());
        claw_machines.push(ClawMachine::new(a, b, prize_location));

        if it.next().is_none() { break; }
    }
    Ok(Problem::new(claw_machines))
}
