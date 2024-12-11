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

// 799438 too low

fn calculate_fencing_bulk(region: &HashSet<Point>) -> usize {
    let area = region.len();

    let mut vertical_border_edges : HashSet<Edge> = HashSet::new();
    let mut horizontal_border_edges : HashSet<Edge> = HashSet::new();
    let mut horizontal_blocked_points : HashSet<Point> = HashSet::new();
    let mut vertically_blocked_points : HashSet<Point> = HashSet::new();

    region.iter().for_each(|point|{
        let left = point.add(&Point::new(-1, 0));
        if !region.contains(&left) {
            let (l, r)  = (Point::new(point.x, point.y),
                                       Point::new(point.x, point.y + 1));
            let edge =  (l.clone(), r.clone());
            vertical_border_edges.insert(edge);
            horizontal_blocked_points.insert(l);
            horizontal_blocked_points.insert(r);
        }
        let right = point.add(&Point::new(1, 0));
        if !region.contains(&right) {
            let (l,r ) = (Point::new(point.x + 1, point.y),
                          Point::new(point.x + 1, point.y + 1));
            let edge = (l.clone(), r.clone());
            vertical_border_edges.insert(edge);
            horizontal_blocked_points.insert(l);
            horizontal_blocked_points.insert(r);
        }

        let up = point.add(&Point::new(0, -1));
        if !region.contains(&up) {
            let (l,r ) =  (Point::new(point.x, point.y),
                                       Point::new(point.x + 1, point.y));
            let edge = (l.clone(), r.clone());
            horizontal_border_edges.insert(edge);
            vertically_blocked_points.insert(l);
            vertically_blocked_points.insert(r);
        }
        let down = point.add(&Point::new(0, 1));
        if !region.contains(&down) {
            let (l, r) = (Point::new(point.x, point.y + 1),
                                      Point::new(point.x + 1, point.y + 1));
            let edge = (l.clone(), r.clone());
            horizontal_border_edges.insert(edge);
            vertically_blocked_points.insert(l);
            vertically_blocked_points.insert(r);
        }
    });


    let mut sides = 0;

    let min_x = region.iter().map(|point| point.x).min().unwrap();
    let max_x = region.iter().map(|point| point.x).max().unwrap();
    let min_y = region.iter().map(|point| point.y).min().unwrap();
    let max_y = region.iter().map(|point| point.y).max().unwrap();

    for vertical_edge_x_coordinate in  min_x..(max_x+2) {
        let mut vertical_edges: Vec<Edge> =
            vertical_border_edges.iter()
                                 .filter(|edge| edge.0.x == vertical_edge_x_coordinate)
                                 .map(|edge|edge.clone())
                                 .collect();

        vertical_edges.sort_by(|l, r|l.0.y.cmp(&r.0.y));

        if vertical_edges.len() == 0 {
            continue
        }

        let amount_of_gaps = vertical_edges.windows(2)
                                                 .filter(|pair|
                                                            pair[1].0.y != pair[0].1.y
                                                            || vertically_blocked_points.contains(&pair[1].0))
                                                 .count();
        sides += amount_of_gaps + 1

    }

    for horizontal_edge_y_coordinate in  min_y..(max_y+2) {
        let mut horizontal_edges: Vec<Edge>  =
            horizontal_border_edges.iter()
                                   .filter(|edge| edge.0.y == horizontal_edge_y_coordinate)
                                   .map(|edge|edge.clone())
                                   .collect();

        horizontal_edges.sort_by(|l, r|l.0.x.cmp(&r.0.x));

        if horizontal_edges.len() == 0 {
            continue
        }

        let amount_of_gaps =
            horizontal_edges.windows(2)
                            .filter(|pair|
                                pair[1].0.x != pair[0].1.x
                                || horizontal_blocked_points.contains(&pair[1].0))
                            .count();

        sides += amount_of_gaps + 1
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

type Edge = (Point, Point);

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
