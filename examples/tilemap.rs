use std::collections::HashMap;

use bevy::{prelude::*, render::camera::Viewport, window::PrimaryWindow};
use bevy_ecs_tilemap::{
  prelude::{
    get_tilemap_center_transform, TilemapGridSize, TilemapId, TilemapSize, TilemapTexture,
    TilemapTileSize, TilemapType,
  },
  tiles::{TileBundle, TilePos, TileStorage, TileTextureIndex},
  TilemapBundle, TilemapPlugin,
};
use bevy_scroller::BitMask;
use bevy_scroller::{
  Scroller, ScrollerBundle, ScrollerDirection, ScrollerGenerator, ScrollerInitialized,
  ScrollerItem, ScrollerPlugin, ScrollerSize,
};
use rand::{thread_rng, Rng};
fn main() {
  let mut app = App::new();
  app
    .add_plugins((DefaultPlugins, ScrollerPlugin, TilemapPlugin))
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

const SCALE: f32 = 2.;

pub fn start(
  mut commands: Commands,
  windows: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  let window = windows.get_single().expect("no primary window");
  let viewport_width = 700_f32;
  let viewport_height = 192_f32;

  commands.spawn(Camera2dBundle {
    camera: Camera {
      viewport: Some(Viewport {
        physical_position: UVec2::new(
          (window.width() - viewport_width * SCALE) as u32 / 2,
          (window.height() - viewport_height * SCALE) as u32 / 2,
        ),
        physical_size: Vec2::new(viewport_width, viewport_height).as_uvec2() * SCALE as u32,
        ..default()
      }),
      ..default()
    },
    ..default()
  });

  commands
    .spawn((
      Name::new("Scene root"),
      SpatialBundle {
        transform: Transform::from_scale(Vec3::splat(SCALE)),
        ..Default::default()
      },
    ))
    .with_children(|parent| {
      parent.spawn((
        TilemapGenerator,
        ScrollerSize {
          size: Vec2::new(viewport_width, viewport_height),
        },
        ScrollerBundle {
          scroller: Scroller {
            speed: 0.5,
            direction: ScrollerDirection::Backward,
            ..default()
          },
          ..default()
        },
        Transform::from_xyz(0., 0., 100.),
      ));
      parent.spawn((
        ScrollerSize {
          size: Vec2::new(viewport_width, viewport_height),
        },
        ScrollerBundle {
          scroller: Scroller {
            speed: 0.4,
            direction: ScrollerDirection::Backward,
            ..default()
          },
          ..default()
        },
        ScrollerGenerator::SpriteSingle("bg3.png".into()),
        Transform::from_xyz(0., 0., 90.),
      ));

      parent.spawn((
        ScrollerSize {
          size: Vec2::new(viewport_width, viewport_height),
        },
        ScrollerBundle {
          scroller: Scroller {
            speed: 0.3,
            direction: ScrollerDirection::Backward,
            ..default()
          },
          ..default()
        },
        Transform::from_xyz(0., 0., 80.),
        ScrollerGenerator::SpriteSingle("bg2.png".into()),
      ));

      parent.spawn((
        ScrollerSize {
          size: Vec2::new(viewport_width, viewport_height),
        },
        ScrollerBundle {
          scroller: Scroller {
            speed: 0.2,
            direction: ScrollerDirection::Backward,
            ..default()
          },
          ..default()
        },
        ScrollerGenerator::SpriteSingle("bg1.png".into()),
        Transform::from_xyz(0., 0., 70.),
      ));
    });
}

pub fn tilemap_generator(
  mut commands: Commands,
  q_scroller: Query<(Entity, &Scroller, &TilemapGenerator)>,
  asset_server: Res<AssetServer>,
) {
  let mut rng = thread_rng();
  let texture_handle: Handle<Image> = asset_server.load("tileset.png");
  for (scroller_entity, scroller, generator) in q_scroller.iter() {
    let map_size = TilemapSize {
      x: rng.gen_range(4..20),
      y: 12,
    };

    // let map_size = TilemapSize { x: 20, y: 12 };

    if scroller.new_item_needed() {
      let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
      let grid_size = TilemapGridSize {
        x: tile_size.x,
        y: tile_size.y,
      };
      let map_type = TilemapType::default();

      commands
        .spawn((
          ScrollerItem {
            size: Vec2::new(
              map_size.x as f32 * tile_size.x,
              map_size.y as f32 * tile_size.y,
            ),
            parent: scroller_entity,
          },
          // Visibility::Hidden,
          SpatialBundle::default(),
        ))
        .with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
          let platform_height = rng.gen_range(1..map_size.y / 2);
          // let platform_height = 10;
          if platform_height > 0 {
            let mut tile_storage = TileStorage::empty(map_size);
            let transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0);
            // let tilemap_entity = parent.spawn_empty().id();
            // SpriteBundle {
            //   texture: texture_handle.clone(),
            //   visibility: Visibility::Hidden,
            //   ..Default::default()
            // },

            let mut ground_map: HashMap<(u32, u32), bool> = HashMap::init(UVec2 {
              x: map_size.x,
              y: map_size.y,
            });
            let platform_offset = rng.gen_range(1..4);
            let platform_length = map_size.x - platform_offset;
            println!(
              "platform_offset {platform_offset}, platform_length {platform_length}, map_size.x {}",
              map_size.x
            );
            for x in platform_offset..map_size.x {
              for y in (0..platform_height).rev() {
                ground_map.insert((x, y), true);
              }
            }

            let tilemap_entity = parent
              .spawn(TilemapBundle {
                grid_size,
                map_type,
                size: map_size,
                storage: tile_storage.clone(),
                transform,
                texture: TilemapTexture::Single(texture_handle.clone()),
                tile_size,
                ..Default::default()
              })
              .id();
            #[rustfmt::skip]
            let ground_tiles = vec![
              7, 7, 7, 18,
              7, 7, 7, 17,
              7, 7, 2, 10,
              0, 8, 1, 7
            ];

            for x in 0..map_size.x {
              for y in 0..map_size.y {
                if *ground_map.get(&(x, y)).unwrap() {
                  let bitmask = ground_map.get_bitmask(UVec2 { x, y }).unwrap();
                  let tile_index = ground_tiles[bitmask as usize];
                  // println!("tile_index {x}:{y}: {bitmask} {tile_index}");
                  let tile_pos = TilePos {
                    x,
                    // y: map_size.y - 1 - y,
                    y,
                  };
                  let tile_entity = parent
                    .spawn(TileBundle {
                      position: tile_pos,
                      texture_index: TileTextureIndex(tile_index),
                      tilemap_id: TilemapId(tilemap_entity),
                      ..Default::default()
                    })
                    .id();
                  tile_storage.set(&tile_pos, tile_entity);
                }
              }
            }
          }
        });
    }
  }
}
