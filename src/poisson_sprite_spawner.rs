use bevy::prelude::*;
use common::AppConfig;
use fast_poisson::Poisson2D;
use rand::{thread_rng, Rng};

use crate::{Scroller, ScrollerItem};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PoissonSpriteSpawner {
  pub sprites: Vec<String>,
  pub radius: f64,
  pub z_index: f32,
}

pub fn poisson_sprite_spawner(
  mut commands: Commands,
  mut q_spawner: Query<(Entity, &mut Scroller, &PoissonSpriteSpawner)>,
  assets_server: Res<AssetServer>,
  config: Res<AppConfig>,
) {
  if !q_spawner.is_empty() {
    let mut rng = thread_rng();
    let resolution: Vec2 = config.window.resolution.into();

    for (entity, mut scroller, spawner) in q_spawner.iter_mut() {
      while scroller.items_queue.len() < 3 {
        let scroller_item = commands
          .spawn((
            Name::new("scroller item"),
            // ScrollerItem { size: resolution.x },
            SpatialBundle {
              visibility: Visibility::Hidden,
              ..default()
            },
          ))
          .with_children(|parent| {
            let points = Poisson2D::new()
              .with_dimensions([resolution.x as f64, resolution.y as f64], spawner.radius);
            points.iter().for_each(|point| {
              let sprite_n = rng.gen_range(0..=spawner.sprites.len() - 1);
              parent.spawn((SpriteBundle {
                texture: assets_server.load(&spawner.sprites[sprite_n]),
                transform: Transform {
                  translation: Vec3::new(
                    point[0] as f32 - resolution.x / 2.,
                    point[1] as f32 - resolution.y / 2.,
                    spawner.z_index,
                  ),
                  ..Default::default()
                },
                ..Default::default()
              },));
            })
          })
          .id();
        scroller.items_queue.push_back(scroller_item);
        commands.entity(entity).add_child(scroller_item);
      }
    }
  }
}
