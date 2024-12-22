use std::cmp::{min, Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;
    solve(&problem, 20, 100);
    Ok(())
}

fn solve(problem: &Problem, distance_cheat: usize, minimal_improvement: usize)  {

    let shortest_points: HashMap<Point, Option<usize>> =
        problem.iter_points()
               .filter(|point| problem.get_char_on_point(point) != Some('#'))
               .fold(HashMap::new(), |mut acc, point| {
            acc.insert(point.clone(), do_solve(problem,
                                               &point,
                                               &problem.end_point(),
                                               None,
                                               distance_cheat,
                                               minimal_improvement));
            acc
    });

    do_solve(problem,
             &problem.start_point(),
             &problem.end_point(),
             Some(shortest_points),
             distance_cheat,
             minimal_improvement
    );
}

fn do_solve(problem: &Problem,
            start_pos: &Point,
            end_pos: &Point,
            pre_calculated_best_routes: Option<HashMap<Point, Option<usize>>>,
            distance_cheat: usize,
            minimal_improvement: usize) -> Option<usize> {

    if problem.get_char_on_point(start_pos) == Some('#') {
        return None
    }

    let start_state = State::new(start_pos.clone(),
                                 0,
                                 start_pos.clone().distance(&end_pos));

    let mut to_do: BinaryHeap<State> = BinaryHeap::new();
    to_do.push(start_state);

    let mut lb = None;
    let mut ubs: HashMap<Point, usize> =  HashMap::new();

    let mut lbs_cheats: HashMap<Cheat, usize>  = HashMap::new();

    while let Some(state) = to_do.pop() {
        let location = state.position.clone();
        let path_len = state.length.clone();

        if &location == end_pos {
            if lb.is_none() {
                lb = Some(path_len);
            }
            lb = Some(min(path_len, lb.unwrap()));
            continue;
        }
        let directions = [Direction::North, Direction::South, Direction::East, Direction::West];

        let next_states =
            directions.iter()
                      .map(|direction| {
                          let next_pos = location.add(&direction.to_dx_dy());
                          State::new(next_pos.clone(),
                              state.length + 1,
                                     next_pos.distance(&end_pos))
                      })
                      .filter(|state| {
                                  problem.is_on_map(&state.position)
                                      && problem.get_char_on_point(&state.position) != Some('#')
                                      && &state.length < ubs.get(&state.position).unwrap_or(&usize::MAX)
                      })
                     .collect::<Vec<State>>();

        if pre_calculated_best_routes.is_some() {

            let possible_exit_and_next_points =
                problem.find_all_cheat_exits_and_next_point_for_entry_point(&state.position,
                                                                            distance_cheat);

            for possible_exit_and_next_point in possible_exit_and_next_points {

                let cheat: Cheat = (location.clone(),
                                    possible_exit_and_next_point.clone());
                let current_lb = lbs_cheats.get(&cheat).unwrap_or(&usize::MAX);

                let cheating_distance =
                    pre_calculated_best_routes.as_ref()
                                              .unwrap()
                                              .get(&possible_exit_and_next_point)
                                              .unwrap();
                if cheating_distance.is_none() {
                    continue;
                }
                let cheating_distance = cheating_distance.unwrap();

                let my_result =
                    state.length +
                    possible_exit_and_next_point.clone().distance(&location) +
                    cheating_distance;

                lbs_cheats.insert(cheat, min(my_result, *current_lb));
            }
        }

       next_states.into_iter().for_each(|state| {
           ubs.insert(state.position.clone(), state.length.clone());
           to_do.push(state);
       })
    }

    if pre_calculated_best_routes.is_some() {

        let res = lbs_cheats.values()
                                  .filter(|v|lb.unwrap() >= *v + minimal_improvement)
                                  .count();
        println!("How many cheats would save you at least {} picoseconds? {}", minimal_improvement, res);
    }

    lb
}

#[derive(Clone)]
struct Problem {
    map: Vec<Vec<char>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    position: Point,
    length: usize,
    distance: usize,
}

impl State {
    fn new(position: Point,
           length: usize,
           distance: usize) -> Self {
        Self { position, length, distance }
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

    fn find_all_cheat_exits_and_next_point_for_entry_point(&self,
                                                           entry_point: &Point,
                                                           max_distance: usize) -> Vec<Point> {
        self.iter_points()
            .filter(|p| {
                self.get_char_on_point(p) != Some('#')
                     && entry_point.distance(p) <= max_distance
                     && entry_point.distance(p) >= 1
            })
            .collect::<Vec<Point>>()
    }

}

type Cheat = (Point, Point);

#[derive(Hash, Eq, PartialEq, Debug, Clone, PartialOrd, Ord)]
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
