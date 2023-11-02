use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{
  poisson::PoissonScrollerGenerator, Scroller, ScrollerBundle, ScrollerPlugin, ScrollerSize,
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

fn start(mut commands: Commands, primary_window: Query<&Window, With<PrimaryWindow>>) {
  let window = primary_window.get_single().expect("no primary window");

  commands.spawn(Camera2dBundle::default());

  commands.spawn((
    PoissonScrollerGenerator {
      radius: 128. * 2.,
      sprites: (1..8).map(|i| format!("gems/{i}.png")).collect(),
      rect: Vec2::new(500., window.height()),
      item_width: 128.,
      ..default()
    },
    ScrollerSize {
      size: Vec2::new(window.width(), window.height()),
    },
    ScrollerBundle {
      name: Name::new("space rocks scroller"),
      scroller: Scroller {
        speed: 5.0,
        ..Default::default()
      },
      ..default()
    },
  ));
}
