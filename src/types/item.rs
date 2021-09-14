use rand::Rng;

#[derive(Copy, Clone)]
pub struct Item {
  pub width: i64,
  pub height: i64,
  pub benefit: i64
}

impl Item {
  pub fn new(w: i64, h: i64, b: i64) -> Item {
    Item {
      width: w,
      height: h,
      benefit: b
    }
  }

  pub fn make_random(w: i64, h: i64, benefit: i64) -> Item {
    let rand_w = rand::thread_rng().gen_range(1..w);
    let rand_h = rand::thread_rng().gen_range(1..h);
    let rand_benefit = rand::thread_rng().gen_range(0..benefit);
    Item::new(rand_w, rand_h, rand_benefit)
  }
}

impl std::fmt::Debug for Item {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(fmt, "Item {}x{} (benefit: {})", self.width, self.height, self.benefit)
  }
}
