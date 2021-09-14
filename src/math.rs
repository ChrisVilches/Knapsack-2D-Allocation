use rand::Rng;

pub fn probability(n: f32) -> bool {
  rand::thread_rng().gen::<f32>() <= n
}

pub fn mean(scores: &Vec<i64>) -> f64 {
  let mut sum: f64 = 0.0;
  for i in 0..scores.len() {
    let n = scores[i] as f64;
    sum += n;
  }
  sum / (scores.len() as f64)
}

pub fn standard_deviation(scores: &Vec<i64>) -> f64 {
  let mut standard_deviation: f64 = 0.0;
  let mean: f64 = mean(scores);

  for i in 0..scores.len() {
    let n = scores[i] as f64;
    standard_deviation += (n - mean) * (n - mean);
  }

  return (standard_deviation / (scores.len() as f64)).sqrt();
}
