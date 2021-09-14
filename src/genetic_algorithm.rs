use super::types::item::Item;
use super::types::container::Container;
use super::types::stats::Stats;
use super::math;
use rand::Rng;
use rand::seq::SliceRandom;
use md5;
use std::collections::HashSet;

// TODO: Creation of offspring can be improved.
// TODO: Some functions should be put inside the GeneticAlgorithm struct.
// TODO: Some stuff is stored in "Stats" and some other stuff in "GeneticAlgorithm". Make it consistent.
// TODO: Change some types to smaller types (e.g. i64 -> i16).
// TODO: Implement something like https://jp.mathworks.com/help/gads/genetic-algorithm-options.html#f9147
//       For scaling (and possibly improving) fitness values.

fn crossover(list1: &Vec<i64>, list2: &Vec<i64>, cross_probability: f32) -> Vec<i64> {
  if list1.len() != list2.len() {
    panic!("Lists must have the same length");
  }

  let mut list: i64 = if math::probability(0.5) { 0 } else { 1 };
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
    if math::probability(cross_probability) {
      list = if list == 0 { 1 } else { 0 };
    }
  }

  result
}

fn mutate(list: &mut Vec<i64>) {
  if math::probability(0.9) { return; }
  for i in 0..list.len() {
    if math::probability(0.8) { continue; }
    let range = (i + 1) as i64 .. list.len() as i64;
    if range.is_empty() { continue; }
    let idx: i64 = rand::thread_rng().gen_range(range);
    let temp = list[idx as usize];
    list[idx as usize] = list[i as usize];
    list[i] = temp;
  }
}

fn make_offspring(list1: &Vec<i64>, list2: &Vec<i64>) -> Vec<i64> {
  let mut cross = crossover(list1, list2, 0.1);
  mutate(&mut cross);
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

  if (row + item.height) > rows { return false; }
  if (col + item.width) > cols { return false; }

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
        // Mark all cells as used.
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

  let wasted_space = count_zeros(&filled);
  (total_benefit, wasted_space)
}

fn generate_new_population(survived_solutions: &mut Vec<&Vec<i64>>) -> Vec<Vec<i64>>{
  let mut result = Vec::<Vec<i64>>::new();

  // Add elite first.
  for i in 0..survived_solutions.len(){
    if i >= survived_solutions.len() { break; }
    if i > 10 { break; }
    result.push(survived_solutions[i].to_vec());
  }

  while result.len() < 100 {
    for i in 0..survived_solutions.len() {
      if math::probability(0.01) {
        result.push(survived_solutions[i].to_vec());
      }

      if math::probability(0.2) {
        let range = (i + 1) as i64 .. survived_solutions.len() as i64;
        if range.is_empty() { continue; }
        let crossover_idx: i64 = rand::thread_rng().gen_range(range);
        let crossover_solution = survived_solutions[crossover_idx as usize];
        let offspring = make_offspring(survived_solutions[i], crossover_solution);
        result.push(offspring);
      }
    }
  }

  // TODO: Maybe try avoid mutating elite (i.e. first few items). And also append mutated elite.
  for sol in &mut result {
    mutate(sol);
  }

  result
}

pub struct GeneticAlgorithm {
  items: Vec<Item>,
  solutions: Vec::<Vec<i64>>,
  container: Container
}

impl GeneticAlgorithm {
  pub fn new(container: Container, items: &Vec<Item>) -> GeneticAlgorithm {
    let mut solutions = Vec::<Vec<i64>>::new();

    for _ in 0..100 {
      solutions.push(random_solution(items.len() as i64));
    }

    GeneticAlgorithm {
      items: items.to_vec(),
      solutions: solutions,
      container: container
    }
  }

  pub fn execute_population(&mut self, stats: &mut Stats) -> bool{ 
    let mut all_scores = Vec::<i64>::new();

    // Solutions that survived.
    let mut survived_solutions = Vec::<&Vec<i64>>::new();
    let mut survived_scores = Vec::<i64>::new();

    // List of solution and score (which is also a tuple).
    let mut tuples = Vec::<(&Vec<i64>, (i64, i64))>::new();

    // Eval many solutions.
    for solution in &self.solutions {
      let score = score(&self.container, &self.items, &solution);
      all_scores.push(score.0);
      tuples.push((&solution, score));
    }

    tuples.sort_by_key(|k| k.1);
    tuples.reverse();
    let gen_best_score: i64 = tuples[0].1.0;

    // New optimal found.
    if gen_best_score > stats.optimal_best_score {
      stats.optimal_best_score = gen_best_score;
      stats.optimal_hash = format!("{:?}", md5::compute(format!("{:?}", tuples[0].0)));
      stats.optimal_wasted = tuples[0].1.1;
      stats.optimal_found_gens.push(stats.total_generations);
      stats.store_optimal_solution(tuples[0].0);

      // For now, this program doesn't try to minimize wasted room.
      // So even if it's 0, that doesn't mean it's the optimal value.
      // if stats.optimal_wasted == 0 {
      //   return true;
      // }

      if stats.optimal_best_score == stats.max_possible_score {
        return true;
      }
    }

    let stddev: f64 = math::standard_deviation(&all_scores);

    for tuple in &tuples {
      // Ensure a minimum number of solutions.
      if survived_solutions.len() < 10 || tuple.1.0 > (stddev as i64) {
        survived_solutions.push(tuple.0);
        survived_scores.push(tuple.1.0);
      } else {
        break;
      }
    }

    self.solutions = generate_new_population(&mut survived_solutions);

    println!("Gen #{} | Best score: {} | Gen avg: {:.2} | Current optimal: {} | Optimal ID: {} | Wasted room: {}", stats.total_generations, gen_best_score, math::mean(&all_scores), stats.optimal_best_score, stats.optimal_hash, stats.optimal_wasted);

    stats.total_generations += 1;

    return false;
  }
}
