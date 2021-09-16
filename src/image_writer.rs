extern crate image;
use super::util;
use super::types::item::Item;
use super::types::container::Container;
use image::RgbImage;

static TILE_SIZE: i64 = 10;

fn draw_solution(img: &mut RgbImage, container: &Container, items: &[Item], solution: &Vec<i64>) {
  let mut filled = vec![vec![0; container.width as usize]; container.height as usize];

  // Place every item until there's no room left.
  for i in 0..solution.len() {
    let item_idx = solution[i] as usize;
    let item = &items[item_idx];
    let tuple = util::first_empty_space(&filled, item);
    match tuple {
      Some(values) => {
        let (row, col) = values;

        for i in row..(row + item.height) {
          for j in col..(col + item.width) {
            filled[i as usize][j as usize] = 1;
          }
        }

        draw_background(img, col, row, item, (193, 101, 10));

        // Draw four sides.
        draw_horizontal_line(img, col, row, item.width, (255, 255, 255));
        draw_horizontal_line(img, col, row + item.height, item.width, (255, 255, 255));
        draw_vertical_line(img, col, row, item.height, (255, 255, 255));
        draw_vertical_line(img, col + item.width, row, item.height, (255, 255, 255));
      },
      None => {}
    }
  }
}

fn draw_background(img: &mut RgbImage, x: i64, y: i64, item: &Item, rgb: (u8, u8, u8)) {
  for w in 0..(item.width * TILE_SIZE) {
    for h in 0..(item.height * TILE_SIZE) {
      let pixel_x = (x * TILE_SIZE) + w;
      let pixel_y = (y * TILE_SIZE) + h;
      img.get_pixel_mut(pixel_x as u32, pixel_y as u32).data = [rgb.0, rgb.1, rgb.2];
    }
  }
}

fn draw_horizontal_line(img: &mut RgbImage, x: i64, y: i64, length: i64, rgb: (u8, u8, u8)) {
  for i in 0..(length * TILE_SIZE) + 1 {
    let pixel_x = (x * TILE_SIZE) + i;
    let pixel_y = y * TILE_SIZE;
    img.get_pixel_mut(pixel_x as u32, pixel_y as u32).data = [rgb.0, rgb.1, rgb.2];
  }
}

fn draw_vertical_line(img: &mut RgbImage, x: i64, y: i64, length: i64, rgb: (u8, u8, u8)) {
  for i in 0..(length * TILE_SIZE) + 1 {
    let pixel_x = x * TILE_SIZE;
    let pixel_y = (y * TILE_SIZE) + i;
    img.get_pixel_mut(pixel_x as u32, pixel_y as u32).data = [rgb.0, rgb.1, rgb.2];
  }
}

pub fn create_image(file_name: String, container: &Container, items: &Vec<Item>, solution: &Vec<i64>) {
  let container_width: i64 = container.width;
  let container_height: i64 = container.height;

  let mut img = RgbImage::new((container_width * TILE_SIZE) as u32 + 1, (container_height * TILE_SIZE) as u32 + 1);

  for i in 0..container.width + 1 {
    draw_vertical_line(&mut img, i, 0, container_height, (50, 50, 50));
  }

  for i in 0..container.height + 1 {
    draw_horizontal_line(&mut img, 0, i, container_width, (50, 50, 50));
  }

  draw_solution(&mut img, container, items, solution);

  img.save(file_name).unwrap();
}
