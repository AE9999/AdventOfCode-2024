use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;
    solve1(&problem);

    Ok(())
}

fn solve1(problem: &Problem)  {

    let start_pos = problem.start_point();

    let end_pos = problem.end_point();

    let start_state = State::new(vec![start_pos.clone()],
                                        None,
                                        start_pos.clone().distance(&end_pos));

    let mut to_do: BinaryHeap<State> = BinaryHeap::new();
    to_do.push(start_state);

    let mut local_ubs: HashMap<(Point, Cheat), usize> =  HashMap::new();
    let mut global_lbs: HashMap<Cheat, usize> =  HashMap::new();

    while let Some(state) = to_do.pop() {

        let location = state.path.last().unwrap().clone();
        let path_len = state.path.len();
        let mut global_lb_mine = global_lbs.get(&state.cheated).unwrap_or(&usize::MAX);

        if location == end_pos {
            global_lb_mine = min(global_lb_mine, &path_len);
            // println!("Found a global lb. Key:{:?}, value: {}", state.cheated, global_lb_mine);
            global_lbs.insert(state.cheated.clone(), *global_lb_mine);
            continue;
        }

        for &direction in &[Direction::North, Direction::South, Direction::East, Direction::West] {
            explore_if_feasible(&state,
                                &end_pos,
                                &direction,
                                &mut local_ubs,
                                &mut global_lbs,
                                &problem)
                .map(|new_states|
                    new_states.into_iter().for_each(
                        |new_state| to_do.push(new_state)));
        }
    }

    let normal_length = global_lbs.get(&None).unwrap();
    
    let res = global_lbs.values()
                              .filter(|ub| *ub + 100 <= *normal_length)
                              .count();


    println!("How many cheats would save you at least 100 picoseconds? {}", res);
}

fn explore_if_feasible(state: &State,
                       end_pos: &Point,
                       direction: &Direction,
                       local_ubs: &mut HashMap<(Point, Cheat), usize>,
                       global_lbs: &mut HashMap<Cheat, usize>,
                       problem: &Problem) -> Option<Vec<State>> {
    let current_position = state.path.last().unwrap();
    let next_pos = current_position.add(&direction.to_dx_dy());

    let states_to_check =
        if problem.get_char_on_point(&next_pos) == Some('#') {
            if state.cheated.is_some() {
                return None
            }
            to_cheating_endpoints(direction, current_position).iter()
                                                              .filter(|endpoint| {
                                                                  problem.is_on_map(endpoint)
                                                                  && problem.get_char_on_point(&endpoint) != Some('#')
                                                              })
                                                              .map(|endpoint| {
                                                                  let mut path= state.path.clone();
                                                                  path.push(next_pos.clone());
                                                                  path.push(endpoint.clone());
                                                                  let cheated = Some((next_pos.clone(), endpoint.clone()));
                                                                  let distance = endpoint.distance(&end_pos);
                                                                  State::new(path,
                                                                             cheated,
                                                                             distance)
                                                              })
                                                              .collect()
        } else {
            let mut current_path = state.path.clone();
            current_path.push(next_pos.clone());
            let next_distance = end_pos.distance(&next_pos);
            let next_state = State::new(current_path,
                                              state.cheated.clone(),
                                              next_distance);
            vec!(next_state)
        };

    let mut next_states: Vec<State> = Vec::new();
    let minimal_improvement = 100_usize;

    for state in states_to_check {

        let has_cheated = state.cheated.is_some();

        let global_lb_mine = global_lbs.get(&state.cheated).unwrap_or(&usize::MAX);

        let current_ub = state.path.len() + state.distance;

        let global_lb_no_cheat = global_lbs.get(&None).unwrap_or(&usize::MAX);
        if  current_ub > *global_lb_mine
            || (has_cheated && (current_ub + minimal_improvement > *global_lb_no_cheat)) {
            continue;
        };

        let my_key = (next_pos.clone(), state.cheated.clone());
        let no_secret_key = (next_pos.clone(), None);
        let local_ub_mine = local_ubs.get(&my_key).unwrap_or(&usize::MAX);
        let local_ub_no_secret = local_ubs.get(&no_secret_key).unwrap_or(&usize::MAX);

        if &state.path.len() > local_ub_mine
           || (has_cheated && &state.path.len() + minimal_improvement > *local_ub_no_secret) {
            continue;
        }

        local_ubs.insert(my_key, state.path.len());
        next_states.push(state.clone());
    }

    Some(next_states)
}

fn to_cheating_endpoints(direction: &Direction,
                         point: &Point) -> Vec<Point> {
    let other_move =
        match &direction {
            Direction::North => {
                [Direction::North, Direction::East, Direction::West]
            },
            Direction::South => {
                [Direction::South, Direction::East, Direction::West]
            },
            Direction::East => {
                [Direction::North, Direction::South, Direction::East]
            },
            Direction::West => {
                [Direction::North, Direction::South, Direction::West]
            },
        };
    other_move.iter().map(|other_move| {
        point.add(&direction.to_dx_dy()).add(&other_move.to_dx_dy())
    }).collect()
}


#[derive(Clone)]
struct Problem {
    map: Vec<Vec<char>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    path: Vec<Point>,
    cheated: Option<(Point, Point)>,
    distance: usize,
}

impl State {
    fn new(path: Vec<Point>,
           cheated: Option<(Point, Point)>,
           distance: usize) -> Self {
        Self { path, cheated, distance }
    }
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

    fn start_point (&self) -> Point {
        self.iter_points().find(|p| self.get_char_on_point(p) == Some('S')).unwrap()
    }

    fn end_point (&self) -> Point {
        self.iter_points().find(|p| self.get_char_on_point(p) == Some('E')).unwrap()
    }

}

type Cheat = Option<(Point, Point)>;

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

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // First, compare the `cheated` field
        match (self.cheated.is_none(), other.cheated.is_none()) {
            (true, false) => Ordering::Greater,  // `self` has higher priority (cheated is None)
            (false, true) => Ordering::Less, // `other` has higher priority (cheated is None)
            _ => {
                // If both are None or both are Some, fall back to `distance`
                other.distance.cmp(&self.distance)
            }
        }
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
