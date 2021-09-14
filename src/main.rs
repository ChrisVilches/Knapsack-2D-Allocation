use rand::Rng;
use std::collections::HashSet;
use std::fmt::Debug;
use rand::seq::SliceRandom;

// TODO: Change some types to smaller types (e.g. i64 -> i16).

struct Container {
  width: i64,
  height: i64
}

impl Container {
  fn new(w: i64, h: i64) -> Container {
    Container {
      width: w,
      height: h
    }
  }
}

struct Item {
  width: i64,
  height: i64,
  benefit: i64
}

impl Item {
  fn new(w: i64, h: i64, b: i64) -> Item {
    Item {
      width: w,
      height: h,
      benefit: b
    }
  }
}

impl Debug for Item {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(fmt, "Item {}x{} (benefit: {})", self.width, self.height, self.benefit)
  }
}

fn probability(n: f32) -> bool {
  rand::thread_rng().gen::<f32>() <= n
}

fn crossover(list1: &Vec<i64>, list2: &Vec<i64>, cross_probability: f32) -> Vec<i64> {
  if list1.len() != list2.len() {
    panic!("Lists must have the same length");
  }

  let mut list: i64 = if probability(0.5) { 0 } else { 1 };
  let mut i = 0;
  let mut j = 0;
  let mut result: Vec<i64> = Vec::<i64>::new();

  let mut added_values: HashSet<i64> = HashSet::<i64>::new();

  // list 1 and 2 should be the same size and contain the same elements.

  loop {
    if (i == list1.len()) && (j == list2.len()) {
      break;
    }

    // This dummy value can be avoided by ensuring the current list always has items left.
    let mut val: i64 = -1;

    if list == 0 && i < list1.len() {
      val = list1[i];
      i += 1;
    } else if j < list2.len() {
      val = list2[j];
      j += 1;
    }

    if val != -1 && !added_values.contains(&val) {
      result.push(val);
      added_values.insert(val);
    }

    // Use different list?
    if probability(cross_probability) {
      list = if list == 0 { 1 } else { 0 };
    }
  }

  result
}

fn mutate(_list: &Vec<i64>) {

}

fn make_offspring(list1: &Vec<i64>, list2: &Vec<i64>) -> Vec<i64> {
  let cross = crossover(list1, list2, 0.25);
  mutate(&cross);
  cross
}

fn random_solution(n: i64) -> Vec<i64> {
  let mut sol: Vec<i64> = (0..n).collect();
  sol.shuffle(&mut rand::thread_rng());
  sol
}

fn item_fits(filled: &Vec<Vec<i64>>, item: &Item, row: i64, col: i64) -> bool {
  let rows: i64 = filled.len() as i64;
  let cols: i64 = filled[0].len() as i64;

  if (row + item.height) >= rows { return false; }
  if (col + item.width) >= cols { return false; }

  for i in row..(row + item.height) {
    for j in col..(col + item.width) {
      if filled[i as usize][j as usize] == 1 {
        return false;
      }
    }
  }

  return true;
}

fn first_empty_space(filled: &Vec<Vec<i64>>, item: &Item) -> Option<(i64, i64)> {
  for i in 0..filled.len() {
    for j in 0..filled[i].len() {
      if item_fits(filled, item, i as i64, j as i64) {
        return Some((i as i64, j as i64));
      }
    }
  }

  None
}

fn count_zeros(filled: &Vec<Vec<i64>>) -> i64 {
  let mut total = 0;
  for i in 0..filled.len() {
    for j in 0..filled[i].len() {
      if filled[i][j] == 0 {
        total += 1;
      }
    }
  }
  total
}

fn score(container: &Container, items: &[Item], solution: &Vec<i64>) -> (i64, i64) {
  // TODO: A static matrix would be faster (reset with memset or something similar).
  let mut filled = vec![vec![0; container.width as usize]; container.height as usize];

  let mut total_benefit = 0;

  // Place every item until there's no room left.
  for i in 0..solution.len() {
    let item_idx = solution[i] as usize;
    let item = &items[item_idx];
    let tuple = first_empty_space(&filled, item);
    match tuple {
      Some(values) => {
        total_benefit += item.benefit;
        let (row, col) = values;
        // TODO: Something faster like fill or memset would be better.
        for i in row..(row + item.height) {
          for j in col..(col + item.width) {
            filled[i as usize][j as usize] = 1;
          }
        }
      },
      None => {
        // If we add this break, then the allocating heuristic changes.
        // What's important is to keep the heuristic the same during the program execution
        // (i.e. don't use different heuristics for different things).
        // break;
      }
    }
  }

  //print_matrix::<i64>(&filled);
  //println!();
  let wasted_space = count_zeros(&filled);
  (total_benefit, wasted_space)
}

fn print_matrix<T>(matrix: &Vec<Vec<T>>) where T: std::fmt::Debug{
  for i in 0..matrix.len() {
    println!("{:?}", matrix[i]);
  }
}

fn mean(scores: &Vec<i64>) -> f64 {
  let mut sum: f64 = 0.0;
  for i in 0..scores.len() {
    let n = scores[i] as f64;
    sum += n;
  }
  sum / (scores.len() as f64)
}

fn standard_deviation(scores: &Vec<i64>) -> f64 {
  let mut standard_deviation: f64 = 0.0;
  let mean: f64 = mean(scores);

  for i in 0..scores.len() {
    let n = scores[i] as f64;
    standard_deviation += (n - mean) * (n - mean);
  }

  return (standard_deviation / (scores.len() as f64)).sqrt();
}

fn generate_new_population(survived_solutions: &Vec<&Vec<i64>>) -> Vec<Vec<i64>>{
  let mut result = Vec::<Vec<i64>>::new();

  for solution in survived_solutions {
    result.push(solution.to_vec()); // TODO: Inefficient?
  }

  let mut extra_offsprings: i64 = 0;

  for i in 0..survived_solutions.len() {
    // Just to make sure the amount of solutions is not too big.
    if extra_offsprings > 100 { break; }
    if probability(0.8) { continue; }
    let range = ((i + 1) as i64 .. survived_solutions.len() as i64);
    if range.is_empty() { continue; }
    let crossover_idx: i64 = rand::thread_rng().gen_range(range);
    let crossover_solution = survived_solutions[crossover_idx as usize];
    let offspring = make_offspring(survived_solutions[i], crossover_solution);
    result.push(offspring);
    extra_offsprings += 1;
  }
  result
}

fn main() {
  let container: Container = Container::new(10, 5);
  let items = [
    Item::new(4, 3, 5),
    Item::new(2, 3, 10),
    Item::new(2, 3, 32),
    Item::new(4, 1, 33),
    Item::new(2, 3, 16),
    Item::new(9, 4, 1),
    Item::new(9, 10, 2),
    Item::new(9, 5, 222),
    Item::new(2, 4, 1),
    Item::new(9, 10, 2),
    Item::new(9, 5, 2),
    Item::new(2, 4, 1),
    Item::new(2, 1, 3),
    Item::new(2, 4, 100),
  ];

  let mut max_possible_score: i64 = 0;
  for item in &items {
    max_possible_score += item.benefit;
  }

  let mut all_scores = Vec::<i64>::new();

  // List of solutions.
  let mut solutions = Vec::<Vec<i64>>::new();

  // Solutions that survived.
  let mut survived_solutions = Vec::<&Vec<i64>>::new();

  // List of solution and score (which is also a tuple).
  let mut tuples = Vec::<(&Vec<i64>, (i64, i64))>::new();

  for i in 0..50 {
    solutions.push(random_solution(items.len() as i64));
  }

  // Eval many solutions.
  for solution in &solutions {
    let score = score(&container, &items, &solution);
    all_scores.push(score.0);
    tuples.push((&solution, score));
  }

  tuples.sort_by_key(|k| k.1);
  tuples.reverse();

  let stddev: f64 = standard_deviation(&all_scores);

  for tuple in &tuples {
    // Ensure a minimum number of solutions.
    if survived_solutions.len() < 10 || tuple.1.0 > (stddev as i64) {
      survived_solutions.push(tuple.0);
    } else {
      break;
    }
  }

  for tuple in &tuples {
    let solution = tuple.0;
    let score = tuple.1;
    println!("Score of solution {:?} --> {:?}", solution, score);
  }

  println!("Std dev of all scores: {}", standard_deviation(&all_scores));
  println!("Mean score: {}", mean(&all_scores));
  println!("(Max possible score is {} when adding all objects)", max_possible_score);

  println!("THE ONES THAT WILL ORGY ({})", survived_solutions.len());

  let new_population = generate_new_population(&survived_solutions);

  println!("Resulting new population size ({})", new_population.len());
}
