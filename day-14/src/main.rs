use std::collections::HashSet;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    solve1(&mut problem.clone());

    solve2(&mut problem.clone());

    Ok(())
}

fn solve1(problem: &mut Problem) {

    (0..100).for_each(|_| {
        problem.step();
    });

    println!("What will the safety factor be after exactly 100 seconds have elapsed? {}",
             problem.safety_factor());
}

fn solve2(problem: &mut Problem) {
    let mut res: usize = 0;

    let mut seen_states: HashSet<Vec<Point>> = HashSet::new();

    let x =  32;
    loop {
        let most_online = problem.most_on_line();

        let state: Vec<Point> = problem.robots.iter().map(|r| r.position.clone()).collect();

        if seen_states.contains(&state) {
            println!("We're looping");
            break;
        }
        seen_states.insert(state);

        if most_online >= x  /* problem.robots.len() / 3*/  {
            println!("Res {}", res);
            problem.display_state();
        }

        problem.step();
        res += 1;
    }
}

#[derive(Clone)]
struct Problem {
    robots: Vec<Robot>,
    width: i32,
    height: i32,
}

impl Problem {
    fn new(robots: Vec<Robot>, width: i32, height: i32) -> Self {
        Problem { robots, width, height  }
    }

    fn step(&mut self) {
        self.robots =
            self.robots.iter()
                .map(|robot| robot.move_modulo(self.width,
                                                       self.height))
                .collect::<Vec<Robot>>()
    }

    fn most_on_line(&self) -> usize {
        (0..(self.height)).map(|x|self.points_on_row(x).len()).max().unwrap()
    }

    fn points_on_row(&self, row: i32) -> Vec<i32> {
        Vec::from_iter((0..self.width).filter(|x|
            self.robots.iter().any(|robot| robot.position == Point::new(*x, row))
        ))
    }

    fn display_state(&self) {
        println!("*****************************");
        for y in 1..self.height {
            let occupied_ys_in_row = self.occupied_xs_in_row(y);
            let row: String =
                (0..self.width).map(|x|
                                            if occupied_ys_in_row.contains(&x) {
                                                '#'
                                            } else {
                                                '.'
                                            }).collect();
            println!("{}", row)
        }
        println!("*****************************");
        println!()
    }

    fn occupied_xs_in_row(&self, row: i32) -> HashSet<i32> {
        HashSet::from_iter((0..self.width).filter(|x|
            self.robots.iter().any(|robot| robot.position == Point::new(*x, row))
        ))
    }

    fn safety_factor(&self) -> usize {
        let mid_x = self.width / 2;
        let mid_y = self.height / 2;

        let lower_left: Square = (Point::new(0, self.height -1),
                                  Point::new(mid_x -1, mid_y + 1));

        let upper_left: Square = (Point::new(0, mid_y -1),
                                  Point::new(mid_x -1, 0));


        let lower_right: Square = (Point::new(mid_x + 1, self.height -1),
                                   Point::new(self.width -1, mid_y + 1));

        let upper_right: Square = (Point::new(mid_x + 1, mid_y -1),
                                   Point::new(self.width - 1, 0));

        self.robots.iter().filter(|robot| robot.is_inside(&lower_left)).count()
            * self.robots.iter().filter(|robot| robot.is_inside(&lower_right)).count()
            * self.robots.iter().filter(|robot| robot.is_inside(&upper_left)).count()
            * self.robots.iter().filter(|robot| robot.is_inside(&upper_right)).count()
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Robot {
    position: Point,
    velocity: Point,
}

impl Robot {
    fn new(position: Point, velocity: Point) -> Self {
        Robot { position, velocity }
    }

    fn move_modulo(&self, max_x: i32, max_y: i32) -> Self {
        let next_position = self.position.add_modulo(&self.velocity,
                                                           max_x,
                                                           max_y);
        Robot::new(next_position, self.velocity.clone())
    }

    fn is_inside(&self, square: &Square) -> bool {
        self.position.is_inside(square)
    }
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

    fn add_modulo(&self, other: &Point, max_x: i32, max_y: i32) -> Self {
        let mut next_x = self.x + other.x;
        if next_x >= max_x {
            next_x = next_x - max_x ;
        } else if next_x < 0 {
            next_x = max_x - next_x.abs()
        }
        let mut next_y = self.y + other.y;
        if next_y >= max_y {
            next_y = next_y - max_y;
        } else if next_y < 0 {
            next_y = max_y - next_y.abs()
        }

        Point { x: next_x, y: next_y }
    }

    fn is_inside(&self, square: &Square) -> bool {
        let (lower_left, upper_right) = square;
        self.x >= lower_left.x && self.x <= upper_right.x
        && self.y <= lower_left.y && self.y >= upper_right.y
    }
}

type Square = (Point, Point);


fn read_input(filename: &String) ->  io::Result<Problem> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    let file_in = File::open(filename)?;
    let width = 101;
    let height = 103;


    let robots =
        BufReader::new(file_in).lines()
                               .map(|x| {
                                   let line = x.unwrap();
                                   let captures =
                                       re.captures(line.as_str()).unwrap();
                                   let position =
                                       Point::new(captures.get(1).unwrap().as_str().parse().unwrap(),
                                       captures.get(2).unwrap().as_str().parse().unwrap());
                                   let velocity =
                                       Point::new(captures.get(3).unwrap().as_str().parse().unwrap(),
                                                  captures.get(4).unwrap().as_str().parse().unwrap());
                                   Robot::new(position, velocity)
                               } )
                               .collect::<Vec<Robot>>();


    Ok(Problem::new(robots, width, height))
}
