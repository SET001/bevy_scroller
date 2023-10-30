use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{
  Scroller, ScrollerBundle, ScrollerGenerator, ScrollerInitialized, ScrollerPlugin, ScrollerSize,
};

#[derive(Resource)]
pub struct ScrollerImages(Vec<Handle<Image>>);

fn main() {
  let mut app = App::new();
  app
    .add_plugins((DefaultPlugins, ScrollerPlugin))
    .add_systems(Startup, start)
    .add_systems(Update, init_mirror);
  #[cfg(feature = "dev")]
  {
    use bevy_editor_pls::EditorPlugin;
    app.add_plugins(EditorPlugin::default());
  }
  app.run();
}

pub fn start(
  mut commands: Commands,
  windows: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  let window = windows.get_single().expect("no primary window");
  let sprite_size = Vec2::new(128., 128.);

  commands.spawn(Camera2dBundle::default());

  let images = (1..=7)
    .map(|i| format!("gems_{:03}.png", i))
    .collect::<Vec<String>>();
  let images_handles = images
    .iter()
    .map(|image_path| asset_server.load(image_path))
    .collect::<Vec<Handle<Image>>>();
  commands.insert_resource(ScrollerImages(images_handles));

  commands.spawn((
    ScrollerGenerator::SpriteRandomSequence(
      (1..=7)
        .map(|i| format!("gems_{:03}.png", i))
        .collect::<Vec<String>>(),
    ),
    ScrollerSize {
      size: Vec2::new(window.width(), sprite_size.y),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 0.5,
        render_layer: Some(1),
        ..default()
      },
      ..default()
    },
    Transform::from_translation(Vec3::new(0., (sprite_size.y - window.height()) / 2., 10.)),
  ));
}

fn init_mirror(
  mut commands: Commands,
  q_initialized: Query<&Scroller, Added<ScrollerInitialized>>,
  windows: Query<&Window, With<PrimaryWindow>>,
) {
  if let Ok(scroller) = q_initialized.get_single() {
    let window = windows.get_single().expect("no primary window");
    let sprite_size = Vec2::new(128., 128.);
    commands.spawn((
      SpriteBundle {
        texture: scroller.texture_handle.clone(),
        transform: Transform {
          translation: Vec3::new(0., (window.height() - sprite_size.y) / 2., 10.),
          rotation: Quat::from_rotation_y(PI) * Quat::from_rotation_z(PI),
          ..default()
        },
        ..default()
      },
      Name::new("Scroller mirror top"),
    ));

    commands.spawn((
      SpriteBundle {
        texture: scroller.texture_handle.clone(),
        transform: Transform {
          translation: Vec3::new((window.width() - sprite_size.x) / 2., 0., 00.),
          rotation: Quat::from_rotation_z(PI / 2.),
          ..default()
        },
        ..default()
      },
      Name::new("Scroller mirror right"),
    ));

    commands.spawn((
      SpriteBundle {
        texture: scroller.texture_handle.clone(),
        transform: Transform {
          translation: Vec3::new((sprite_size.x - window.width()) / 2., 0., 0.),
          rotation: Quat::from_rotation_y(PI) * Quat::from_rotation_z(PI / 2.),
          ..default()
        },
        ..default()
      },
      Name::new("Scroller mirror left"),
    ));
  };
}
