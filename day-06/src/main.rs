use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use crate::Direction::North;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    solve1(&problem);
    solve2(&problem);

    Ok(())
}

fn solve1(problem: &Problem) {

    println!("{} distinct positions will the guard visit before leaving the mapped area",
             problem.calculate_visited_points().len())

}

fn solve2(problem: &Problem) {
    let res = problem.calculate_visited_points()
                            .iter()
                            .filter(|point| {
                                    problem.can_be_made_obstructed(point)
                                       && problem.make_point_obstructed(point).results_in_loop()
                             })
                            .count();
    println!("{} different positions could you choose for this obstruction.", res)
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Problem {
    map: Vec<Vec<char>>,
}

impl Problem {

    fn width(&self) -> i32 {
        self.map.first().unwrap().len() as i32
    }

    fn height(&self) -> i32 {
        self.map.len() as i32
    }

    fn get_char_on_point(&self, point: &Point) -> Option<char> {
        if self.is_on_map(point) {
            Some(self.map[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    fn iter_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height()).flat_map(move |y| {
            (0..self.width()).map(move |x| Point::new(x, y))
        })
    }

    fn is_on_map(&self, point: &Point) -> bool {
        point.x < self.width()  && point.x >= 0  && point.y >= 0 && point.y < self.height()
    }

    fn is_obstructed(&self, point: &Point) -> bool {
        self.is_on_map(point) && self.get_char_on_point(point) == Some('#')
    }

    fn start_position(&self) -> Point {
        self.iter_points().find(|point| self.get_char_on_point(point) == Some('^')).unwrap()
    }

    fn can_be_made_obstructed(&self, point: &Point) -> bool {
        self.is_on_map(point) && self.get_char_on_point(point) == Some('.')
    }

    fn make_point_obstructed(&self, point: &Point) -> Self {
        let mut next_problem = self.clone();
        next_problem.map[point.y as usize][point.x as usize] = '#';
        next_problem
    }

    fn results_in_loop(&self) -> bool {
        let start_position = self.start_position();
        let mut state = State::new(start_position, North);

        let mut visited_positions: HashSet<State> = HashSet::new();

        while self.is_on_map(&state.point) {
            if visited_positions.contains(&state) {
                return true;
            }
            visited_positions.insert(state.clone());
            let mut next_state = state.step();
            if self.is_obstructed(&next_state.point) {
                next_state = state.rotate()
            }
            state = next_state
        }

        false
    }

    fn calculate_visited_points(&self) -> HashSet<Point>   {
        let start_position = self.start_position();
        let mut visited_positions: HashSet<Point> = vec![start_position.clone()].into_iter().collect();
        let mut state = State::new(start_position, North);

        while self.is_on_map(&state.point) {
            visited_positions.insert(state.point.clone());
            let mut next_state = state.step();
            if self.is_obstructed(&next_state.point) {
                next_state = state.rotate()
            }
            state = next_state
        }

        visited_positions
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

    fn add(&self, other: &Point) -> Self {
        Point { x: self.x + other.x, y: self.y + other.y  }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct State {
    point: Point,
    direction: Direction
}

impl State {
    fn new(point: Point, direction: Direction) -> Self {
        State { point, direction }
    }

    fn step(&self) -> Self {
        let dxdy = match self.direction {
            Direction::North => Point::new(0, -1),
            Direction::South => Point::new(0, 1),
            Direction::East => Point::new(1, 0),
            Direction::West => Point::new(-1, 0),
        };
        State::new(self.point.add(&dxdy), self.direction)
    }

    fn rotate(&self) -> Self {
        let next_direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        State::new(self.point.clone(), next_direction)
    }
}

fn read_input(filename: &String) -> io::Result<Problem> {
    let file_in = File::open(filename)?;
    let map = BufReader::new(file_in)
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Ok(Problem { map })
}