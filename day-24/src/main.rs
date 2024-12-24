use std::collections::HashMap;
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
    let res = evaluate(&problem);
    println!("What decimal number does it output on the wires starting with z? {}", res);
}

fn evaluate(problem: &Problem) -> usize {
    let mut input: HashMap<String, bool> = problem.inputs.clone();
    let expressions: &HashMap<String, Expression> = &problem.expressions;
    let values = problem.values();
    for value in values {
        do_evaluate(&value, &mut input, expressions);
    }

    let mut filtered_and_sorted: Vec<_> = input
        .iter()
        .filter(|(key, _)| key.starts_with('z')) // Filter keys starting with 'z'
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
               expressions: &HashMap<String, Expression>) -> bool {

    if input.contains_key(value) {
        return input.get(value).unwrap().clone();
    }

    let expression = expressions.get(value).unwrap();
    let l = do_evaluate(&expression.0, input, expressions);
    let r = do_evaluate(&expression.2, input, expressions);
    let operator = &expression.1;
    let res = match operator {
        Operator::And => l && r,
        Operator::Xor => l ^ r,
        Operator::Or => l || r,
    };
    input.insert(value.clone(), res);
    res
}


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
