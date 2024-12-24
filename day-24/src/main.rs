use std::collections::{HashMap, HashSet};
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
    let res = evaluate(&problem).unwrap().0;
    println!("What decimal number does it output on the wires starting with z? {}", res);
}

fn solve2(problem: &Problem) {

    let x = get_number('x', &problem.inputs);
    let y = get_number('y', &problem.inputs);

    let max_z =  problem.expressions.keys()
                              .filter(|key| key.starts_with('z')) // Filter keys starting with 'z'
                              .map(|key| key[1..].parse::<usize>().unwrap())
                              .max()
                              .unwrap();

    let expected_value = x + y;
    let expected_array = usize_to_binary_vec(expected_value);


    println!("{:?}, max_z:{}", expected_array, max_z);


    // let prefix_false = max_z - expected_array.len();
    //
    // let expected_array: Vec<bool> =
    //     std::iter::repeat(false).take(prefix_false)
    //                                 .chain(expected_array.into_iter())
    //                                 .collect();

    let mut switched_wires: Vec<String> =
        do_restore_circuit(&expected_value,
                           &expected_array,
                           problem.clone(),
                           Vec::new()).unwrap();
    switched_wires.sort();
    let res = switched_wires.join(",");

    println!("what do you get if you sort the names of the eight wires involved in a swap and then join those names with commas? {}",
             res)
}

fn do_restore_circuit(expected_value: &usize,
                      expected_array: &Vec<bool>,
                      problem: Problem,
                      switched_wires: Vec<String>) -> Option<Vec<String>> {
    println!("Calling expected_value: {}, switched_wires: {:?}", expected_value, switched_wires);

    let actual = evaluate(&problem);

    if actual.is_err() {
        return None;
    }

    let (actual, input) = actual.unwrap();

    if actual == *expected_value {
        return Some(switched_wires);
    } else if switched_wires.len() >= 8 {
        return None;
    }

    for (index, expected) in expected_array.iter().rev().enumerate() {
        let key = format!("z{:02}", index);
        let x = input.get(&key);
        if x.is_none() {
            continue;
        }
        if input.get(&key).unwrap() != expected {
            let mut involved_wires : HashSet<String> = HashSet::new();
            find_wires_connected_to_key(&key, &problem.expressions, &mut involved_wires);
            for find_wire_connected_to_key in  involved_wires {
                if switched_wires.contains(&find_wire_connected_to_key) {
                    continue;
                }

                let available_wires :Vec<String> =
                    problem.wires()
                        .into_iter()
                        .filter(|other_wire| {
                            other_wire != &find_wire_connected_to_key
                                && !switched_wires.contains(other_wire)
                        })
                        .collect();

                for available_wire in available_wires {
                    let mut new_switched_wires = switched_wires.clone();
                    new_switched_wires.push(available_wire.clone());
                    new_switched_wires.push(find_wire_connected_to_key.clone());
                    let mut new_problem = problem.clone();
                    let mut new_expressions = problem.expressions.clone();
                    let l_expression = problem.expressions.get(&find_wire_connected_to_key).unwrap();
                    let r_expression = problem.expressions.get(&available_wire).unwrap();
                    new_expressions.insert(find_wire_connected_to_key.clone(), r_expression.clone());
                    new_expressions.insert(available_wire.clone(), l_expression.clone());
                    new_problem.expressions = new_expressions;

                    let answer =
                        do_restore_circuit(expected_value,
                                           expected_array,
                                           new_problem,
                                           new_switched_wires);
                    if answer != None {
                        return answer;
                    }
                }
            }
        }
    }

    None
}

// Potential optimization only look for things that have a wanted value
fn find_wires_connected_to_key(key: &String,
                               expressions: &HashMap<String, Expression>,
                               involved_wires: &mut HashSet<String>) {
    if key.starts_with("x")
       || key.starts_with("y"){
        return;
    }

    if ! key.starts_with("z") {
        involved_wires.insert(key.clone());
    }

    let expression =  expressions.get(key).unwrap();
    find_wires_connected_to_key(&expression.0, expressions, involved_wires);
    find_wires_connected_to_key(&expression.2, expressions, involved_wires);
}

fn evaluate(problem: &Problem) -> Result<(usize, HashMap<String, bool>), String> {
    let mut input: HashMap<String, bool> = problem.inputs.clone();
    let expressions: &HashMap<String, Expression> = &problem.expressions;
    let values = problem.values();
    for value in values {
        let  evaluations_active: HashSet<String> = HashSet::new();
        do_evaluate(&value, &mut input, expressions, evaluations_active)?;
    }
    Ok((get_number('z', &input), input))
}

fn usize_to_binary_vec(mut n: usize) -> Vec<bool> {
    let mut binary_vec = Vec::new();

    // Extract bits while n > 0
    while n > 0 {
        binary_vec.push((n % 2) != 0); // Get the least significant bit
        n /= 2;                         // Shift right (integer division by 2)
    }

    // Reverse the vector to get the correct order (most significant bit first)
    binary_vec.reverse();
    binary_vec
}

fn get_number(prefix: char,
              input: &HashMap<String, bool>) -> usize {

    let mut filtered_and_sorted: Vec<_> = input
        .iter()
        .filter(|(key, _)| key.starts_with(prefix)) // Filter keys starting with 'z'
        .collect();


    filtered_and_sorted.sort_by_key(|(key, _)| {
        key[1..].parse::<usize>().unwrap()
    });

    filtered_and_sorted.iter()
        .enumerate()
        .map(|(index, (_, &value))| {
            let digit = 2_usize.pow(index as u32) ; // z00 = 1, z01 = 2, etc.
            digit * value as usize // Multiply by the bool value (true = 1, false = 0)
        }).sum()
}


fn do_evaluate(value: &String,
               input:  &mut HashMap<String, bool>,
               expressions: &HashMap<String, Expression>,
               mut evaluations_active: HashSet<String>) -> Result<bool, String> {

    if input.contains_key(value) {
        return Ok(input.get(value).unwrap().clone());
    }

    if evaluations_active.contains(value) {
        return Err(String::from("Circuit contains loop"));
    }

    evaluations_active.insert(value.clone());

    let expression = expressions.get(value).unwrap();
    let l = do_evaluate(&expression.0,
                        input,
                        expressions,
                        evaluations_active.clone());
    let r = do_evaluate(&expression.2,
                        input,
                        expressions,
                        evaluations_active.clone());

    let l = l?;
    let r = r?;
    let operator = &expression.1;
    let res = match operator {
        Operator::And => l && r,
        Operator::Xor => l ^ r,
        Operator::Or => l || r,
    };
    input.insert(value.clone(), res);
    Ok(res)
}


#[derive(Debug, Clone)]
struct Problem {
    inputs: HashMap<String, bool>,
    expressions: HashMap<String, Expression>,
}

impl Problem {
    fn new(inputs: HashMap<String, bool>, expressions: HashMap<String, Expression>) -> Self {
        Self { inputs, expressions }
    }

    fn values(&self) -> Vec<String> {
        self.expressions.keys().cloned().collect()
    }

    fn wires(&self) -> HashSet<String> {
        self.expressions.keys()
                        .filter(|key| key.starts_with("z"))
                        .map(|x| x.clone())
                        .collect()
    }
}

type Expression = (String, Operator, String);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Operator {
    Xor,
    Or,
    And,
}

fn read_input(filename: &String) ->  io::Result<Problem> {
    let file_in = File::open(filename)?;
    let mut parsing_input = true;

    let mut inputs: HashMap<String, bool> = HashMap::new();
    let mut expressions: HashMap<String, (String, Operator, String)> = HashMap::new();

    for line in BufReader::new(file_in).lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            parsing_input = false;
            continue;
        }
        if parsing_input {
            let words: Vec<&str> = line.split(": ").collect();
            let key = words[0].to_string();
            let value = words[1].parse::<usize>().unwrap() == 1;

            // Insert or append the value to the Vec for the key
            inputs.insert(key, value);
        } else {
            let words: Vec<&str> = line.split(" -> ").collect();
            let key = words[1].to_string();
            let words: Vec<&str> = words[0].split_whitespace().collect();
            let operator: Operator = match words[1] {
                "XOR" => Operator::Xor,
                "AND" => Operator::And,
                "OR" => Operator::Or,
                _ => unreachable!(),
            };
            let expression: Expression = (words[0].to_string(), operator, words[2].to_string());
            expressions.insert(key, expression);
        }
    }

    Ok(Problem::new(inputs, expressions))
}
