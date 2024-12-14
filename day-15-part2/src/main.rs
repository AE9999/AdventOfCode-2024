use std::collections::{HashSet, VecDeque};
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
    left_half_boxes: HashSet<Point>,

    right_half_boxes: HashSet<Point>,

    walls: HashSet<Point>,

    robot: Point,

    instructions: Vec<char>,
}

impl Problem {
    fn new(left_half_boxes: HashSet<Point>,
           right_half_boxes: HashSet<Point>,
           walls: HashSet<Point>,
           robot: Point,
           instructions: Vec<char>) -> Self {
        Problem {
            left_half_boxes,
            right_half_boxes,
            walls,
            robot,
            instructions,
        }
    }

    fn char_at(&self, point: &Point) -> char {
        let is_robot = point == &self.robot;
        let is_wall = self.walls.contains(point);
        let is_left_box = self.left_half_boxes.contains(point);
        let is_right_box = self.right_half_boxes.contains(point);
        assert!(
            [is_robot, is_wall, is_left_box, is_right_box].iter().filter(|&&x| x).count() <= 1,
            "At most one of the booleans may be true, but this condition was violated!"
        );
        if is_left_box {
            '['
        } else if is_right_box {
            ']'
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
        self.left_half_boxes.iter().map( |p| {
            ((100 * p.y) + p.x) as usize
        }).sum()
    }

    fn run_instructions(&mut self) {
        // self.display();

        self.instructions.clone().iter().for_each(|instruction| {
            self.do_step(instruction);
            // self.display();
        });
    }

    fn do_step(&mut self, instruction: &char) {

        let mut effected_boxes: Vec<Box> = Vec::new();

        let dxdy =  match instruction {
            '^' => Point::new(0, -1),
            'v' => Point::new(0, 1),
            '>' => Point::new(1, 0),
            '<' => Point::new(-1, 0),
            _ => panic!("Unknown instruction {}", instruction),
        };

        let mut queue: VecDeque<Point> = VecDeque::new();
        queue.push_back(self.robot.add(&dxdy));

        while let Some(point) = queue.pop_front() {
            let next_char = self.char_at(&point);
            match next_char {
                '[' => {
                    let right_point = point.add(&Point::new(1, 0));
                    let next_box = (point.clone(), right_point.clone());
                    effected_boxes.push(next_box);
                    match instruction {
                        '^' | 'v' => {
                            queue.push_back(point.add(&dxdy));
                            queue.push_back(right_point.add(&dxdy));
                        },
                        '>' => {
                            queue.push_back(right_point.add(&dxdy));
                        },
                        '<' => panic!("Can't push from the left to char"),
                        _ => panic!("Unknown instruction {}", instruction),
                    };
                },
                ']' => {
                    let left_point = point.add(&Point::new(-1, 0));
                    let next_box = (left_point.clone(), point.clone());
                    effected_boxes.push(next_box);
                    match instruction {
                        '^' | 'v' => {
                            queue.push_back(left_point.add(&dxdy));
                            queue.push_back(point.add(&dxdy));
                        },
                        '>' => {
                            panic!("Can't push from the right to char")
                        },
                        '<' => {
                            queue.push_back(left_point.add(&dxdy));
                        }
                        _ => panic!("Unknown instruction {}", instruction),
                    };
                },
                '#' => {
                    return
                },
                '.' => {},
                _ => panic!("Unknown character {}", next_char),
            }
        }

        // We can push everything update flow
        self.robot = self.robot.add(&dxdy);
        effected_boxes.iter().for_each(|effected_box| {
            self.left_half_boxes.remove(&effected_box.0);
            self.right_half_boxes.remove(&effected_box.1);
        });
        effected_boxes.iter().for_each(|effected_box| {
            self.left_half_boxes.insert(effected_box.0.add(&dxdy));
            self.right_half_boxes.insert(effected_box.1.add(&dxdy));
        });

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

type Box = (Point, Point);

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let mut parsing_maze = true;
    let mut y = 0;

    let mut left_half_boxes: HashSet<Point> = HashSet::new();

    let mut right_half_boxes: HashSet<Point> = HashSet::new();

    let mut  walls: HashSet<Point> = HashSet::new();

    let mut  robot: Point = Point::new(0, 0);

    let mut instructions: Vec<char> = Vec::new();

    for line in BufReader::new(file_in).lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            parsing_maze = false;
            continue;
        }
        if parsing_maze {
            line.chars().enumerate().for_each(|(index, input_char)| {
                let left_point = Point::new((index * 2) as i64, y as i64);
                let right_point = Point::new((index * 2 + 1) as i64, y as i64);

                match input_char {
                    'O' => {
                        left_half_boxes.insert(left_point.clone());
                        right_half_boxes.insert(right_point.clone());
                    },
                    '#' => {
                        walls.insert(left_point);
                        walls.insert(right_point);
                    },
                    '@' => { robot = left_point; },
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

    Ok(Problem::new(left_half_boxes, right_half_boxes, walls, robot, instructions))
}
