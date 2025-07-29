mod shared;

use bevy::prelude::*;
use bevy_scroller::{
  Direction, FillMode, Scroller, ScrollerBundle, ScrollerGenerators, ScrollerSpawner,
  SingleSpriteGenerator,
};
use shared::get_app;

fn main() {
  get_app("multiple".into()).add_systems(Startup, start).run();
}

pub fn start(
  mut commands: Commands,
  spawners: Res<ScrollerGenerators>,
  asset_server: Res<AssetServer>,
) {
  let sprite_size = Vec2::new(128., 128.);

  commands.spawn(Camera2dBundle::default());

  commands
    .spawn(ScrollerBundle {
      scroller: Scroller::new(5.),
      size: Vec2::new(1000., sprite_size.y).into(),
      generator: SingleSpriteGenerator {
        texture: asset_server.load("bevy_logo.png"),
        size: sprite_size,
      },
      spawner: ScrollerSpawner(*spawners.get::<SingleSpriteGenerator>().unwrap()),
      direction: Direction::default(),
      fill_mode: FillMode::default(),
      spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0., 200., 0.))),
    })
    .with_children(|parent| {
      parent.spawn(());
    });

  commands
    .spawn(ScrollerBundle {
      scroller: Scroller::new(5.),
      size: Vec2::new(1000., sprite_size.y).into(),
      generator: SingleSpriteGenerator {
        texture: asset_server.load("gems/2.png"),
        size: sprite_size,
      },
      spawner: ScrollerSpawner(*spawners.get::<SingleSpriteGenerator>().unwrap()),
      direction: Direction::default(),
      fill_mode: FillMode::default(),
      spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0., -200., 0.))),
    })
    .with_children(|parent| {
      parent.spawn(());
    });
}
