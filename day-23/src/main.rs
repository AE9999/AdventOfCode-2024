use std::collections::{HashMap, HashSet};
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

fn solve1(problem: &Problem) {
    let connections = problem.nodes_to_connected();

    let cliques_of_size_3: HashSet<String> =
        connections.keys()
                   .filter(|k| k.starts_with('t'))
                   .fold(HashSet::new(), |mut acc, key| {
                        acc.extend(find_cliques_of_size_3(key,
                                                          &connections).iter()
                                                                       .map(|s| s.to_string()));
                        acc
                   });
    let res = cliques_of_size_3.len();

    println!("How many contain at least one computer with a name that starts with 't'? {}", res);
}

fn find_cliques_of_size_3(key: &String,
                          connections: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let clique_candidates =  connections.get(key).unwrap();
    let cliques: HashSet<String> =
        clique_candidates.iter().flat_map(|clique_candidate| {
            let clique_candidate_connections = connections.get(clique_candidate).unwrap();
            let intersection = clique_candidates.intersection(clique_candidate_connections);
            intersection.map( |third| {
                let mut x = vec![key.clone(), clique_candidate.clone(), third.clone()];
                x.sort();
                x.join("-")
            })
        }).collect();
    cliques
}

struct Problem {
    connections: Vec<Vec<String>>
}

impl Problem {
    fn new(connections: Vec<Vec<String>>) -> Self {
        Self { connections }
    }

    fn nodes_to_connected(&self) -> HashMap<String, HashSet<String>> {
        self.connections.iter()
                        .fold(HashMap::new(), |mut acc, connections| {
                            // Deal with problem of multiple mutual borrows.
                            {let l = acc.entry(connections[0].clone()).or_insert(HashSet::new());
                             l.insert(connections[1].clone());}
                            {let r = acc.entry(connections[1].clone()).or_insert(HashSet::new());
                             r.insert(connections[0].clone());}
                            acc
                        })
    }
}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;

    let connections =
        BufReader::new(file_in).lines()
                               .map(|x| {
                                   x.unwrap()
                                    .split('-')
                                    .map(|x|x.to_string())
                                    .collect::<Vec<String>>()
                               } )
                               .collect::<Vec<Vec<String>>>();

    Ok(Problem::new(connections))
}
