use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{
  Scroller, ScrollerBundle, ScrollerPlugin, ScrollerSize, SingleSpriteGenerator,
};

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
  let primary_window = windows.get_single().expect("no primary window");

  commands.spawn(Camera2dBundle::default());

  commands.spawn((
    ScrollerSize {
      size: Vec2::new(primary_window.width(), 300.),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 1.,
        render_layer: Some(1),
        ..default()
      },
      generator: SingleSpriteGenerator {
        path: "bevy_logo.png".into(),
        size: Vec2 { x: 300., y: 300. },
      },
      ..default()
    },
  ));
}
