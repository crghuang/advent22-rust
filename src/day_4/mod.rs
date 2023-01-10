use std::fs;

pub fn day_4() {
  println!("\n\n--- Day 4 ---");

  let cleaning_list = fs::read_to_string("./src/day_4/input.txt").expect("ERROR : Unable to read file!");

  let pairs = cleaning_list.split_whitespace();
  let mut full_overlaps = 0;
  let mut part_overlaps = 0;

  for pair in pairs {
    let split = pair.split(",").collect::<Vec<&str>>();
    let (start1, end1) = parse_range(split[0]);
    let (start2, end2) = parse_range(split[1]);

    // Part 1 - count full range overlaps
    if (start1 >= start2 && end1 <= end2) || (start1 <= start2 && end1 >= end2) {
      full_overlaps += 1;
    }

    // Part 2 - count partial range overlaps
    if (start1 <= end2 && start2 <= end1) || (start2 <= end1 && start1 <= end2) {
      part_overlaps += 1;
    }
  }
  println!("\nPart 1: {full_overlaps} full overlaps");
  println!("\nPart 2: {part_overlaps} partial overlaps");
}

fn parse_range(range: &str) -> (i32, i32) {
  let mut split = range.split("-");
  let start  = split.next();
  let end = split.next();

  match (start, end) {
    (Some(start), Some(end))=> return (start.parse::<i32>().unwrap(), end.parse::<i32>().unwrap()),
    _ => panic!("Found null for start/end!"),
  }
}