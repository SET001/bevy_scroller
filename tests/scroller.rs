// use bevy::{prelude::*, render::view::visibility};
// use common::AppConfig;
// use rstest::rstest;
// use scroller::{Scroller, ScrollerBundle, ScrollerItem, ScrollerPlugin};

// fn get_app() -> App {
//   let mut app = App::new();
//   app
//     .init_resource::<AppConfig>()
//     .add_plugins((MinimalPlugins, ScrollerPlugin));
//   app
// }

// #[test]
// fn scroller_add_items_test() {
//   let mut app = get_app();

//   let mut scroller = Scroller {
//     direction: Vec2::new(-1., 0.),
//     speed: 0.5,
//     rect: Rect::from_corners(Vec2::new(-100., 50.), Vec2::new(100., -50.)),
//     ..default()
//   };
//   for _ in 1..=10 {
//     let item = app.world.spawn(ScrollerItem { size: 100. }).id();
//     scroller.items_queue.push_back(item);
//   }

//   let scroller_entity = app
//     .world
//     .spawn((scroller, SpatialBundle::default()))
//     .with_children(|parent| {})
//     .id();

//   app.update();

//   let scroller = app
//     .world
//     .query::<&Scroller>()
//     .get(&app.world, scroller_entity)
//     .unwrap()
//     .clone();
//   println!("scroller.queued_items: {}", scroller.items_queue.len());
//   let mut items = app.world.query::<(&ScrollerItem, Option<&Visibility>)>();
//   let visible_items_count = items
//     .iter(&app.world)
//     .filter_map(|(i, v)| Some((i, v?)))
//     .count();
//   println!("scroller witdh: {:?}", scroller.rect.width());
//   println!("scroller Items: {:?}", items.iter(&app.world).len());
//   println!("scroller Items visible: {visible_items_count}",);
//   assert_eq!(visible_items_count, 30);
//   // println!("{:#?}", scroller);
//   // app.world.g
// }

// #[rstest]
// #[case(Vec2::new(-1., 0.), 0.5, Vec2::new(-0.5, 0.))]
// #[case(Vec2::new(-1., 0.), -0.5, Vec2::new(0.5, 0.))]
// #[case(Vec2::new(1., 0.), 0.5, Vec2::new(0.5, 0.))]
// #[case(Vec2::new(0., -1.), 0.5, Vec2::new(0., -0.5))]
// #[case(Vec2::new(0., 1.), 0.5, Vec2::new(0., 0.5))]
// #[case(Vec2::new(1., 1.), 0.5, Vec2::new(0.5, 0.5))]
// fn scroller_update_items_test(
//   #[case] direction: Vec2,
//   #[case] speed: f32,
//   #[case] expectation: Vec2,
// ) {
//   let mut app = get_app();
//   let scroller_entity = app
//     .world
//     .spawn((
//       Scroller {
//         direction,
//         speed,
//         rect: Rect::from_corners(Vec2::new(-100., 50.), Vec2::new(100., -50.)),
//         ..default()
//       },
//       SpatialBundle::default(),
//     ))
//     .id();

//   app.update();

//   let scroller_transform = app
//     .world
//     .query_filtered::<&Transform, With<Scroller>>()
//     .get(&app.world, scroller_entity)
//     .unwrap();

//   assert_eq!(scroller_transform.translation, expectation.extend(0.));
// }

// #[test]
// fn scroller_delete_items_test() {}
