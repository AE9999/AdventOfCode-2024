use std::cmp::PartialEq;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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
    let antana_type_to_their_points: HashMap<char, HashSet<Point>> =
        problem.iter_points()
               .filter(|point| problem.get_char_on_point(point) != Some('.')
                                      && problem.get_char_on_point(point) != Some('#'))
               .fold(HashMap::new(), |mut acc, point| {
                    acc.entry(problem.get_char_on_point(&point).unwrap())
                       .or_insert_with(HashSet::new).insert(point);
                    acc
               });

    let res: usize =
        problem.iter_points()
               .filter(|point| is_antinode(&antana_type_to_their_points, point))
               .count();

    println!("{} many unique locations within the bounds of the map contain an antinode.", res);

    let res: usize = problem.iter_points()
                            .filter(|point|
                                is_antinode_relaxed(&antana_type_to_their_points,
                                                    point,
                                                    problem))

                            .count();

    println!("{} many unique locations within the bounds of the map contain an antinode.", res);
}

fn is_antinode_relaxed(antana_type_to_their_points: &HashMap<char, HashSet<Point>>,
                       point: &Point,
                       problem: &Problem) -> bool {
    antana_type_to_their_points.values().any(|points| {
        points.iter()
            .tuple_combinations() // Generate all unique pairs of points
            .any(|(antana1, antana2)|
                match_antinode_condition_relaxed(antana1,
                                                 antana2,
                                                 point,
                                                 problem))
    })
}

fn match_antinode_condition_relaxed(antana1: &Point,
                                    antana2: &Point,
                                    point: &Point,
                                    problem: &Problem) -> bool {
    let (mut l, mut r) = if antana1.x <= antana2.x {
        (antana1.clone(), antana2.clone())
    } else {
        (antana2.clone(), antana1.clone())
    };
    let difference = l.difference(&r);

    while problem.is_on_map(&l) {
        if point == &l {
            return true;
        }
        l = l.minus(&difference);
    }

    while problem.is_on_map(&r) {
        if point == &r {
            return true;
        }
        r = r.add(&difference);
    }

    false
}


fn is_antinode(antana_type_to_their_points: &HashMap<char, HashSet<Point>>,
               point: &Point) -> bool {
    antana_type_to_their_points.values().any(|points| {
        points.iter()
              .tuple_combinations() // Generate all unique pairs of points
              .any(|(antana1, antana2)| match_antinode_condition(antana1, antana2, point))
    })
}

fn match_antinode_condition(antana1: &Point,
                            antana2: &Point,
                            point: &Point) -> bool {
    let (l, r) = if antana1.x <= antana2.x {
        (antana1, antana2)
    } else {
        (antana2, antana1)
    };
    let difference = l.difference(r);
    let l_antinode = l.minus(&difference);
    let r_antinode = r.add(&difference);
    point == &l_antinode || point == &r_antinode
}

#[derive(Debug)]
struct Problem {
    map: Vec<Vec<char>>
}

impl Problem {

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

    fn minus(&self, other: &Point) -> Self {
        Point { x: self.x - other.x, y: self.y - other.y  }
    }

    fn difference(&self, other: &Point) -> Point {
        if self.x <= other.x {
            Point::new(other.x - self.x, other.y - self.y)
        } else {
            Point::new(self.x - other.x, self.y - other.y)
        }
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