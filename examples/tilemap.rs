use bevy::{prelude::*, window::PrimaryWindow};
use scroller::{Scroller, ScrollerBundle, ScrollerInitialized, ScrollerPlugin, ScrollerSize};

fn main() {
  let mut app = App::new();
  app
    .add_plugins((
      DefaultPlugins.set(AssetPlugin {
        asset_folder: "../../assets".to_string(),
        ..default()
      }),
      ScrollerPlugin,
    ))
    .add_systems(Startup, start)
    .add_systems(Update, tilemap_generator);
  #[cfg(feature = "dev")]
  {
    use bevy_editor_pls::EditorPlugin;
    app.add_plugins(EditorPlugin::default());
  }
  app.run();
}

#[derive(Component)]
pub struct TilemapGenerator;

pub fn start(
  mut commands: Commands,
  windows: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  let window = windows.get_single().expect("no primary window");
  let sprite_size = Vec2::new(128., 128.);

  commands.spawn(Camera2dBundle::default());

  commands.spawn((
    TilemapGenerator,
    ScrollerSize {
      size: Vec2::new(window.width(), sprite_size.y),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 0.5,
        render_layer: Some(1),
        ..default()
      },
      spatial_bundle: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
        0.,
        (sprite_size.y - window.height()) / 2.,
        10.,
      ))),
      ..default()
    },
  ));
}

pub fn tilemap_generator(
  mut commands: Commands,
  q_scroller: Query<(&Scroller, &TilemapGenerator), With<ScrollerInitialized>>,
) {
  for (scroller, generator) in q_scroller.iter() {
    let tilemap_size = ;
    if scroller.new_item_needed(){
      commands.spawn((
        ScrollerItem {
          size: image.size(),
          parent,
        },
        SpriteBundle {
          texture: image_handle,
          visibility: Visibility::Hidden,
          ..Default::default()
        },
      ));   
    }
  }
}
