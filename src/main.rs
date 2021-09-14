mod item;
mod container;
mod math;

use item::Item;
use container::Container;
extern crate clap;
extern crate text_io;
use clap::{Arg, App, SubCommand};
use text_io::read;

use std::io::Read;
use md5;
use rand::Rng;
use std::collections::HashSet;
use rand::seq::SliceRandom;

// TODO: Change some types to smaller types (e.g. i64 -> i16).

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
  //let init = format!("{:?}", list);
  for i in 0..list.len() {
    if math::probability(0.8) { continue; }
      let range = (i + 1) as i64 .. list.len() as i64;
      if range.is_empty() { continue; }
      //println!("mutating..");
      let idx: i64 = rand::thread_rng().gen_range(range);
      let temp = list[idx as usize];
      list[idx as usize] = list[i as usize];
      list[i] = temp;
    //}
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
        result.push(offspring); // TODO: This line was commented for a long time. Is it OK to add it back?
      }
    }
  }

  // TODO: Maybe try avoid mutating elite (i.e. first few items). And also append mutated elite.
  for sol in &mut result {
    mutate(sol);
  }

  result
}

fn execute_algorithm(container: &Container, items: &Vec<Item>, stats: &mut Stats){
  // List of solutions.
  let mut solutions = Vec::<Vec<i64>>::new();

  // Create initial solution set.
  for _ in 0..100 {
    solutions.push(random_solution(items.len() as i64));
  }

  // For each population.
  loop {   
    let mut all_scores = Vec::<i64>::new();

    // Solutions that survived.
    let mut survived_solutions = Vec::<&Vec<i64>>::new();
    let mut survived_scores = Vec::<i64>::new();

    // List of solution and score (which is also a tuple).
    let mut tuples = Vec::<(&Vec<i64>, (i64, i64))>::new();

    // Eval many solutions.
    for solution in &solutions {
      let score = score(&container, &items, &solution);
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

    let new_population = generate_new_population(&mut survived_solutions);
    solutions = new_population;

    // TODO: Is it possible to make it safe?
    unsafe {
      if STOPPED_EXECUTION {
        print_stats(stats);
        break;
      } else {
        println!("Gen #{} | Best score: {} | Gen avg: {:.2} | Current optimal: {} | Optimal ID: {} | Wasted room: {}", stats.total_generations, gen_best_score, math::mean(&all_scores), stats.optimal_best_score, stats.optimal_hash, stats.optimal_wasted);
      }
    }

    stats.total_generations += 1;
  }
}

fn print_stats(stats: &Stats){
  println!("Max score assuming infinite container: {}", stats.max_possible_score);
  println!("Total generations: {}", stats.total_generations);
  println!("Current optimal: {}", stats.optimal_best_score);
  println!("Optimal ID: {}", stats.optimal_hash);
  println!("Wasted room: {}", stats.optimal_wasted);
  println!("Generations where local optimals were found: {:?}", stats.optimal_found_gens);
}

struct Stats {
  max_possible_score: i64,
  total_generations: i64,
  optimal_best_score: i64,
  optimal_hash: String,
  optimal_wasted: i64,
  optimal_found_gens: Vec<i64>
}

impl Stats {
  fn new(items: &Vec<Item>) -> Stats {
    let mut max_possible_score: i64 = 0;
    for item in items {
      max_possible_score += item.benefit;
    }

    Stats {
      max_possible_score: max_possible_score,
      total_generations: 0,
      optimal_best_score: 0,
      optimal_hash: String::new(),
      optimal_wasted: 0,
      optimal_found_gens: Vec::<i64>::new()
    }
  }
}

static mut STOPPED_EXECUTION: bool = false;

fn ensure_positive(n: i64){
  if n < 1 {
    panic!("Values must be greater than 0");
  }
}

fn random_scenario(container_square_size: i64, item_count: i64, item_max_side: i64, item_max_benefit: i64) -> (Container, Vec<Item>) {
  ensure_positive(container_square_size);
  ensure_positive(item_count);
  ensure_positive(item_max_side);
  ensure_positive(item_max_benefit);

  let container: Container = Container::new(container_square_size, container_square_size);
  let mut items: Vec::<Item> = Vec::<Item>::new();

  for _ in 0..item_count {
    items.push(Item::make_random(item_max_side, item_max_side, item_max_benefit));
  }

  (container, items)
}

fn file_scenario(filename: String) -> (Container, Vec<Item>) {
  let mut file = std::fs::File::open(filename).unwrap().bytes().map(|ch| ch.unwrap());
  let container_width: i64 = read!("{}", file);
  let container_height: i64 = read!("{}", file);
  let item_count: i64 = read!("{}", file);

  let container = Container::new(container_width, container_height);

  let mut items: Vec<Item> = Vec::<Item>::new();

  println!("------ Data from file ------");
  println!("{:?}", container);

  for _ in 0..item_count {
    let w: i64 = read!("{}", file);
    let h: i64 = read!("{}", file);
    let b: i64 = read!("{}", file);
    let item = Item::new(w, h, b);
    println!("{:?}", &item);
    items.push(item);
  }

  // TODO: How to close file?
  println!("------ File read finished ------");
  (container, items)
}

// TODO: Should be <T>.
fn parse(s: Option<&str>) -> i64 {
  match s {
    Some(string) => {
      match string.parse::<i64>() {
        Ok(n) => n,
        Err(_e) => {
          panic!("Option value is not a number")
        }
      }
    },
    None => {
      // Value is required. This case never happens.
      0
    }
  }
}

static PROGRAM_DESCRIPTION: &str = "Genetic algorithm for knapsack 2D rectangle allocation";

fn file_subcommand() -> App<'static, 'static> {
  SubCommand::with_name("file") 
    .arg(Arg::with_name("file input")
    .long("file-input")
    .value_name("FILE")
    .takes_value(true)
    .required(true))
}

fn random_subcommand() -> App<'static, 'static> {
  SubCommand::with_name("random") 
    .arg(Arg::with_name("item count")
      .long("item-count")
      .value_name("ITEM_COUNT")
      .takes_value(true)
      .required(true))
    .arg(Arg::with_name("container square side")
      .long("container-square-side")
      .value_name("CONTAINER_SQUARE_SIDE")
      .takes_value(true)
      .required(true))
    .arg(Arg::with_name("item max square side")
      .long("item-max-square-side")
      .value_name("ITEM_MAX_SQUARE_SIDE")
      .takes_value(true).required(true))
    .arg(Arg::with_name("max benefit")
      .long("max-benefit")
      .value_name("MAX_BENEFIT")
      .takes_value(true)
      .required(true))
}

fn build_scenario_from_opts() -> (Container, Vec<Item>) {
  let matches = App::new(PROGRAM_DESCRIPTION).subcommand(file_subcommand())
                                             .subcommand(random_subcommand())
                                             .get_matches();

  match matches.subcommand() {
    ("random", Some(matches)) => {
      random_scenario(
        parse(matches.value_of("container square side")),
        parse(matches.value_of("item count")),
        parse(matches.value_of("item max square side")),
        parse(matches.value_of("max benefit"))
      )
    },
    ("file", Some(matches)) => {
      let file_name = match matches.value_of("file input") {
        Some(string) => string,
        None => { panic!() }
      };
      file_scenario(file_name.to_string())
    },
    _ => {
      panic!();
    }
  }
}

fn main() {
  let (container, items) = build_scenario_from_opts();
  let mut stats = Stats::new(&items);

  println!("Items: {}", items.len());
  println!("Max score assuming infinite container: {}", stats.max_possible_score);

  ctrlc::set_handler(|| {
    unsafe {
      if !STOPPED_EXECUTION {
        println!();
        println!("Stopping execution...");
        println!();
      }
      STOPPED_EXECUTION = true;
    }
  })
  .expect("Error setting Ctrl-C handler");

  execute_algorithm(&container, &items, &mut stats);
}
