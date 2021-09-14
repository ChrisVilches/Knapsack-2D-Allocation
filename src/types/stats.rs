use super::item::Item;

pub struct Stats {
  pub max_possible_score: i64,
  pub total_generations: i64,
  pub optimal_best_score: i64,
  pub optimal_hash: String,
  pub optimal_wasted: i64,
  pub optimal_found_gens: Vec<i64>,
  pub optimal_solution: Vec<i64>
}

impl Stats {
  pub fn new() -> Stats {
    Stats {
      max_possible_score: 0,
      total_generations: 0,
      optimal_best_score: 0,
      optimal_hash: String::new(),
      optimal_wasted: 0,
      optimal_found_gens: Vec::<i64>::new(),
      optimal_solution: Vec::<i64>::new()
    }
  }

  pub fn initialize(&mut self, items: &Vec<Item>) {
    let mut max_possible_score: i64 = 0;
    for item in items {
      max_possible_score += item.benefit;
    }

    self.max_possible_score = max_possible_score;
  }

  pub fn store_optimal_solution(&mut self, solution: &Vec<i64>) {
    self.optimal_solution = solution.to_vec();
  }

  pub fn print(&self){
    println!("Max score assuming infinite container: {}", self.max_possible_score);
    println!("Total generations: {}", self.total_generations);
    println!("Current optimal: {}", self.optimal_best_score);
    println!("Optimal ID: {}", self.optimal_hash);
    println!("Wasted room: {}", self.optimal_wasted);
    println!("Generations where local optimals were found: {:?}", self.optimal_found_gens);
    println!("Best solution found: {:?}", self.optimal_solution);
  }
}
