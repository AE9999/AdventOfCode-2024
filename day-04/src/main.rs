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
    let res: usize  =
        problem.iter_points()
               .map(|point| is_expected_char_on_point_on_map(problem, &point))
               .sum();
    println!("{} many times does XMAS appear", res);
}

fn solve2(problem: &Problem) {
    let res: usize  =
        problem.iter_points()
            .filter(|point| is_expected_x_char_on_point_on_map(problem, &point))
            .count();
    println!("{} many times does X-MAS appear", res);
}

fn is_expected_char_on_point_on_map(problem: &Problem,
                                    point: &Point) -> usize {

    let expected_chars = vec!['X', 'M', 'A', 'S'];

    let dxys = vec![Point::new(0,1),
                               Point::new(0,-1),
                               Point::new(-1,0),
                               Point::new(1,0),
                               Point::new(1,1),
                               Point::new(-1,-1),
                               Point::new(1,-1),
                               Point::new(-1,1)];

    dxys.iter().filter(|dxdy| { is_expected_char_on_point_on_map_h(problem,
                                                                           point,
                                                                           dxdy,
                                                                         0,
                                                                           &expected_chars)})
              .count()
}

fn is_sam(chars: &Option<Vec<char>>) -> bool {
    if let Some(chars) = chars {
        let concatenated: String = chars.iter().collect();
        concatenated == "SAM" || concatenated == "MAS"
    } else {
        false
    }
}

fn is_expected_x_char_on_point_on_map(problem: &Problem,
                                      point: &Point) -> bool {
    let left_cross_bar: [&Point; 3] = [&point.add(&Point::new(-1, -1)),
                                       point,
                                       &point.add(&Point::new(1, 1))];

    let left_sam = problem.get_all_chars_on_points(&left_cross_bar);

    let right_cross_bar: [&Point; 3] = [&point.add(&Point::new(1, -1)),
                                        point,
                                        &point.add(&Point::new(-1, 1))];

    let right_sam = problem.get_all_chars_on_points(&right_cross_bar);

    is_sam(&left_sam) && is_sam(&right_sam)
}

fn is_expected_char_on_point_on_map_h(problem: &Problem,
                                      point: &Point,
                                      dxdy: &Point,
                                      i: usize,
                                      expected_chars: &Vec<char>) -> bool {

    i >= expected_chars.len()
        ||  (problem.get_char_on_point(&point) == Some(expected_chars[i])
                && is_expected_char_on_point_on_map_h(problem,
                                                      &point.add(&dxdy),
                                                      dxdy,
                                                    i + 1,
                                                      expected_chars))
}


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

    fn get_all_chars_on_points(&self, points: &[&Point]) -> Option<Vec<char>> {
        // Collect all characters from the given points
        let chars: Vec<_> = points
            .iter()
            .map(|point| self.get_char_on_point(point))
            .collect();

        // Check if any point is out of bounds (None in the collected chars)
        if chars.iter().any(|&ch| ch.is_none()) {
            None
        } else {
            Some(chars.into_iter().map(|ch| ch.unwrap()).collect())
        }
    }

    fn is_on_map(&self, point: &Point) -> bool {
        point.x < self.width()  && point.x >= 0  && point.y >= 0 && point.y < self.height()
    }

    fn iter_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..self.height()).flat_map(move |y| {
            (0..self.width()).map(move |x| Point::new(x, y))
        })
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
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
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    Ok(Problem { map })
}