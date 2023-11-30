use std::collections::VecDeque;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{
  Scroller, ScrollerBundle, ScrollerPlugin, ScrollerSize, SequenceSpriteSheetGenerator,
};
fn main() {
  let mut app = App::new();
  app
    .add_plugins((DefaultPlugins, ScrollerPlugin))
    .add_systems(Startup, startup);
  #[cfg(feature = "dev")]
  {
    use bevy_editor_pls::EditorPlugin;
    app.add_plugins(EditorPlugin::default());
  }
  app.run();
}

fn startup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  windows: Query<&Window, With<PrimaryWindow>>,
) {
  let primary_window = windows.get_single().expect("no primary window");
  commands.spawn(Camera2dBundle::default());

  let texture_handle = asset_server.load("sprite_sheet.png");
  let sprite_size = Vec2::new(64., 64.);
  let texture_atlas = TextureAtlas::from_grid(texture_handle, sprite_size, 10, 10, None, None);
  let texture_atlas_handle = texture_atlases.add(texture_atlas);

  commands.spawn((
    ScrollerSize {
      size: Vec2::new(primary_window.width(), sprite_size.y * 2.),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 1.,
        render_layer: Some(1),
        ..default()
      },
      generator: SequenceSpriteSheetGenerator {
        sprites: VecDeque::from_iter(0..100),
        texture_atlas: texture_atlas_handle,
      },
      ..default()
    },
  ));
}
