mod shared;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::*;
use shared::get_app;

fn main() {
  get_app("BEVY_SCROLLER: random sequence example".into())
    .add_systems(Startup, start)
    .run();
}

pub fn start(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
  commands.spawn(Camera2dBundle::default());
  let primary_window = windows.get_single().expect("no primary window");

  let items = (1..=7)
    .map(|i| SpriteScrollerItem {
      path: format!("gems/{i}.png"),
      size: Vec2 { x: 128., y: 128. },
    })
    .collect::<Vec<SpriteScrollerItem>>();

  commands.spawn((
    ScrollerSize {
      size: Vec2::new(primary_window.width(), 128.),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 5.,
        render_layer: Some(1),
        ..default()
      },
      generator: RandomSequenceSpriteGenerator { items },
      ..default()
    },
  ));
}
