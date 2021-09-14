use super::item::Item;

pub struct Stats {
  pub max_possible_score: i64,
  pub total_generations: i64,
  pub optimal_best_score: i64,
  pub optimal_hash: String,
  pub optimal_wasted: i64,
  pub optimal_found_gens: Vec<i64>
}

impl Stats {
  pub fn new(items: &Vec<Item>) -> Stats {
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
