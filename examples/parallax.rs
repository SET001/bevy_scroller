use std::collections::VecDeque;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{
  Scroller, ScrollerBundle, ScrollerDirection, ScrollerGenerator, ScrollerPlugin, ScrollerSize,
};

#[derive(Resource)]
pub struct ScrollerImages(Vec<Handle<Image>>);

fn main() {
  let mut app = App::new();
  app
    .add_plugins((DefaultPlugins, ScrollerPlugin))
    .add_systems(Startup, start);
  #[cfg(feature = "dev")]
  {
    use bevy_editor_pls::EditorPlugin;
    app.add_plugins(EditorPlugin::default());
  }
  app.run();
}

fn start(
  mut commands: Commands,
  windows: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  let item_height = 1980_f32;
  let direction = ScrollerDirection::Backward;
  let primary_window = windows.get_single().expect("no primary window");
  commands.spawn(Camera2dBundle::default());

  let images = (0..=5)
    .map(|i| format!("parallax/{i}.png"))
    .collect::<VecDeque<String>>();
  let images_handles = images
    .iter()
    .map(|image_path| asset_server.load(image_path))
    .collect::<Vec<Handle<Image>>>();
  commands.insert_resource(ScrollerImages(images_handles));

  commands.spawn(SpriteBundle {
    texture: asset_server.load(images.get(0).unwrap()),
    ..default()
  });
  (1..=5).for_each(|i| {
    let image_handle = images
      .get(i)
      .expect(&format!("no image with index {i}"))
      .into();
    commands.spawn((
      ScrollerGenerator::SpriteSingle(image_handle),
      ScrollerSize {
        size: Vec2::new(primary_window.width(), item_height),
      },
      ScrollerBundle {
        scroller: Scroller {
          speed: 0.5 + i as f32 * 0.5,
          direction: direction.clone(),
          ..default()
        },
        ..default()
      },
      Transform::from_translation(Vec3::new(0., 0., 1. + i as f32)),
    ));
  });
}
