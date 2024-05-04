use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{
  Scroller, ScrollerBundle, ScrollerPlugin, ScrollerSize, SingleSpriteGenerator,
};

fn main() {
  let mut app = App::new();
  app
    .add_plugins((DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        present_mode: bevy::window::PresentMode::AutoNoVsync,
        title: "BEVY_SCROLLER: multiple example".into(),
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

pub fn start(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
  let window = windows.get_single().expect("no primary window");
  let sprite_size = Vec2::new(128., 128.);

  commands.spawn(Camera2dBundle::default());

  commands.spawn((
    ScrollerSize {
      size: Vec2::new(window.width(), sprite_size.y),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 5.,
        ..default()
      },
      generator: SingleSpriteGenerator {
        path: "gems/1.png".into(),
        size: sprite_size,
      },
      spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
        0.,
        (sprite_size.y - window.height()) / 2.,
        0.,
      ))),
      ..default()
    },
  ));

  commands.spawn((
    ScrollerSize {
      size: Vec2::new(window.width(), sprite_size.y),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 5.,
        ..default()
      },
      generator: SingleSpriteGenerator {
        path: "gems/2.png".into(),
        size: sprite_size,
      },
      spatial: SpatialBundle::from_transform(Transform {
        translation: Vec3::new(0., (window.height() - sprite_size.y) / 2., 0.),
        rotation: Quat::from_rotation_z(PI),
        ..default()
      }),
      ..default()
    },
  ));
}
