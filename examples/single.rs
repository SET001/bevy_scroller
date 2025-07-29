mod shared;

use bevy::prelude::*;
use bevy_scroller::{
  Direction, FillMode, Scroller, ScrollerBundle, ScrollerGenerators, ScrollerSpawner,
  SingleSpriteGenerator,
};
use shared::*;

fn main() {
  get_app("single".into()).add_systems(Startup, start).run();
}

pub fn start(
  mut commands: Commands,
  spawners: Res<ScrollerGenerators>,
  asset_server: Res<AssetServer>,
) {
  let sprite_size = Vec2::new(300., 300.);
  commands.spawn(Camera2dBundle::default());
  commands
    .spawn(ScrollerBundle {
      scroller: Scroller::new(5.),
      size: Vec2::new(1000., 300.).into(),
      generator: SingleSpriteGenerator {
        texture: asset_server.load("bevy_logo.png"),
        size: sprite_size,
      },
      direction: Direction::default(),
      fill_mode: FillMode::default(),
      spatial: SpatialBundle::default(),
    })
    .with_children(|parent| {
      parent.spawn(());
    });
}
