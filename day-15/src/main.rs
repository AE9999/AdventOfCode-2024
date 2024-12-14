use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    solve(problem.clone());

    Ok(())
}

fn solve(mut problem: Problem) {
    problem.run_instructions();
    println!("what is the sum of all boxes' GPS coordinates? {}", problem.sum_of_gps_coordinates());
}

#[derive(Clone)]
struct Problem {
    boxes: HashSet<Point>,

    walls: HashSet<Point>,

    robot: Point,

    instructions: Vec<char>,
}

impl Problem {
    fn new(boxes: HashSet<Point>,
           walls: HashSet<Point>,
           robot: Point,
           instructions: Vec<char>) -> Self {
        Problem {
            boxes,
            walls,
            robot,
            instructions,
        }
    }

    fn char_at(&self, point: &Point) -> char {
        let is_robot = point == &self.robot;
        let is_wall = self.walls.contains(point);
        let is_box = self.boxes.contains(point);
        assert!(
            [is_robot, is_wall, is_box].iter().filter(|&&x| x).count() <= 1,
            "At most one of the booleans may be true, but this condition was violated!"
        );
        if is_box {
            'O'
        } else if is_wall {
            '#'
        } else if is_robot {
            '@'
        } else {
            '.'
        }
    }

    #[allow(dead_code)]
    fn display(&self) {
        let width = self.walls.iter().map(|p| p.x).max().unwrap() + 1;
        let height = self.walls.iter().map(|p| p.y).max().unwrap() + 1;

        println!("*****************************");
        for y in 0..height {
            let row: String =
                (0..width).map(|x|
                    self.char_at(&Point::new(x,y))).collect();
            println!("{}", row)
        }
        println!("*****************************");
        println!()
    }

    fn sum_of_gps_coordinates(&self) -> usize {
        self.boxes.iter().map(|p| {
            ((100 * p.y) + p.x) as usize
        }).sum()
    }

    fn run_instructions(&mut self) {
        //self.display();

        self.instructions.clone().iter().for_each(|instruction| {
            self.do_step(instruction);
            //self.display();
        });
    }

    fn do_step(&mut self, instruction: &char) {

        let mut effected_boxes: Vec<Point> = Vec::new();

        let dxdy =  match instruction {
            '^' => Point::new(0, -1),
            'v' => Point::new(0, 1),
            '>' => Point::new(1, 0),
            '<' => Point::new(-1, 0),
            _ => panic!("Unknown instruction {}", instruction),
        };

        let mut pointer = self.robot.clone();
        loop {
            let next_point = pointer.add(&dxdy);
            let next_char = self.char_at(&next_point);

            if next_char == '#' {
                return; // Wall nothing happens
            } else if next_char == '.' {
                // Free space perform move
                for old_box in effected_boxes.iter() {
                    self.boxes.remove(old_box);
                }
                for old_box in effected_boxes.iter() {
                    self.boxes.insert(old_box.add(&dxdy));
                }
                self.robot = self.robot.add(&dxdy);
                return
            } else if next_char == 'O' {
                effected_boxes.push(next_point.clone());
                pointer = next_point;
            } else {
                panic!("Unknown char in map {}", next_char);
            }
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
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let mut parsing_maze = true;
    let mut y = 0;

    let mut boxes: HashSet<Point> = HashSet::new();

    let mut  walls: HashSet<Point> = HashSet::new();

    let mut  robot: Point = Point::new(0, 0);

    let mut instructions: Vec<char> = Vec::new();

    for line in BufReader::new(file_in).lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            parsing_maze = false;
            continue;
        }
        if parsing_maze {
            line.chars().enumerate().for_each(|(x, input_char)| {
                let point = Point::new(x as i64, y as i64);
                match input_char {
                    'O' => { boxes.insert(point); },
                    '#' => { walls.insert(point); },
                    '@' => { robot = point; },
                    '.' => {},
                    _ => panic!("Unknown char {}", input_char),
                }
            });
            y += 1;
        } else {
            let mut next_instructions = line.chars().collect::<Vec<char>>();
            instructions.append(&mut next_instructions);
        }
    }

    Ok(Problem::new(boxes, walls, robot, instructions))
}
