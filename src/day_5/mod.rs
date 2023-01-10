use std::fs;
use regex::Regex;

pub fn day_5() {
  println!("\n\n--- Day 5 ---");

  let mut stacks_9k = vec![
    vec!["F", "T", "C", "L", "R", "P", "G", "Q"],
    vec!["N", "Q", "H", "W", "R", "F", "S", "J"],
    vec!["F", "B", "H", "W", "P", "M", "Q"],
    vec!["V", "S", "T", "D", "F"],
    vec!["Q", "L", "D", "W", "V", "F", "Z"],
    vec!["Z", "C", "L", "S"],
    vec!["Z", "B", "M", "V", "D", "F"],
    vec!["T", "J", "B"],
    vec!["Q", "N", "B", "G", "L", "S", "P", "H"],
  ];
  let mut stacks_9k1 = stacks_9k.clone();

  let text = fs::read_to_string("./src/day_5/input.txt").expect("ERROR : Unable to read file!");

  let lines = text.split("\n").collect::<Vec<&str>>();

  let re = Regex::new(r"^move\s+([1-9][0-9]*)\s+from\s+([1-9][0-9]*)\s+to\s+([1-9][0-9]*)$").unwrap();

  for line in lines {
    if re.is_match(line) {
      let step = re
        .captures(line)
        .unwrap();

      let moves = step
        .get(1)
        .unwrap()
        .as_str()
        .parse::<i32>()
        .unwrap();

      let from = step
        .get(2)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap() - 1;

      let to = step
        .get(3)
        .unwrap()
        .as_str()
        .parse::<usize>()
        .unwrap() - 1;

      for _n in 1..=moves {
        let item = stacks_9k[from]
          .pop()
          .unwrap();
        stacks_9k[to].push(item);
      }

      let from_length = stacks_9k1[from].len();
      let items: Vec<&str> = stacks_9k1[from].drain(from_length-usize::try_from(moves).ok().unwrap()..).collect();

      stacks_9k1[to].extend_from_slice(&items);
    }
  }

  println!("\nPart 1: Crates on top using CrateMover 9000 are '{}'", print_top_crates(&stacks_9k));
  println!("\nPart 2: Crates on top using CrateMover 9001 are '{}'", print_top_crates(&stacks_9k1));
}

fn print_top_crates(stacks: &Vec<Vec<&str>>) -> String {
  let mut top_crates = String::from("");
  for i in 0..stacks.len() {
    top_crates.push_str(stacks[i].last().unwrap());
  }
  return top_crates
}

fn print_stacks(stacks: &Vec<Vec<&str>>) {
  println!();
  for i in 0..stacks.len() {
    println!("{} {:?}", i+1, stacks[i]);
  }
}