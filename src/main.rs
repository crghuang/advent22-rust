pub mod day_1;

fn main() {
  println!("Welcome to Craig's Advent of Code 2022, Rust edition");

  let mut tmp = day_1::parse_food_list("./src/day_1_input.txt".to_string());
  let elves = &mut tmp;
  day_1::elf_with_most_calories(elves);
  day_1::top_three_elves(elves);
}
