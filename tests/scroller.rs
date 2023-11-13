use bevy::prelude::*;
use bevy_scroller::{
  GeneratedItem, ScrollerBundle, ScrollerGenerator, ScrollerItem, ScrollerPlugin, ScrollerSize,
};

#[derive(Component, Clone, Default)]
struct FooGenerator;
#[derive(Debug)]
struct FooItem;
impl GeneratedItem for FooItem {
  fn size(&self) -> Vec2 {
    Vec2::new(100., 100.)
  }
}
impl ScrollerGenerator for FooGenerator {
  type I = FooItem;

  fn gen_item(&mut self) -> Self::I {
    Self::I {}
  }
}

fn get_app() -> App {
  let mut app = App::new();
  app.add_plugins((
    MinimalPlugins,
    ScrollerPlugin,
    AssetPlugin::default(),
    ImagePlugin::default(),
  ));
  app
}

fn get_app_with_empty_scroller() -> (App, Entity) {
  let mut app = get_app();
  let scroller = app
    .world
    .spawn((
      ScrollerBundle::<FooGenerator>::default(),
      ScrollerSize {
        size: Vec2::new(1000., 100.),
      },
    ))
    .id();
  (app, scroller)
}

fn get_app_with_full_scroller() -> App {
  let (mut app, scroller) = get_app_with_empty_scroller();

  app.world.spawn((
    ScrollerItem {
      parent: scroller,
      size: Vec2::new(2000., 100.),
    },
    Transform::default(),
    Visibility::Hidden,
  ));
  app
}

mod init {
  use bevy::prelude::Name;
  use bevy_scroller::{ScrollerBundle, ScrollerSize};

  use crate::{get_app, FooGenerator};

  #[test]
  fn should_insert_name_component_if_it_does_not_exist() {
    let mut app = get_app();
    app.world.spawn((
      ScrollerBundle::<FooGenerator>::default(),
      ScrollerSize::default(),
    ));
    app.update();

    let name = app.world.query::<&Name>().get_single(&app.world).unwrap();
    assert_eq!(name.as_str(), "Scroller #1");
  }

  #[test]
  fn should_increment_unnamed_index() {
    let mut app = get_app();
    app.world.spawn((
      ScrollerBundle::<FooGenerator>::default(),
      ScrollerSize::default(),
    ));

    app.world.spawn((
      ScrollerBundle::<FooGenerator>::default(),
      ScrollerSize::default(),
    ));

    app.update();

    let names = app
      .world
      .query::<&Name>()
      .iter(&app.world)
      .map(|name| name.as_str())
      .collect::<Vec<&str>>();

    assert_eq!(names[1], "Scroller #2");
  }

  #[test]
  fn should_preserve_name_if_it_exist() {
    let mut app = get_app();
    let name = "some name";
    app.world.spawn((
      ScrollerBundle::<FooGenerator>::default(),
      Name::new(name),
      ScrollerSize::default(),
    ));
    app.update();

    let comp_name = app.world.query::<&Name>().get_single(&app.world).unwrap();
    assert_eq!(comp_name.as_str(), name);
  }
}

mod pre_generator {
  use crate::{get_app, get_app_with_full_scroller, FooGenerator};
  use bevy::prelude::*;
  use bevy_scroller::{ScrollerApp, ScrollerBundle, ScrollerSize, SpawnerInput};

  #[test]
  fn should_return_empty_vector_for_empty_world() {
    fn generator(In(input): In<SpawnerInput<FooGenerator>>) {
      assert_eq!(input.len(), 0);
    }
    let mut app = get_app();
    app.add_scroller_generator::<FooGenerator, _, _>(generator);
    app.update();
  }

  #[test]
  fn should_return_empty_vector_for_full_scroller() {
    fn generator(In(input): In<SpawnerInput<FooGenerator>>) {
      assert_eq!(input.len(), 0);
    }
    let mut app = get_app_with_full_scroller();
    app.add_scroller_generator::<FooGenerator, _, _>(generator);
    app.update();
  }

  #[test]
  fn should_return_vector_with_correct_size() {
    fn generator(In(input): In<SpawnerInput<FooGenerator>>) {
      assert_eq!(input.len(), 10);
    }
    let mut app = get_app();
    app.world.spawn((
      ScrollerBundle::<FooGenerator>::default(),
      ScrollerSize {
        size: Vec2::new(1000., 100.),
      },
    ));
    app.add_scroller_generator::<FooGenerator, _, _>(generator);
    app.update();
  }
}

mod update {
  use bevy::prelude::*;
  use bevy_scroller::{NeedInitialFilling, Scroller};

  use crate::{get_app_with_empty_scroller, get_app_with_full_scroller};

  // #[test]
  // fn should_not_update() {}
  // #[test]
  // fn should_update_transform() {}
  #[test]
  fn should_set_visible_after_initialization() {
    let mut app = get_app_with_full_scroller();

    app.update();

    let (_, visibility) = app
      .world
      .query::<(&Scroller, &Visibility)>()
      .get_single(&app.world)
      .unwrap();
    assert_eq!(visibility, Visibility::Inherited);
  }

  #[test]
  fn should_not_override_visible_on_initialised_scrollers() {
    let mut app = get_app_with_full_scroller();

    app.update();

    let (_, mut visibility) = app
      .world
      .query::<(&Scroller, &mut Visibility)>()
      .get_single_mut(&mut app.world)
      .unwrap();
    *visibility = Visibility::Hidden;

    app.update();

    let (_, visibility) = app
      .world
      .query::<(&Scroller, &Visibility)>()
      .get_single(&app.world)
      .unwrap();

    assert_eq!(visibility, Visibility::Hidden);
  }

  #[test]
  fn should_remove_init_marker_component_if_filled() {
    let mut app = get_app_with_full_scroller();
    app.update();

    let marker = app
      .world
      .query_filtered::<&NeedInitialFilling, With<Scroller>>()
      .get_single(&app.world);

    assert!(marker.is_err());
  }

  #[test]
  fn should_not_remove_init_marker_component_if_not_filled() {
    let (mut app, _) = get_app_with_empty_scroller();
    app.update();

    let marker = app
      .world
      .query_filtered::<&NeedInitialFilling, With<Scroller>>()
      .get_single(&app.world);

    assert!(marker.is_ok());
  }

  #[test]
  fn should_not_scroll_not_initialized_scrollers() {}

  #[test]
  fn should_scroller_initialized_scrollers() {}
}
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
