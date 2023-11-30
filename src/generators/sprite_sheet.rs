use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{GeneratedItem, Scroller, ScrollerGenerator, ScrollerItem};

#[derive(Debug)]
pub struct SpriteSheetScrollerItem {
  sprite: u32,
}

impl GeneratedItem for SpriteSheetScrollerItem {
  fn size(&self) -> Vec2 {
    Vec2::new(64., 64.) * 2.
  }
}

#[derive(Default, Component, Clone)]
pub struct SequenceSpriteSheetGenerator {
  pub texture_atlas: Handle<TextureAtlas>,
  pub sprites: VecDeque<u32>,
}

impl ScrollerGenerator for SequenceSpriteSheetGenerator {
  type I = SpriteSheetScrollerItem;
  fn gen_item(&mut self) -> Self::I {
    self.sprites.rotate_left(1);

    SpriteSheetScrollerItem {
      sprite: *self.sprites.front().unwrap(),
    }
  }
}
pub fn spritesheet_spawner(
  In(input): In<Vec<(Entity, Scroller, Box<SpriteSheetScrollerItem>)>>,
  q_gen: Query<&SequenceSpriteSheetGenerator>,
  mut commands: Commands,
) {
  input.into_iter().for_each(|(entity, _, item)| {
    let generator = q_gen.get(entity).unwrap();
    commands.spawn((
      ScrollerItem {
        size: item.size(),
        parent: entity,
      },
      SpriteSheetBundle {
        visibility: Visibility::Hidden,
        texture_atlas: generator.texture_atlas.clone(),
        sprite: TextureAtlasSprite::new(item.sprite as usize),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..default()
      },
    ));
  })
}
