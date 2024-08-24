use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;

use crate::ScrollerItem;

use super::{
  generator::{GeneratedItem, ScrollerGenerator},
  SpawnerInput,
};

#[derive(Clone, Debug, Reflect)]
pub struct SpriteScrollerItem {
  pub texture: Handle<Image>,
  pub size: Vec2,
}

impl GeneratedItem for SpriteScrollerItem {
  fn size(&self) -> Vec2 {
    self.size
  }
}

#[derive(Component, Clone)]
pub struct SingleSpriteGenerator {
  pub texture: Handle<Image>,
  pub size: Vec2,
  // spawner: SystemId,
}

impl ScrollerGenerator for SingleSpriteGenerator {
  type Item = SpriteScrollerItem;

  // fn get_spawner(&self) -> SystemId {
  //   self.spawner
  // }

  fn gen_item(&mut self) -> Self::Item {
    Self::Item {
      size: self.size,
      texture: self.texture.clone(),
    }
  }
}

#[derive(Component, Clone)]
pub struct SequenceSpriteGenerator {
  pub items: VecDeque<SpriteScrollerItem>,
}

impl ScrollerGenerator for SequenceSpriteGenerator {
  type Item = SpriteScrollerItem;

  fn gen_item(&mut self) -> Self::Item {
    let item = self.items.pop_front().unwrap();
    self.items.push_back(item.clone());
    item
  }
}

#[derive(Component, Clone)]
pub struct RandomSequenceSpriteGenerator {
  pub items: Vec<SpriteScrollerItem>,
}

impl ScrollerGenerator for RandomSequenceSpriteGenerator {
  type Item = SpriteScrollerItem;

  fn gen_item(&mut self) -> Self::Item {
    let mut rng = thread_rng();
    self.items.choose(&mut rng).unwrap().clone()
  }
}

pub fn sprite_spawner(
  In((entity, items)): In<SpawnerInput<SpriteScrollerItem>>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  info!(
    "sprite spawner to generate {:?} items",
    items.len()
  );
  items.iter().for_each(|item| {
    commands.spawn((
      ScrollerItem::new(item.size(), entity),
      SpriteBundle {
        texture: item.texture.clone(),
        ..default()
      },
    ));
  });
  // input.into_iter().for_each(|(entity, _, item)| {
  //   let handle = asset_server.load(item.path.clone());
  //   commands.spawn((
  //     // ScrollerItem {
  //     //   size: item.size(),
  //     //   parent: entity,
  //     // },
  //     SpriteBundle {
  //       texture: handle,
  //       visibility: Visibility::Hidden,
  //       ..default()
  //     },
  //   ));
  // });
}
