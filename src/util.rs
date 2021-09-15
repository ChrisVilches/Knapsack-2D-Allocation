extern crate num_traits;
use num_traits::int::PrimInt;

pub fn parse<N: PrimInt + std::str::FromStr>(s: Option<&str>) -> N {
  match s {
    Some(string) => {
      match N::from_str(string) {
        Ok(n) => n,
        Err(_e) => {
          panic!("Option value is not a number")
        }
      }
    },
    None => {
      // Value is required. This case never happens.
      panic!();
    }
  }
}

pub fn ensure_positive(n: i64){
  if n < 1 {
    panic!("Values must be greater than 0");
  }
}
