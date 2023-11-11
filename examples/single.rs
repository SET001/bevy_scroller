use bevy::{prelude::*, window::PrimaryWindow};
use bevy_scroller::{
  Scroller, ScrollerBundle, ScrollerDirection, ScrollerPlugin, ScrollerSize, SingleSpriteGenerator,
};

#[derive(Resource, Default)]
pub struct ScrollerImage(Handle<Image>);
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

pub fn start(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  windows: Query<&Window, With<PrimaryWindow>>,
) {
  let primary_window = windows.get_single().expect("no primary window");

  let image_path = "bevy_logo.png";
  commands.insert_resource(ScrollerImage(asset_server.load(image_path)));

  commands.spawn(Camera2dBundle::default());

  commands.spawn((
    ScrollerSize {
      size: Vec2::new(primary_window.width(), 300.),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 1.,
        direction: ScrollerDirection::Forward,
        ..default()
      },
      generator: SingleSpriteGenerator {
        path: image_path.into(),
        size: Vec2 { x: 300., y: 300. },
      },
      ..default()
    },
  ));
}
