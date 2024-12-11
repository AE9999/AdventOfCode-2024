use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let doubly_linked_list = read_input(input)?;

    solve(doubly_linked_list);

    Ok(())
}

fn solve(mut doubly_linked_list: DoublyLinkedList) {

    loop {
        let finger_id = doubly_linked_list.finger.unwrap();
        let tail_id = doubly_linked_list.tail.unwrap();

        if finger_id == tail_id {
            break;
        }

        if !doubly_linked_list.is_free(finger_id) {
            doubly_linked_list.finger_next();
            continue;
        }

        if doubly_linked_list.is_free(tail_id) {
            doubly_linked_list.pop();
            continue;
        }

        let file_id_of_tail = doubly_linked_list.get_file_id(tail_id);
        let amount_free_in_finger = doubly_linked_list.get_amount(finger_id);
        let amount_of_items_in_tail =  doubly_linked_list.get_amount(tail_id);


        let amount_stored_in_finger =
            if amount_free_in_finger >= amount_of_items_in_tail {
                amount_of_items_in_tail
            } else {
                amount_free_in_finger
            };

        let amount_of_item_left_in_tail = amount_of_items_in_tail - amount_stored_in_finger;
        let amount_of_item_left_in_finger = amount_free_in_finger - amount_stored_in_finger;

        doubly_linked_list.set_file_id(finger_id, file_id_of_tail);
        doubly_linked_list.set_amount(finger_id, amount_stored_in_finger);

        if amount_of_item_left_in_tail > 0 {
            doubly_linked_list.set_amount(tail_id, amount_of_item_left_in_tail);
        } else {
            doubly_linked_list.pop();
        }

        if amount_of_item_left_in_finger > 0 {
            doubly_linked_list.insert_after_finger(amount_of_item_left_in_finger, None);
            doubly_linked_list.finger_next();
            continue;
        }
    }

    doubly_linked_list.finger_reset();


    let mut checksome: usize = 0;
    let mut index: usize = 0;

    loop {
        let finger_id = doubly_linked_list.finger.unwrap();
        let tail_id = doubly_linked_list.tail.unwrap();

        let is_free = doubly_linked_list.is_free(finger_id);
        let amount = doubly_linked_list.get_amount(finger_id);

        if !is_free {
            let file_id = doubly_linked_list.get_file_id(finger_id);
            checksome += (index..index+amount).map(|i| file_id * i).sum::<usize>();
        }

        index += amount;

        if finger_id == tail_id {
            break;
        }

        doubly_linked_list.finger_next();
    }

    println!("{} is the resulting filesystem checksum", checksome);
}


#[derive(Debug, Clone)]
struct Node {
    file_id: Option<usize>,
    amount: usize,
    prev: Option<usize>,
    next: Option<usize>,
}

#[derive(Debug, Clone)]
struct DoublyLinkedList {
    nodes: HashMap<usize, Node>,
    head: Option<usize>,
    tail: Option<usize>,
    finger: Option<usize>,
}

impl DoublyLinkedList {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            head: None,
            tail: None,
            finger: None,
        }
    }

    fn push(&mut self, amount: usize, file_id: Option<usize>) {
        let new_id = self.nodes.len();

        let new_node = Node {
            file_id,
            amount,
            prev: self.tail,
            next: None,
        };

        if let Some(tail_id) = self.tail {
            self.nodes.get_mut(&tail_id).unwrap().next = Some(new_id);
        }

        self.tail = Some(new_id);
        if self.head.is_none() {
            self.head = Some(new_id);
        }

        self.nodes.insert(new_id, new_node);
        if self.finger.is_none() {
            self.finger = Some(new_id);
        }
    }

    fn pop(&mut self) {
        if let Some(tail_id) = self.tail {
            if let Some(prev_id) = self.nodes.get(&tail_id).unwrap().prev {
                self.nodes.get_mut(&prev_id).unwrap().next = None;
                self.tail = Some(prev_id);
            } else {
                self.head = None;
                self.tail = None;
            }
            //self.nodes.remove(&tail_id);
        }
    }

    fn insert_after_finger(&mut self, amount: usize, file_id: Option<usize>) {
        if let Some(finger_id) = self.finger {

            let new_id = self.nodes.len();

            let finger_node = self.nodes.get_mut(&finger_id).unwrap();
            let next_id = finger_node.next;

            let new_node = Node {
                file_id,
                amount,
                prev: Some(finger_id),
                next: next_id,
            };

            finger_node.next = Some(new_id);

            if let Some(next_id) = next_id {
                self.nodes.get_mut(&next_id).unwrap().prev = Some(new_id);
            } else {
                self.tail = Some(new_id);
            }

            self.nodes.insert(new_id, new_node);
        }
    }

    fn finger_next(&mut self) {
        if let Some(finger_id) = self.finger {
            self.finger = self.nodes.get(&finger_id).unwrap().next;
        }
    }

    fn finger_reset(&mut self) {
        self.finger = self.head;
    }

    fn is_free(&self, id: usize) -> bool {
        self.nodes.get(&id).unwrap().file_id.is_none()
    }

    fn get_amount(&self, id: usize) -> usize {
        self.nodes.get(&id).unwrap().amount
    }

    fn get_file_id(&self, id: usize) -> usize {
        self.nodes.get(&id).unwrap().file_id.unwrap()
    }

    fn set_amount(&mut self, id: usize, amount: usize) {
        self.nodes.get_mut(&id).unwrap().amount = amount;
    }

    fn set_file_id(&mut self, id: usize, file_id: usize ) {
        self.nodes.get_mut(&id).unwrap().file_id = Some(file_id);
    }
}


fn read_input(filename: &String) ->  io::Result<DoublyLinkedList> {
    let file_in = File::open(filename)?;

    let mut doubly_linked_list = DoublyLinkedList::new();

    let line =
        BufReader::new(file_in).lines().map(|line| line.unwrap()).next().unwrap();

    let mut is_free = false;
    let mut id: usize = 0;

    for c in line.chars() {
        let node_id = if is_free {
            None
        } else {
            id += 1;
            Some(id - 1)
        };

        let amount = c.to_string().parse().unwrap();
        doubly_linked_list.push(amount, node_id);

        is_free = !is_free;
    }

    Ok(doubly_linked_list)
}
