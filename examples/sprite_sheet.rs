mod shared;
use std::collections::VecDeque;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{Direction, Scroller, ScrollerBundle, SequenceSpriteSheetGenerator, Size};
use shared::get_app;
fn main() {
  get_app("sprite sheet".into())
    .add_systems(Startup, startup)
    .run();
}

fn startup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
  windows: Query<&Window, With<PrimaryWindow>>,
) {
  let primary_window = windows.get_single().expect("no primary window");
  commands.spawn(Camera2dBundle::default());

  let texture = asset_server.load("sprite_sheet.png");
  let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
    UVec2::splat(64),
    10,
    10,
    None,
    None,
  ));
  let sprite_size = Vec2::new(64., 64.);

  commands.spawn((
    Size(Vec2::new(primary_window.width(), sprite_size.y * 2.)),
    ScrollerBundle {
      size: Default::default(),

      scroller: Scroller {
        speed: 1.,
        ..default()
      },
      generator: SequenceSpriteSheetGenerator {
        sprites: VecDeque::from_iter(0..100),
        layout,
        texture,
      },
      ..default()
    },
  ));
}
