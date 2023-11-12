use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::*;

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

  let items = (1..=7)
    .map(|i| {
      let path = format!("gems/{i}.png");
      let _: Handle<Image> = asset_server.load(path.clone());
      SpriteScrollerItem {
        path,
        size: Vec2 { x: 128., y: 128. },
      }
    })
    .collect::<Vec<SpriteScrollerItem>>();

  commands.spawn((
    ScrollerSize {
      size: Vec2::new(primary_window.width(), 128.),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 1.,
        ..default()
      },
      generator: RandomSequenceSpriteGenerator { items },
      ..default()
    },
  ));
}
