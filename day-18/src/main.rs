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

fn solve1(problem: &Problem)  {

    let corrupted_bytes = problem.corrupted_bytes
        .iter()
        .take(1024)
        .collect::<HashSet<_>>();

    let res = do_solve(&corrupted_bytes).unwrap();

    println!("Afterward, what is the minimum number of steps needed to reach the exit? {}", res);
}

fn solve2(problem: &Problem) {
    let mut start = 1025;

    loop {
        let corrupted_bytes = problem.corrupted_bytes
            .iter()
            .take(start)
            .collect::<HashSet<_>>();

        let res = do_solve(&corrupted_bytes);
        if res.is_none() {
            break;
        }
        start += 1;
    }

    let res = problem.corrupted_bytes.get(start -1).unwrap();
    println!("What are the coordinates of the first byte that will prevent the exit from being reachable from your starting position? {},{}",
             res.x,
             res.y);

}

fn do_solve(corrupted_bytes: &HashSet<&Point>) -> Option<usize> {
    let max_x = 70;
    let max_y = 70;

    let start = Point::new(0,0);
    let end = Point::new(max_x, max_y);

    let dxdys =
        vec![Point::new(-1, 0), Point::new(1, 0), Point::new(0, -1), Point::new(0, 1)];

    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
    let mut seen: HashSet<Point> = HashSet::new();
    queue.push_back((start.clone(), 0));

    while let Some((position, current_distance)) = queue.pop_front() {
        if position == end {
            return Some(current_distance);
        }
        seen.insert(position.clone());

        let new_points: Vec<Point> = dxdys
            .iter()
            .map(|dxdy| position.add(dxdy))
            .filter(|p| !seen.contains(&p))
            .filter(|p| is_accessible(p, corrupted_bytes, max_x, max_y))
            .collect();

        for new_point in new_points {
            seen.insert(new_point.clone());
            queue.push_back((new_point, current_distance + 1));
        }
    }
    None
}

fn is_accessible(p: &Point,
                 corrupted_bytes: &HashSet<&Point>,
                 max_x: i64,
                 max_y: i64 ) -> bool {
    p.x >= 0
    && p.x <= max_x
    && p.y >= 0
    && p.y <= max_y
    && !corrupted_bytes.contains(p)
}

#[derive(Debug, Clone)]
struct Problem {
    corrupted_bytes: Vec<Point>
}

impl Problem {
    fn new(corrupted_bytes: Vec<Point>) -> Self {
        Problem { corrupted_bytes }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn add(&self, other: &Point) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let corrupted_bytes: Vec<Point> =
        BufReader::new(file_in).lines()
            .map(|l| l.unwrap())
            .map(|line| {
            let words: Vec<&str> = line.split(',').collect();
            Point::new(words[0].parse().unwrap(), words[1].parse().unwrap())
        }).collect();
    Ok(Problem::new(corrupted_bytes))
}
