use bevy::prelude::*;
use fast_poisson::Poisson2D;
use rand::{seq::SliceRandom, thread_rng};

use crate::{GeneratedItem, Scroller, ScrollerGenerator, ScrollerItem};

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct PoissonSpriteGenerator {
  pub radius: f32,
  pub sprites: Vec<String>,
  pub item_size: Vec2,
  pub sub_item_size: Vec2,
}

#[derive(Debug)]
struct ItemSprite {
  path: String,
  position: Vec2,
}

#[derive(Debug)]
pub struct PoissonScrollerItem {
  sprites: Vec<ItemSprite>,
  size: Vec2,
}

impl GeneratedItem for PoissonScrollerItem {
  fn size(&self) -> Vec2 {
    self.size
  }
}

impl ScrollerGenerator for PoissonSpriteGenerator {
  type I = PoissonScrollerItem;

  fn gen_item(&mut self) -> Self::I {
    let mut rng = thread_rng();

    Self::I {
      size: self.item_size,
      sprites: Poisson2D::new()
        .with_dimensions(
          [
            self.item_size.x - self.sub_item_size.x,
            self.item_size.y - self.sub_item_size.y,
          ],
          self.radius,
        )
        .iter()
        .map(|point| ItemSprite {
          path: self.sprites.choose(&mut rng).unwrap().clone(),
          position: Vec2::from(point) - (self.item_size - self.sub_item_size) / 2.,
        })
        .collect(),
    }
  }
}

pub fn poisson_generator(
  In(input): In<Vec<(Entity, Scroller, Box<PoissonScrollerItem>)>>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  for (entity, _, item) in input.iter() {
    commands
      .spawn((
        ScrollerItem {
          size: item.size,
          parent: *entity,
        },
        SpatialBundle::default(),
      ))
      .with_children(|parent| {
        for subitem in item.sprites.iter() {
          let image_handle = asset_server.load(subitem.path.clone());
          parent.spawn(SpriteBundle {
            texture: image_handle,
            transform: Transform::from_translation(subitem.position.extend(0.)),
            ..default()
          });
        }
      });
  }
}
