use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{Scroller, ScrollerBundle, ScrollerGenerator, ScrollerPlugin, ScrollerSize};

#[derive(Resource)]
pub struct ScrollerImages(Vec<Handle<Image>>);
fn main() {
  let mut app = App::new();
  app
    .add_plugins((DefaultPlugins, ScrollerPlugin))
    .add_systems(Startup, start);
  app.run();
}

pub fn start(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  windows: Query<&Window, With<PrimaryWindow>>,
) {
  commands.spawn(Camera2dBundle::default());
  let primary_window = windows.get_single().expect("no primary window");

  let images = (1..=7)
    .map(|i| format!("gems/{i}.png"))
    .collect::<Vec<String>>();
  let images_handles = images
    .iter()
    .map(|image_path| asset_server.load(image_path))
    .collect::<Vec<Handle<Image>>>();
  commands.insert_resource(ScrollerImages(images_handles));

  commands.spawn((
    ScrollerGenerator::SpriteRandomSequence(images),
    ScrollerSize {
      size: Vec2::new(primary_window.width(), 128.),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 1.,
        ..default()
      },
      ..default()
    },
  ));
}
