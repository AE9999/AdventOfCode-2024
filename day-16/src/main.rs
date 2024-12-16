use std::cmp::min;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::cmp::Ordering;
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;
    solve(&problem);

    Ok(())
}

fn solve(problem: &Problem) {
    let mut ub: usize = usize::MAX;

    let start_pos =
        problem.iter_points().find(|p| problem.get_char_on_point(p) == Some('S')).unwrap();

    let end_pos =
        problem.iter_points().find(|p| problem.get_char_on_point(p) == Some('E')).unwrap();

    let mut completed_paths : Vec<State> = Vec::new();

    let start_state = State::new(Direction::East,
                                  0,
                                        start_pos.clone().distance(&end_pos),
                                 vec![start_pos.clone()]);

    let mut lbs: HashMap<(Point, Direction), usize> =  HashMap::new();

    let mut to_do: BinaryHeap<State> = BinaryHeap::new();
    to_do.push(start_state);

    while let Some(state) = to_do.pop() {
        let location = state.path.last().unwrap().clone();
        if location == end_pos {
            ub = min(ub, state.score);
            completed_paths.push(state);
            continue;
        }

        for &direction in &[Direction::North, Direction::South, Direction::East, Direction::West] {
            explore_if_feasible(&state, direction, &end_pos, &mut lbs, &ub, &problem)
                .map(|new_state| to_do.push(new_state));
        }

    }

    println!("What is the lowest score a Reindeer could possibly get? {}", ub);

    let mut best_path_tiles: HashSet<Point> = HashSet::new();
    completed_paths.iter().filter(|state| state.score == ub).for_each(|state| {
        state.path.iter().for_each(|point| { best_path_tiles.insert(point.clone()); })
    });

    println!("How many tiles are part of at least one of the best paths through the maze? {}",
             best_path_tiles.len());
}

fn explore_if_feasible(current_state: &State,
                       direction: Direction,
                       end_pos: &Point,
                       lbs:  &mut HashMap<(Point, Direction), usize>,
                       ub: &usize,
                       problem: &Problem) -> Option<State> {
    let dxdy = direction.to_dx_dy();
    let next_point = current_state.path.last().unwrap().add(&dxdy);
    let next_score = current_state.score + 1 + (current_state.turns_required(direction) * 1000);
    let next_distance = next_point.distance(end_pos);
    let next_char = problem.get_char_on_point(&next_point);

    let mut approximate_score = next_score + next_distance;
    if direction == Direction::North {
        if next_point.x != end_pos.x {
            approximate_score += 1000; // we need to turn east or west
        }
        if next_point.y > end_pos.y {
            approximate_score += 1000; // we are going up, but after going east or west we will also need to go south agin
        }
    } else if direction == Direction::South {
        if next_point.x != end_pos.x {
            approximate_score += 1000; // we need to turn east or west
        }
        if next_point.y < end_pos.y {
            approximate_score += 1000; // we are going up, but after going east or west we will also need to go south agin
        }
    } else if direction == Direction::East {
        if next_point.y != end_pos.y {
            approximate_score += 1000; // we need to turn south or nord
        }
        if next_point.x > end_pos.x {
            approximate_score += 1000; // we are going too east, need to go west at least once
        }
    } else {
        assert!(direction == Direction::West);
        if next_point.y != end_pos.y {
            approximate_score += 1000; // we need to turn south or nord
        }
        if next_point.x < end_pos.x {
            approximate_score += 1000; // we are going too east, need to go east at least once
        }
    }

    if approximate_score > *ub {
        return None
    }

    if next_char == Some('S')
        || next_char == Some('#') {
        return None
    }

    let key = (next_point.clone(), direction.clone());
    let lb =
        if lbs.contains_key(&key) {
            *lbs.get(&key).unwrap()
        } else {
            usize::MAX
        };
    if lb < next_score.clone() {
        None
    } else {
        lbs.insert(key, min(lb, next_score.clone()) );
        let mut path = current_state.path.clone();
        path.push(next_point);
        Some(State::new(direction, next_score, next_distance, path))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    direction: Direction,
    score: usize,
    distance: usize,
    path: Vec<Point>
}

impl State {
    fn new(direction: Direction,
           score: usize,
           distance: usize,
           path: Vec<Point>) -> Self {
        State { direction, score, distance, path }
    }

    fn turns_required(&self, next_direction: Direction) -> usize {
        match self.direction {
            Direction::North => {
                match next_direction {
                    Direction::North => 0,
                    Direction::East => 1,
                    Direction::South => 2,
                    Direction::West => 1,
                }
            },
            Direction::East => {
                match next_direction {
                    Direction::North => 1,
                    Direction::East => 0,
                    Direction::South => 1,
                    Direction::West => 2,
                }
            },
            Direction::South => {
                match next_direction {
                    Direction::North => 2,
                    Direction::East => 1,
                    Direction::South => 0,
                    Direction::West => 1,
                }
            },
            Direction::West => {
                match next_direction {
                    Direction::North => 1,
                    Direction::East => 2,
                    Direction::South => 1,
                    Direction::West => 0,
                }
            },
        }
    }
}


impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order to make it a min-heap (lower priority is better)
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn to_dx_dy(&self) -> Point {
        match &self {
            Direction::North => Point::new(0, 1),
            Direction::South => Point::new(0, -1),
            Direction::East => Point::new(1, 0),
            Direction::West => Point::new(-1, 0),
        }
    }
}

#[derive(Clone)]
struct Problem {
    map: Vec<Vec<char>>,
}

impl Problem {
    fn new(map: Vec<Vec<char>>) -> Self {
        Problem { map}
    }

    fn width(&self) -> i32 {
        self.map.first().unwrap().len() as i32
    }

    fn height(&self) -> i32 {
        self.map.len() as i32
    }
    fn iter_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height()).flat_map(move |y| {
            (0..self.width()).map(move |x| Point::new(x, y))
        })
    }

    fn is_on_map(&self, point: &Point) -> bool {
        point.x < self.width()  && point.x >= 0  && point.y >= 0 && point.y < self.height()
    }

    fn get_char_on_point(&self, point: &Point) -> Option<char> {
        if self.is_on_map(point) {
            Some(self.map[point.y as usize][point.x as usize])
        } else {
            None
        }
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

    fn distance(&self, other: &Point) -> usize {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as usize
    }
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let map = BufReader::new(file_in)
        .lines()
        .map(|line| line.unwrap()
            .chars()
            .map(|c|c.to_string().parse::<char>().unwrap())
            .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Ok(Problem::new(map))
}
