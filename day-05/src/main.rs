use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let problem = read_input(input)?;
    problem.solve1();
    problem.solve2();
    Ok(())
}


struct Problem {
    rules: HashMap<usize, Vec<usize>>,
    reverse_rules: HashMap<usize, Vec<usize>>,
    orders: Vec<Vec<usize>>,
}

impl Problem {
    fn new(rules: HashMap<usize, Vec<usize>>, orders: Vec<Vec<usize>>) -> Self {

        let reverse_rules: HashMap<usize, Vec<usize>> =
            rules.iter()
                 .flat_map(|(&key, values)|
                                values.iter().map(move |&value| (value, key)))
                 .fold(HashMap::new(), |mut acc, (value, key)| {
                        acc.entry(value).or_insert_with(Vec::new).push(key);
                        acc });

        Problem {
            rules,
            reverse_rules,
            orders
        }
    }

    fn solve1(&self) {
        let res: usize = self.orders.iter()
                                    .filter(|order| !self.is_order_incorrect(order))
                                    .map(|order| order.get(order.len() / 2).unwrap())
                                    .sum();
        println!("you get {} if you add up the middle page number from those correctly-ordered updates",
                 res)
    }

    fn solve2(&self) {
        let res: usize = self.orders.iter()
                                    .filter(|order| self.is_order_incorrect(order))
                                    .map(|order| self.put_in_order(order))
                                    .map(|o| o[o.len() / 2]) // Directly access the middle value
                                    .sum();
        println!("you get {} if you add up the middle page numbers after correctly ordering just those updates",
                 res)
    }

    fn put_in_order(&self, order: &Vec<usize>) -> Vec<usize> {
        let mut todo_list = order.clone();
        let mut ordered_list: Vec<usize> = Vec::new();

        while let Some(&next) = todo_list.iter().find(|&&item| {
            self.reverse_rules.get(&item).map_or(true, |deps| {
                deps.iter().all(|&dep| !todo_list.contains(&dep))
            })
        }) {
            todo_list.retain(|&x| x != next);
            ordered_list.push(next);
        }

        ordered_list
    }

    fn is_order_incorrect(&self, order: &Vec<usize>) -> bool {
        let order_map: HashMap<usize, usize> = order.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (idx, &val)| {
                acc.insert(val, idx);
                acc
            });

        order.iter()
             .enumerate()
             .any(|(idx, &val)| {
                self.rules.get(&val)
                    .map_or(false, |rule_values|
                        rule_values.iter()
                            .any(|&rule_value|
                                order_map.get(&rule_value).unwrap_or(&order.len()) < &idx))
            })
    }

}


fn read_input(filename: &String) ->  io::Result<Problem> {
    let mut parsing_rules = true;
    let file_in = File::open(filename)?;

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut orders: Vec<Vec<usize>> = Vec::new();

    for line in BufReader::new(file_in).lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            let words: Vec<&str> = line.split('|').collect();
            let key = words[0].parse::<usize>().unwrap();
            let value = words[1].parse::<usize>().unwrap();

            // Insert or append the value to the Vec for the key
            rules.entry(key).or_insert_with(Vec::new).push(value);
        } else {
            let order: Vec<usize> =
                line.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
            orders.push(order);
        }
    }

    Ok(Problem::new(rules, orders))
}
