use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::iter::repeat;
use once_cell::sync::Lazy;

static MAIN_CONSOLE_BUTTONS_TO_LOCATIONS: Lazy<HashMap<char, Point>> = Lazy::new(|| {
    HashMap::from([
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
    ])
});

static SECONDARY_CONSOLE_BUTTONS_TO_LOCATIONS: Lazy<HashMap<char, Point>> = Lazy::new(|| {
    HashMap::from([
        ('<', Point::new(0, 0)),
        ('v', Point::new(1, 0)),
        ('>', Point::new(2, 0)),

        ('^', Point::new(1, 1)),
        ('A', Point::new(2, 1)),
    ])
});


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

    let step1_options = calculate_quickest_path_on_first_console(code);

    let min =
        step1_options.iter()
                     .map(|sequence_1| {
                         let step2_options =
                             calculate_path_on_redirect_console(sequence_1, true);
                         step2_options.iter().map(
                             |sequence_2| {
                                 let step3 = calculate_path_on_redirect_console(sequence_2,
                                                                                false);
                                 // println!("Step 3: len {} ..", step3.get(0).unwrap().len());
                                 step3.get(0).unwrap().len()
                             }
                         ).min().unwrap()
                     }).min().unwrap();

    min
}

fn calculate_quickest_path_on_first_console(code: &Vec<char>) -> Vec<Vec<char>> {
    do_calculate_quickest_path_on_first_console(code,
                                                0,
                                                MAIN_CONSOLE_BUTTONS_TO_LOCATIONS.get(&'A').unwrap().clone(),
                                                Vec::new())
}

fn do_calculate_quickest_path_on_first_console(code: &Vec<char>,
                                               mut index: usize,
                                               mut location_of_robot: Point,
                                               mut current_path: Vec<char>) -> Vec<Vec<char>> {
    let mut res = Vec::new();

    while index < code.len() {
        let c = &code[index];

        let dest = MAIN_CONSOLE_BUTTONS_TO_LOCATIONS.get(&c).unwrap();
        let dx = dest.x - location_of_robot.x;
        let dy = dest.y - location_of_robot.y;

        let force_x = dy < 0 && dy.abs() >= location_of_robot.y  && location_of_robot.x == 0;
        let force_y = dx < 0 && dx.abs() >= location_of_robot.x && location_of_robot.y == 0;

        if force_x {
            location_of_robot = location_of_robot.add(&Point::new(1, 0));
            let dx = dest.x - location_of_robot.x;
            let dy = dest.y - location_of_robot.y;
            current_path.push('>');

            let presses = if dx > 0 { '>' } else { '<' };
            current_path.extend(repeat(presses).take(dx.abs() as usize));

            let presses = if dy > 0 { '^' } else { 'v' };
            current_path.extend(repeat(presses).take(dy.abs() as usize));

            current_path.push('A');
            location_of_robot = dest.clone();
        } else if force_y {
            location_of_robot = location_of_robot.add(&Point::new(0, 1));
            let dx = dest.x - location_of_robot.x;
            let dy = dest.y - location_of_robot.y;
            current_path.push('^');

            let presses = if dy > 0 { '^' } else {'v' };
            current_path.extend(repeat(presses).take(dy.abs() as usize));

            let presses = if dx > 0 { '>' } else {'<' };
            current_path.extend(repeat(presses).take(dx.abs() as usize));

            current_path.push('A');
            location_of_robot = dest.clone();
        } else {
            let mut loption = current_path.clone();

            let presses = if dy > 0 { '^' } else {'v' };
            loption.extend(repeat(presses).take(dy.abs() as usize));
            let presses = if dx > 0 { '>' } else {'<' };
            loption.extend(repeat(presses).take(dx.abs() as usize));
            loption.push('A');
            let first_answer = do_calculate_quickest_path_on_first_console(code,
                                                                           index + 1,
                                                                           dest.clone(),
                                                                           loption);

            let mut roption = current_path.clone();
            let presses = if dx > 0 { '>' } else {'<' };
            roption.extend(repeat(presses).take(dx.abs() as usize));
            let presses = if dy > 0 { '^' } else {'v' };
            roption.extend(repeat(presses).take(dy.abs() as usize));
            roption.push('A');
            let second_answer = do_calculate_quickest_path_on_first_console(code,
                                                                            index + 1,
                                                                            dest.clone(),
                                                                            roption);

            res.extend(first_answer.into_iter());
            res.extend(second_answer.into_iter());
            return res
        }
        index += 1;
    }
    res.push(current_path);
    res
}

fn calculate_path_on_redirect_console(code: &Vec<char>,
                                      explore_all: bool) -> Vec<Vec<char>> {

    // println!("redirect: {:?}, explore all: {}", code, explore_all);

    let location_of_robot: Point = SECONDARY_CONSOLE_BUTTONS_TO_LOCATIONS.get(&'A').unwrap().clone();

    do_calculate_path_on_redirect_console(code, 0, location_of_robot, Vec::new(), explore_all)
}


fn do_calculate_path_on_redirect_console(code: &Vec<char>,
                                         mut index: usize,
                                         mut location_of_robot: Point,
                                         mut current_path: Vec<char>,
                                         explore_all: bool) -> Vec<Vec<char>> {
    let mut res = Vec::new();

    while index < code.len() {
        let c = &code[index];

        let dest = SECONDARY_CONSOLE_BUTTONS_TO_LOCATIONS.get(&c).unwrap();

        let dx = dest.x - location_of_robot.x;
        let dy = dest.y - location_of_robot.y;

        let force_x = dy > 0 && dy.abs() >= location_of_robot.y  && location_of_robot.x == 0;
        let force_y = dx < 0 && dx.abs() >= location_of_robot.x && location_of_robot.y == 1;

        if force_x  {
            // println!("Adding '>' due to location_of_robot: {:?}, dest: {:?} dx: {}, dy: {}", location_of_robot, dest, dx, dy);
            location_of_robot = location_of_robot.add(&Point::new(1, 0));
            current_path.push('>');

            let dx = dest.x - location_of_robot.x;
            let dy = dest.y - location_of_robot.y;

            let presses = if dx > 0 { '>' } else {'<' };
            current_path.extend(repeat(presses).take(dx.abs() as usize));

            let presses = if dy > 0 { '^' } else {'v' };
            current_path.extend(repeat(presses).take(dy.abs() as usize));

            current_path.push('A');
        }

        else if force_y  {
            // println!("Adding 'v' due to location_of_robot: {:?}, dest: {:?} dx: {}, dy: {}", location_of_robot, dest, dx, dy);
            location_of_robot = location_of_robot.add(&Point::new(0, -1));
            current_path.push('v');

            let dx = dest.x - location_of_robot.x;
            let dy = dest.y - location_of_robot.y;

            let presses = if dy > 0 { '^' } else {'v' };
            current_path.extend(repeat(presses).take(dy.abs() as usize));

            let presses = if dx > 0 { '>' } else {'<' };
            current_path.extend(repeat(presses).take(dx.abs() as usize));

            current_path.push('A');
        } else if ! explore_all {
            let presses = if dx > 0 { '>' } else {'<' };
            current_path.extend(repeat(presses).take(dx.abs() as usize));

            let presses = if dy > 0 { '^' } else {'v' };
            current_path.extend(repeat(presses).take(dy.abs() as usize));

            current_path.push('A');
        } else {
            let mut loption = current_path.clone();

            let presses = if dy > 0 { '^' } else {'v' };
            loption.extend(repeat(presses).take(dy.abs() as usize));
            let presses = if dx > 0 { '>' } else {'<' };
            loption.extend(repeat(presses).take(dx.abs() as usize));
            loption.push('A');
            let first_answer = do_calculate_path_on_redirect_console(code,
                                                                           index + 1,
                                                                           dest.clone(),
                                                                           loption,
                                                                           explore_all);

            let mut roption = current_path.clone();
            let presses = if dx > 0 { '>' } else {'<' };
            roption.extend(repeat(presses).take(dx.abs() as usize));
            let presses = if dy > 0 { '^' } else {'v' };
            roption.extend(repeat(presses).take(dy.abs() as usize));
            roption.push('A');
            let second_answer = do_calculate_path_on_redirect_console(code,
                                                                            index + 1,
                                                                            dest.clone(),
                                                                            roption,
                                                                            explore_all);

            res.extend(first_answer.into_iter());
            res.extend(second_answer.into_iter());
            return res
        }

        location_of_robot = dest.clone();

        index += 1;
    }

    res.push(current_path);
    res
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
