use std::collections::VecDeque;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{
  Scroller, ScrollerBundle, ScrollerDirection, ScrollerPlugin, ScrollerSize, SingleSpriteGenerator,
};

#[derive(Resource)]
pub struct ScrollerImages(Vec<Handle<Image>>);

fn main() {
  let mut app = App::new();
  app
    .add_plugins((DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        present_mode: bevy::window::PresentMode::AutoNoVsync,
          ..default()
        }),
      ..default()
    }), ScrollerPlugin))
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
  let scroller_speed_min = 0.2;
  let scroller_speed_step = 0.2;
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
  let sizes = [
    Vec2::new(320., 240.),
    Vec2::new(128., 240.),
    Vec2::new(144., 240.),
    Vec2::new(160., 240.),
    Vec2::new(320., 240.),
    Vec2::new(240., 240.),
  ];

  sizes.into_iter().enumerate().for_each(|(i, size)| {
    commands.spawn((
      ScrollerSize {
        size: Vec2::new(primary_window.width(), item_height),
      },
      ScrollerBundle {
        scroller: Scroller {
          speed: scroller_speed_min + i as f32 * scroller_speed_step,
          direction: direction.clone(),
          render_layer: Some(1),
          ..default()
        },
        generator: SingleSpriteGenerator {
          path: format!("parallax/{i}.png"),
          size,
        },
        spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
          0.,
          0.,
          1. + i as f32,
        ))),
        ..default()
      },
    ));
  });
}
