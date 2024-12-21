use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::iter::repeat;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;
    solve1(&problem);

    Ok(())
}

fn solve1( problem: &Problem) {
    let res =
        problem.codes.iter()
                     .map(|code|
                              { let x = calculate_quickest_path(code);
                                let y = code.iter().take(3).collect::<String>().parse::<usize>().unwrap();
                              println!("{} * {} = {}", x, y, x*y);
                          x * y }
                     )
                     .sum::<usize>();
    println!("What is the sum of the complexities of the five codes on your list? {}", res);
}

fn calculate_quickest_path(code: &Vec<char>) -> usize {

    let step1 = calculate_quickest_path_on_first_console(code);
    println!("Step 1: {:?}", step1.iter().collect::<String>());
    let step2 = calculate_path_on_redirect_console(&step1);
    println!("Step 2: {:?}", step2.iter().collect::<String>());
    let step3 = calculate_path_on_redirect_console(&step2);
    println!("Step 3: {:?}", step3.iter().collect::<String>());
    step3.len()
}

fn calculate_quickest_path_on_first_console(code: &Vec<char>) -> Vec<char> {
    let buttons_to_locations: HashMap<char, Point> = HashMap::from([
        ('0', Point::new(1, 0)),
        ('A', Point::new(2, 0)),
        ('1', Point::new(0, 1)),
        ('2', Point::new(1, 1)),
        ('3', Point::new(2, 1)),
        ('4', Point::new(0, 2)),
        ('5', Point::new(1, 2)),
        ('6', Point::new(2, 2)),
        ('7', Point::new(0, 3)),
        ('8', Point::new(1, 3)),
        ('9', Point::new(2, 3)),
    ]);

    let mut inputs = Vec::new();

    let mut location_of_robot: Point = buttons_to_locations.get(&'A').unwrap().clone();

    let mut last_index = 0;

    for c in code {
        println!("calculating: {}  location_of_robot: {:?}",
                 c,
                 buttons_to_locations.iter().find((|(k,v)| v == &&location_of_robot)).unwrap());
        let dest = buttons_to_locations.get(&c).unwrap();

        let dx = dest.x - location_of_robot.x;
        let dy = dest.y - location_of_robot.y;

        if dy < 0 && dy.abs() >= location_of_robot.y  && location_of_robot.x == 0 {
            location_of_robot = location_of_robot.add(&Point::new(1, 0));
            inputs.push('>');
        }

        if dx < 0 && dx.abs() >= location_of_robot.x && location_of_robot.y == 0 {
            location_of_robot = location_of_robot.add(&Point::new(0, 1));
            inputs.push('^');
        }

        let dx = dest.x - location_of_robot.x;
        let presses = if dx > 0 { '>' } else {'<' };
        inputs.extend(repeat(presses).take(dx.abs() as usize));

        let dy = dest.y - location_of_robot.y;
        let presses = if dy > 0 { '^' } else {'v' };
        inputs.extend(repeat(presses).take(dy.abs() as usize));

        inputs.push('A');
        location_of_robot = dest.clone();

        println!("inputs: {}", inputs.iter().skip(last_index).collect::<String>());
        last_index = inputs.len();
    }

    inputs
}

fn calculate_path_on_redirect_console(code: &Vec<char>) -> Vec<char> {
    let buttons_to_locations: HashMap<char, Point> = HashMap::from([
        ('<', Point::new(0, 0)),
        ('v', Point::new(1, 0)),
        ('>', Point::new(2, 0)),

        ('^', Point::new(1, 1)),
        ('A', Point::new(2, 1)),

    ]);

    let mut inputs = Vec::new();

    let mut location_of_robot: Point = buttons_to_locations.get(&'A').unwrap().clone();

    let mut last_index = 0;

    for c in code {
        println!("calculating: {}  location_of_robot: {:?}",
                 c,
                 buttons_to_locations.iter().find((|(k,v)| v == &&location_of_robot)).unwrap());
        let dest = buttons_to_locations.get(&c).unwrap();

        let dx = dest.x - location_of_robot.x;
        let dy = dest.y - location_of_robot.y;

        if dy > 0 && dy.abs() >= location_of_robot.y  && location_of_robot.x == 0 {
            // println!("Adding '>' due to location_of_robot: {:?}, dest: {:?} dx: {}, dy: {}", location_of_robot, dest, dx, dy);
            location_of_robot = location_of_robot.add(&Point::new(1, 0));
            inputs.push('>');
        }

        if dx < 0 && dx.abs() >= location_of_robot.x && location_of_robot.y == 1 {
            // println!("Adding 'v' due to location_of_robot: {:?}, dest: {:?} dx: {}, dy: {}", location_of_robot, dest, dx, dy);
            location_of_robot = location_of_robot.add(&Point::new(0, -1));
            inputs.push('v');
        }

        let dx = dest.x - location_of_robot.x;
        let presses = if dx > 0 { '>' } else {'<' };
        inputs.extend(repeat(presses).take(dx.abs() as usize));

        let dy = dest.y - location_of_robot.y;
        let presses = if dy > 0 { '^' } else {'v' };
        inputs.extend(repeat(presses).take(dy.abs() as usize));

        inputs.push('A');
        println!("inputs: {}", inputs.iter().skip(last_index).collect::<String>());
        last_index = inputs.len();
        location_of_robot = dest.clone();
    }

    inputs
}

struct Problem {
    codes: Vec<Vec<char>>
}

impl Problem {
    fn new(codes: Vec<Vec<char>>) -> Self { Problem { codes } }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn add(&self, other: &Point) -> Self {
        Point { x: self.x + other.x, y: self.y + other.y  }
    }
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let codes: Vec<Vec<char>> = BufReader::new(file_in).lines().map(|l| l.unwrap().chars().collect()).collect();
    Ok(Problem::new(codes))
}
