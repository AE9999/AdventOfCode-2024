use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;

    solve(&problem);

    Ok(())
}


fn solve(problem: &Problem) {
    let mut regions: Vec<HashSet<Point>> = Vec::new();

    for point in problem.iter_points() {
        if regions.iter().any(|set| set.contains(&point)) {
            continue;
        }
        regions.push(problem.find_region(&point));
    }

    let res = regions.iter().map(|region|calculate_fencing(region)).sum::<usize>();
    println!("What is the total price of fencing all regions on your map? {res}");

    let res = regions.iter().map(|region|calculate_fencing_bulk(region)).sum::<usize>();
    println!("What is the new total price of fencing all regions on your map? {res}");
}

fn calculate_fencing(region: &HashSet<Point>) -> usize {

    let dxdys =
        vec![Point::new(-1, 0), Point::new(1, 0), Point::new(0, -1), Point::new(0, 1)];

    let area = region.len();

    let perimeter =
        region.iter()
              .map(|point|{
                   dxdys.iter()
                        .filter(|dxdy| !region.contains(&point.add(dxdy)))
                        .count()
              })
            .sum::<usize>();

    perimeter * area
}

fn calculate_fencing_bulk(region: &HashSet<Point>) -> usize {
    let area = region.len();

    let mut sides = 0;

    let min_x = region.iter().map(|point| point.x).min().unwrap();
    let max_x = region.iter().map(|point| point.x).max().unwrap();
    let min_y = region.iter().map(|point| point.y).min().unwrap();
    let max_y = region.iter().map(|point| point.y).max().unwrap();

    for x in  min_x..(max_x+1) {

    }

    for y in  min_y..(max_y+1) {

    }

    area * sides
}


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

    fn find_region(&self, point: &Point) -> HashSet<Point> {
        let mut points = HashSet::new();

        let mut queue: VecDeque<Point> = VecDeque::new();

        let dxdys =
            vec![Point::new(-1, 0), Point::new(1, 0), Point::new(0, -1), Point::new(0, 1)];

        queue.push_back(point.clone());
        while !queue.is_empty() {
            let current_point = queue.pop_front().unwrap();
            if points.contains(&current_point) {
                continue;
            }
            points.insert(current_point.clone());
            dxdys.iter()
                .map(|dxdy| current_point.add(dxdy))
                .filter(|next_point| {
                    self.get_char_on_point(point) == self.get_char_on_point(&next_point)
                    && !points.contains(&next_point)
                }).for_each(|np| {
                queue.push_back(np.clone())
            });
        }

        points
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
            .map(|c|c.to_string().parse::<char>().unwrap())
            .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Ok(Problem::new(map))
}
