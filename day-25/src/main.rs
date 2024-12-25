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
    let locks = problem.locks();
    let keys = problem.keys();
    let res =
        locks.iter().map(|lock| {
            keys.iter().filter(|key| {
                                (0..key.len()).all(
                                        |i| {
                                            lock.get(i).unwrap() + key.get(i).unwrap() < 6
                                        }
                                )}
            ).count()
        }).sum::<usize>();
    println!("How many unique lock/key pairs fit together without overlapping in any column? {}",
             res);
}

struct Problem {
    input: Vec<Vec<Vec<char>>>
}
impl Problem {
    fn new(input: Vec<Vec<Vec<char>>>) -> Self {
        Problem { input }
    }

    fn locks(&self) -> Vec<Vec<usize>> {
        self.input.iter()
                  .filter(|candidate|
                      { candidate.get(0).unwrap().iter().all(|item| *item == '#' ) }
                  )
                  .map(|lock|
                        (0..lock.get(0).unwrap().len()).map(|x|
                            (0..lock.len()).find(|y |
                                lock.get(*y)
                                    .unwrap()
                                    .get(x) == Some(&'.'))
                                .map(|y| y - 1)
                                .unwrap()).collect()
                    )
                    .collect()
    }


    fn keys(&self) -> Vec<Vec<usize>> {
        self.input.iter()
            .filter(|candidate|
                { candidate.last().unwrap().iter().all(|item| *item == '#') }
            )
            .map(|key|
                (0..key.get(0).unwrap().len()).map(|x|
                    (0..key.len()).find(|y|
                        key.get(*y)
                            .unwrap()
                            .get(x) == Some(&'#'))
                        .map(|y| key.len() - 1 - y)
                        .unwrap()).collect()
            )
            .collect()
    }
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let mut input: Vec<Vec<Vec<char>>> = Vec::new();
    let mut current: Vec<Vec<char>> = Vec::new();

    let mut it = BufReader::new(file_in).lines();
    while let Some(line) = it.next() {
        let l = line.unwrap();
        if l.is_empty() {
            input.push(current.clone());
            current.clear();
            continue;
        }
        current.push(l.chars().collect());
    }
    input.push(current.clone());

    Ok(Problem::new(input))
}
