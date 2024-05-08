use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;

use crate::{Scroller, ScrollerGenerator, ScrollerItem};

use super::generator::GeneratedItem;

#[derive(Clone, Reflect, Debug)]
pub struct SpriteScrollerItem {
  pub path: String,
  pub size: Vec2,
}

impl GeneratedItem for SpriteScrollerItem {
  fn size(&self) -> Vec2 {
    self.size
  }
}

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct SingleSpriteGenerator {
  pub path: String,
  pub size: Vec2,
}

impl ScrollerGenerator for SingleSpriteGenerator {
  type I = SpriteScrollerItem;

  fn gen_item(&mut self) -> Self::I {
    Self::I {
      size: self.size,
      path: self.path.clone(),
    }
  }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct SequenceSpriteGenerator {
  pub items: VecDeque<SpriteScrollerItem>,
}

impl ScrollerGenerator for SequenceSpriteGenerator {
  type I = SpriteScrollerItem;

  fn gen_item(&mut self) -> Self::I {
    let item = self.items.pop_front().unwrap();
    self.items.push_back(item.clone());
    item
  }
}

#[derive(Component, Default, Reflect, Clone)]
#[reflect(Component)]
pub struct RandomSequenceSpriteGenerator {
  pub items: Vec<SpriteScrollerItem>,
}

impl ScrollerGenerator for RandomSequenceSpriteGenerator {
  type I = SpriteScrollerItem;

  fn gen_item(&mut self) -> Self::I {
    let mut rng = thread_rng();
    self.items.choose(&mut rng).unwrap().clone()
  }
}

pub fn sprite_spawner(
  In(input): In<Vec<(Entity, Scroller, Box<SpriteScrollerItem>)>>,
  mut commands: Commands,
  asset_server: Res<AssetServer>,
) {
  input.into_iter().for_each(|(entity, _, item)| {
    let handle = asset_server.load(item.path.clone());
    commands.spawn((
      ScrollerItem {
        size: item.size(),
        parent: entity,
      },
      SpriteBundle {
        texture: handle,
        visibility: Visibility::Hidden,
        ..default()
      },
    ));
  });
}
