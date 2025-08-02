mod shared;

use bevy::prelude::*;
use bevy_scroller::{Scroller, ScrollerBundle, ScrollerSize, SingleSpriteGenerator};
use shared::*;

fn main() {
  get_app("single".into()).add_systems(Startup, start).run();
}

pub fn start(mut commands: Commands) {
  let sprite_size = Vec2::new(300., 300.);

  commands.spawn(Camera2d);

  commands.spawn((
    ScrollerSize {
      size: Vec2::new(600., 400.),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 5.,
        ..default()
      },
      generator: SingleSpriteGenerator {
        path: "bevy_logo.png".into(),
        size: sprite_size,
      },
      ..default()
    },
  ));
}
