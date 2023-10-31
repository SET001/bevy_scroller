use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::ScrollerPlugin;

fn main() {
  let mut app = App::new();
  app
    .add_plugins((DefaultPlugins, ScrollerPlugin))
    .add_systems(Startup, start);
  app.run();
}

fn start(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>) {
  let primary_window = windows.get_single().expect("no primary window");
  commands.spawn(Camera2dBundle::default());
}
