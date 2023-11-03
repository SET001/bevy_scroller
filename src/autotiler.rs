use std::collections::HashMap;

use bevy::prelude::{UVec2, Vec2};

pub trait BitMask {
  fn init(size: UVec2) -> Self;
  #[must_use]
  fn get_in_direction_from(&self, from: UVec2, direction: Vec2) -> bool;
  #[must_use]
  fn get_bitmask(&self, position: UVec2) -> Result<i32, String>;

  fn get_top_from(&self, position: UVec2) -> i32;
  fn get_left_from(&self, position: UVec2) -> i32;
  fn get_right_from(&self, position: UVec2) -> i32;
  fn get_bottom_from(&self, position: UVec2) -> i32;
}

impl BitMask for HashMap<(u32, u32), bool> {
  fn init(size: UVec2) -> Self {
    Self::from_iter(
      (0..size.x)
        .map(|x| (0..size.y).map(move |y| ((x, y), false)))
        .flatten()
        .collect::<Vec<((u32, u32), bool)>>(),
    )
  }

  fn get_in_direction_from(&self, from: UVec2, direction: Vec2) -> bool {
    let key = from.as_vec2() + direction;
    match self.get(&(key.x as u32, key.y as u32)) {
      Some(value) => *value,
      None => false,
    }
  }

  fn get_top_from(&self, position: UVec2) -> i32 {
    self.get_in_direction_from(position, Vec2 { x: 0., y: 1. }) as i32
  }

  fn get_left_from(&self, position: UVec2) -> i32 {
    self.get_in_direction_from(position, Vec2 { x: -1., y: 0. }) as i32
  }

  fn get_right_from(&self, position: UVec2) -> i32 {
    self.get_in_direction_from(position, Vec2 { x: 1., y: 0. }) as i32
  }

  fn get_bottom_from(&self, position: UVec2) -> i32 {
    self.get_in_direction_from(position, Vec2 { x: 0., y: -1. }) as i32
  }

  fn get_bitmask(&self, position: UVec2) -> Result<i32, String> {
    self
      .get(&(position.x, position.y))
      .and_then(|_| {
        let ground_index = self.get_top_from(position)
          + 2 * self.get_left_from(position)
          + 4 * self.get_right_from(position)
          + 8 * self.get_bottom_from(position);
        Some(ground_index)
      })
      .ok_or("asdasd".into())
  }

  // fn get_left(x: u32, y: u32) -> bool {
  //   false
  // }
}

// impl BitMask for Vec<Vec<bool>> {
//   fn get_in_direction_from(&self, from: UVec2, direction: UVec2) -> bool {
//     false
//   }
// }
