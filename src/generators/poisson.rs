use bevy::prelude::*;
use fast_poisson::Poisson2D;
use rand::{seq::SliceRandom, thread_rng};

use crate::{Scroller, ScrollerItem};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PoissonScrollerGenerator {
  pub radius: f32,
  pub rect: Vec2,
  pub sprites: Vec<String>,
  pub item_width: f32,
}

pub fn poisson_generator(
  mut commands: Commands,
  mut q_scroller: Query<(&Scroller, &mut PoissonScrollerGenerator, Entity)>,
  asset_server: Res<AssetServer>,
) {
  let mut rng = thread_rng();
  for (scroller, generator, scroller_entity) in q_scroller.iter_mut() {
    if scroller.new_item_needed() {
      commands
        .spawn((
          ScrollerItem {
            size: generator.rect,
            parent: scroller_entity,
          },
          SpatialBundle::default(),
        ))
        .with_children(|parent| {
          Poisson2D::new()
            .with_dimensions(
              [
                (generator.rect.x - generator.item_width) as f64,
                (generator.rect.y - generator.item_width) as f64,
              ],
              generator.radius as f64,
            )
            .iter()
            .for_each(|point| {
              let image = generator.sprites.choose(&mut rng).unwrap();
              let image_handle = asset_server.load(image);
              parent.spawn(SpriteBundle {
                texture: image_handle,
                transform: Transform::from_translation(Vec3::new(
                  point[0] as f32 - generator.rect.x / 2. + generator.item_width / 2.,
                  point[1] as f32 - generator.rect.y / 2. + generator.item_width / 2.,
                  0.,
                )),
                ..default()
              });
            });
        });
    }
  }
}
