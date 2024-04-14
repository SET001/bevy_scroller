use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{
  Scroller, PoissonSpriteGenerator, ScrollerBundle, ScrollerPlugin, ScrollerSize,
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
    ScrollerSize {
      size: Vec2::new(window.width(), window.height()),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 5.,
        ..Default::default()
      },
      generator: PoissonSpriteGenerator {
        radius: 128. * 2.,
        sprites: (1..8).map(|i| format!("gems/{i}.png")).collect(),
        item_size: Vec2::new(window.width(), window.height()),
        sub_item_size: Vec2::splat(128.),
      },
      ..default()
    },
  ));
}
