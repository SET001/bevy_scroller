use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{Scroller, ScrollerBundle, ScrollerGenerator, ScrollerPlugin, ScrollerSize};

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

pub fn start(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
  let window = windows.get_single().expect("no primary window");
  let sprite_size = Vec2::new(128., 128.);

  commands.spawn(Camera2dBundle::default());

  commands.spawn((
    ScrollerGenerator::SpriteSingle("gems/1.png".into()),
    ScrollerSize {
      size: Vec2::new(window.width(), sprite_size.y),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 0.5,
        ..default()
      },
      ..default()
    },
    Transform::from_translation(Vec3::new(0., (sprite_size.y - window.height()) / 2., 0.)),
  ));

  commands.spawn((
    ScrollerGenerator::SpriteSingle("gems/2.png".into()),
    ScrollerSize {
      size: Vec2::new(window.width(), sprite_size.y),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 0.5,
        ..default()
      },
      ..default()
    },
    Transform {
      translation: Vec3::new(0., (window.height() - sprite_size.y) / 2., 0.),
      rotation: Quat::from_rotation_z(PI),
      ..default()
    },
  ));
}
