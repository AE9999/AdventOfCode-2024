use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;
    solve1(&problem);
    solve2(&problem);
    Ok(())
}

fn solve1(problem: &Problem) {
    let res: usize =
        problem.iter_points()
               .filter(|point| problem.get_char_on_point(point) == Some(0))
               .map(|start_pos|calculate_trail_head_score_for_start(&start_pos, problem))
               .sum();

    println!("{} is the sum of the scores of all trailheads on your topographic map", res);
}

fn solve2(problem: &Problem) {
    let res: usize =
        problem.iter_points()
            .filter(|point| problem.get_char_on_point(point) == Some(0))
            .map(|start_pos|calculate_distinct_trail_head_score_for_start(&start_pos, problem))
            .sum();

    println!("{} is the sum of the ratings of all trailheads", res);
}

fn calculate_trail_head_score_for_start(start_pos: &Point,
                                        problem: &Problem) -> usize {
    let mut visited_points: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();

    let dxdys =
        vec![Point::new(-1, 0), Point::new(1, 0), Point::new(0, -1), Point::new(0, 1)];

    queue.push_back(start_pos.clone());
    while !queue.is_empty() {
        let current_point = queue.pop_front().unwrap();
        if visited_points.contains(&current_point) { continue; }
        visited_points.insert(current_point.clone());
        let current_height = problem.get_char_on_point(&current_point).unwrap();
        dxdys.iter()
            .map(|dxdy| current_point.add(dxdy))
            .filter(|next_point| {
                let next_height = problem.get_char_on_point(&next_point);
                !visited_points.contains(&next_point)
                    && next_height.is_some()
                    && next_height.unwrap() == current_height + 1
            }).for_each(|np| {
            queue.push_back(np.clone())
        });
    }

    problem.iter_points()
        .filter(|p| problem.get_char_on_point(p) == Some(9)
            && visited_points.contains(p))
        .count()
}

fn calculate_distinct_trail_head_score_for_start(start_pos: &Point,
                                                 problem: &Problem) -> usize {
    let mut made_paths: HashSet<Vec<Point>> = HashSet::new();
    let mut queue: VecDeque<Vec<Point>> = VecDeque::new();

    let dxdys =
        vec![Point::new(-1, 0), Point::new(1, 0), Point::new(0, -1), Point::new(0, 1)];

    let path_start = vec![start_pos.clone()];

    queue.push_back(path_start.clone());

    while !queue.is_empty() {
        let current_path = queue.pop_front().unwrap();

        if made_paths.contains(&current_path) { continue; }
        made_paths.insert(current_path.clone());
        let current_point = current_path.last().unwrap();

        let current_height = problem.get_char_on_point(&current_point).unwrap();
        dxdys.iter()
            .map(|dxdy| {
                let next_point=  current_point.add(dxdy);
                let mut next_path = current_path.clone();
                next_path.push(next_point.clone());
                next_path
            })
            .filter(|next_path| {
                let next_point = next_path.last().unwrap();
                let next_height = problem.get_char_on_point(&next_point);
                !made_paths.contains(&next_path[..])
                    && next_height.is_some()
                    && next_height.unwrap() == current_height + 1
            }).for_each(|np| {
            queue.push_back(np.clone())
        });
    }

    made_paths.iter()
              .filter(|path| {
                let last_point = path.last().unwrap();
                problem.get_char_on_point(last_point) == Some(9)
               })
              .count()
}

struct Problem {
    map: Vec<Vec<usize>>,
}

impl Problem {
    fn new(map: Vec<Vec<usize>>) -> Self {
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

    fn get_char_on_point(&self, point: &Point) -> Option<usize> {
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
}

fn read_input(filename: &String) -> io::Result<Problem> {
    let file_in = File::open(filename)?;
    let map = BufReader::new(file_in)
        .lines()
        .map(|line| line.unwrap()
                                     .chars()
                                     .map(|c|c.to_string().parse::<usize>().unwrap())
                                     .collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();
    Ok(Problem::new(map))
}
