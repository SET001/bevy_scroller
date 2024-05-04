use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{
  Scroller, ScrollerBundle, ScrollerPlugin, ScrollerSize, SingleSpriteGenerator,
};

fn main() {
  let mut app = App::new();
  app
    .add_plugins((
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..default()
          }),
        ..default()
      })
      , ScrollerPlugin))
    .add_systems(Startup, start);
  #[cfg(feature = "dev")]
  {
    use bevy_editor_pls::EditorPlugin;
    app.add_plugins(EditorPlugin::default());
  }
  app.run();
}

pub fn start(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
  let primary_window = windows.get_single().expect("no primary window");
  let sprite_size = Vec2::new(300., 300.);

  commands.spawn(Camera2dBundle::default());

  commands.spawn((
    ScrollerSize {
      size: Vec2::new(primary_window.width(), sprite_size.y),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 5.,
        ..default()
      },
      generator: SingleSpriteGenerator {
        path: "bevy_logo.png".into(),
        size: sprite_size,
      },
      ..default()
    },
  ));
}
