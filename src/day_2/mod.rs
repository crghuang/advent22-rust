use crate::utility;

pub fn day_2() {
  println!("\n\n--- Day 2 ---");

  if let Ok(lines) = utility::read_lines("./src/day_2/input.txt".to_string()) {
    let mut total_score_pt1: u32 = 0;
    let mut total_score_pt2: u32 = 0;

    // Parse input line-by-line
    for line in lines {
      if let Ok(selections) = line {
        let split = selections.split_whitespace();
        let vec = split.collect::<Vec<&str>>();

        // Accumulate round score to total
        total_score_pt1 += calculate_round_score_pt1(vec[0].to_string(), vec[1].to_string());
        total_score_pt2 += calculate_round_score_pt2(vec[0].to_string(), vec[1].to_string());
      }
    }
    println!("\nPart 1: Total score = {total_score_pt1}");
    println!("\nPart 2: Total score = {total_score_pt2}");
  }
}

fn calculate_round_score_pt1(opponent: String, you: String) -> u32 {
  let your_shape = letter_to_shape(you);
  let outcome = find_result(letter_to_shape(opponent).as_ref().unwrap(), your_shape.as_ref().unwrap());

  return result_value(outcome) + shape_value(your_shape)
}

fn calculate_round_score_pt2(opponent: String, you: String) -> u32 {
  let outcome = letter_to_result(you);
  let opponent_shape = letter_to_shape(opponent);
  let your_shape = find_shape(opponent_shape.unwrap(), outcome.as_ref().unwrap());

  return result_value(outcome.unwrap()) + shape_value(Some(your_shape));
}

// Part 1: Your letter indicates shape, determine outcome based off opponent and your shapes
fn find_result(opponent: &Shape, you: &Shape) -> Outcome {
  let duple = (opponent, you);

  match duple {
    (Shape::Rock, Shape::Paper) | (Shape::Paper, Shape::Scissors) | (Shape::Scissors, Shape::Rock) => Outcome::Win,
    (Shape::Rock, Shape::Scissors) | (Shape::Paper, Shape::Rock) | (Shape::Scissors, Shape::Paper) => Outcome::Loss,
    (Shape::Rock, Shape::Rock) | (Shape::Paper, Shape::Paper) | (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
  }
}

// Part 2: Your letter indicates result, convert result to your played shape based off known outcome
fn find_shape(shape: Shape, outcome: &Outcome) -> Shape {
  match outcome {
    Outcome::Win => return match shape {
      Shape::Rock => Shape::Paper,
      Shape::Paper => Shape::Scissors,
      Shape::Scissors => Shape::Rock,
    },
    Outcome::Loss => return match shape {
      Shape::Rock => Shape::Scissors,
      Shape::Paper => Shape::Rock,
      Shape::Scissors => Shape::Paper,
    },
    Outcome::Draw => return match shape {
      Shape::Rock => Shape::Rock,
      Shape::Paper => Shape::Paper,
      Shape::Scissors => Shape::Scissors,
    },
  }
}

#[derive(PartialEq)]
enum Shape {
  Rock,
  Paper,
  Scissors,
}

#[derive(PartialEq)]
enum Outcome {
  Win,
  Draw,
  Loss,
}

fn letter_to_shape(letter: String) -> Option<Shape> {
  match letter.as_ref() {
    "A" | "X" => Some(Shape::Rock),
    "B" | "Y" => Some(Shape::Paper),
    "C" | "Z" => Some(Shape::Scissors),
    _ => None,
  }
}

fn letter_to_result(letter: String) -> Option<Outcome> {
  match letter.as_ref() {
    "X" => Some(Outcome::Loss),
    "Y" => Some(Outcome::Draw),
    "Z" => Some(Outcome::Win),
    _ => None,
  }
}

fn shape_value(shape: Option<Shape>) -> u32 {
  match shape {
    None => todo!(),
    Some(Shape::Rock) => 1,
    Some(Shape::Paper) => 2,
    Some(Shape::Scissors) => 3,
  }
}

fn result_value(result: Outcome) -> u32 {
  match result {
    Outcome::Win => 6,
    Outcome::Draw => 3,
    Outcome::Loss => 0,
  }
}
