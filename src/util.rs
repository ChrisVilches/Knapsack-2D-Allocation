// TODO: Should be <T>.
pub fn parse(s: Option<&str>) -> i64 {
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

pub fn ensure_positive(n: i64){
  if n < 1 {
    panic!("Values must be greater than 0");
  }
}
