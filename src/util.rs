extern crate num_traits;
use num_traits::int::PrimInt;
use super::types::item::Item;

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

pub fn first_empty_space(filled: &Vec<Vec<i64>>, item: &Item) -> Option<(i64, i64)> {
  for i in 0..filled.len() {
    for j in 0..filled[i].len() {
      if item_fits(filled, item, i as i64, j as i64) {
        return Some((i as i64, j as i64));
      }
    }
  }

  None
}
