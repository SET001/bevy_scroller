mod shared;

use bevy::{ecs::system::RunSystemOnce, prelude::*, window::PrimaryWindow};
use bevy_scroller::*;
use shared::get_app;
use std::f32::consts::PI;

fn main() {
  get_app("mirrors".into()).add_systems(Startup, start).run();
}

pub fn start(world: &mut World) {
  let mirrors = world.register_system(spawn_mirrors);
  let scroller_entity = world.run_system_once(spawn_scroller);
  world
    .entity_mut(scroller_entity)
    .insert(OnScrollerInit(mirrors));
}

pub fn spawn_scroller(
  mut commands: Commands,
  windows: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) -> Entity {
  let window = windows.get_single().expect("no primary window");
  let sprite_size = Vec2::new(128., 128.);
  commands.spawn(Camera2dBundle::default());

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

  commands
    .spawn((
      ScrollerSize {
        size: Vec2::new(window.width(), sprite_size.y),
      },
      ScrollerBundle {
        scroller: Scroller {
          speed: 5.,
          render_layer: Some(1),
          ..default()
        },
        generator: RandomSequenceSpriteGenerator { items },
        spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
          0.,
          (sprite_size.y - window.height()) / 2.,
          10.,
        ))),
        ..default()
      },
    ))
    .id()
}

fn spawn_mirrors(
  In(scroller_entity): In<Entity>,
  mut commands: Commands,
  windows: Query<&Window, With<PrimaryWindow>>,
  q_scroller: Query<&Scroller>,
) {
  let scroller = q_scroller.get(scroller_entity).unwrap();
  let window = windows.get_single().expect("no primary window");
  let sprite_size = Vec2::new(128., 128.);
  commands.spawn((
    SpriteBundle {
      texture: scroller.texture_handle.clone(),
      transform: Transform {
        translation: Vec3::new(0., (window.height() - sprite_size.y) / 2., 10.),
        rotation: Quat::from_rotation_y(PI) * Quat::from_rotation_z(PI),
        ..default()
      },
      ..default()
    },
    Name::new("Scroller mirror top"),
  ));

  commands.spawn((
    SpriteBundle {
      texture: scroller.texture_handle.clone(),
      transform: Transform {
        translation: Vec3::new((window.width() - sprite_size.x) / 2., 0., 00.),
        rotation: Quat::from_rotation_z(PI / 2.),
        ..default()
      },
      ..default()
    },
    Name::new("Scroller mirror right"),
  ));

  commands.spawn((
    SpriteBundle {
      texture: scroller.texture_handle.clone(),
      transform: Transform {
        translation: Vec3::new((sprite_size.x - window.width()) / 2., 0., 0.),
        rotation: Quat::from_rotation_y(PI) * Quat::from_rotation_z(PI / 2.),
        ..default()
      },
      ..default()
    },
    Name::new("Scroller mirror left"),
  ));
}
