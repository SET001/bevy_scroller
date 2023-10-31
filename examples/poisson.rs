use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*, window::PrimaryWindow};
use common::AppConfig;
use scroller::{scroller_update, PoissonSpriteSpawner, Scroller, ScrollerBundle, ScrollerPlugin};

fn main() {
  let mut app = App::new();
  app
    .init_resource::<AppConfig>()
    .add_plugins(DefaultPlugins.set(AssetPlugin {
      watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
      asset_folder: "../../assets".to_string(),
    }))
    .add_plugins(ScrollerPlugin)
    .add_systems(Startup, start)
    .add_systems(Update, scroller_update);
  #[cfg(feature = "dev")]
  {
    use bevy_editor_pls::EditorPlugin;
    app.add_plugin(EditorPlugin::default());
  }
  app.run();
}

fn start(mut commands: Commands, primary_window: Query<&Window, With<PrimaryWindow>>) {
  let window = primary_window.get_single().expect("no primary window");

  commands.spawn(Camera2dBundle::default());

  commands.spawn((
    ScrollerBundle {
      name: Name::new("space rocks scroller"),
      scroller: Scroller {
        speed: 5.0,
        // bounds: Some(Bounds {
        //   start: window.width() / 2.,
        //   end: -window.width() / 2.,
        // }),
        ..Default::default()
      },
      spatial_bundle: SpatialBundle {
        transform: Transform::from_translation(Vec2::new(10., 10.).extend(0.)),
        ..Default::default()
      },
      ..default()
    },
    PoissonSpriteSpawner {
      sprites: (1..8)
        .map(|e| format!("images/levels/red_star/stone{}.png", e))
        .collect(),
      radius: 300.,
      ..default()
    },
  ));
}
