use std::collections::HashMap;

use bevy::prelude::{UVec2, Vec2};
use rstest::rstest;

use bevy_scroller::BitMask;

#[test]
fn init() {
  let map = HashMap::init(UVec2 { x: 2, y: 2 });
  assert_eq!(
    map,
    HashMap::from([
      ((0, 0), false,),
      ((0, 1), false,),
      ((1, 0), false,),
      ((1, 1), false,),
    ])
  )
}

#[test]
fn get_in_direction_from() {
  let mut map = HashMap::<(u32, u32), bool>::init(UVec2 { x: 2, y: 2 });
  map.insert((0, 0), true);
  assert_eq!(
    map.get_in_direction_from(UVec2::new(0, 0), Vec2::new(0., 0.)),
    true
  );
}

#[rstest]
#[case(vec![
  ((0, 0), false), ((1, 0), false), ((2, 0), false),
  ((0, 1), false), ((1, 1), false), ((2, 1), false),
  ((0, 2), false), ((1, 2), false), ((2, 2), false),
  ], 0)]
#[case(vec![
  ((0, 0), false), ((1, 0), true), ((2, 0), false),
  ((0, 1), false), ((1, 1), false), ((2, 1), false),
  ((0, 2), false), ((1, 2), false), ((2, 2), false),
  ], 8)]
#[case(vec![
  ((0, 0), false), ((1, 0), true), ((2, 0), false),
  ((0, 1), true), ((1, 1), false), ((2, 1), true),
  ((0, 2), false), ((1, 2), true), ((2, 2), false),
  ], 15)]
fn get_bitmask(#[case] map: Vec<((u32, u32), bool)>, #[case] res: i32) {
  let map = HashMap::from_iter(map);
  let value = map.get_bitmask(UVec2 { x: 1, y: 1 }).unwrap();
  assert_eq!(value, res);
}
