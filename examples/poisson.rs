mod shared;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{
  PoissonSpriteGenerator, Scroller, ScrollerBundle, ScrollerPlugin, ScrollerSize,
};
use shared::get_app;

fn main() {
  get_app("poisson".into()).add_systems(Startup, start).run();
}

fn start(mut commands: Commands, primary_window: Query<&Window, With<PrimaryWindow>>) {
  let window = primary_window.get_single().expect("no primary window");

  commands.spawn(Camera2dBundle::default());

  commands.spawn((
    ScrollerSize {
      size: Vec2::new(window.width(), window.height()),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 5.,
        ..Default::default()
      },
      generator: PoissonSpriteGenerator {
        radius: 128. * 2.,
        sprites: (1..8).map(|i| format!("gems/{i}.png")).collect(),
        item_size: Vec2::new(window.width(), window.height()),
        sub_item_size: Vec2::splat(128.),
      },
      ..default()
    },
  ));
}
