#[derive(Copy, Clone)]
pub struct Container {
  pub width: i64,
  pub height: i64
}

impl Container {
  pub fn new(w: i64, h: i64) -> Container {
    Container {
      width: w,
      height: h
    }
  }
}

impl std::fmt::Debug for Container {
  fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(fmt, "Container {}x{}", self.width, self.height)
  }
}
