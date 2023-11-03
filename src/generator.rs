use std::collections::VecDeque;

use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

use crate::{Scroller, ScrollerItem};

#[derive(Component, Reflect)]
pub enum ScrollerGenerator {
  SpriteSingle(String),
  SpriteSequence(VecDeque<String>),
  SpriteRandomSequence(Vec<String>),
}

impl Default for ScrollerGenerator {
  fn default() -> Self {
    Self::SpriteSingle("".into())
  }
}

pub fn spawn_sprite(
  commands: &mut Commands,
  images: &Res<Assets<Image>>,
  asset_server: &Res<AssetServer>,
  image_path: &str,
  parent: Entity,
) {
  let image_handle = asset_server.load(image_path);
  if let Some(image) = images.get(&image_handle) {
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
pub fn generator(
  mut commands: Commands,
  images: Res<Assets<Image>>,
  asset_server: Res<AssetServer>,
  mut q_scroller: Query<(&Scroller, &mut ScrollerGenerator, Entity)>,
) {
  let mut rng = thread_rng();
  for (scroller, mut generator, scroller_entity) in q_scroller.iter_mut() {
    if scroller.new_item_needed() {
      match *generator {
        ScrollerGenerator::SpriteSingle(ref image_path) => {
          spawn_sprite(
            &mut commands,
            &images,
            &asset_server,
            image_path,
            scroller_entity,
          );
        }
        ScrollerGenerator::SpriteSequence(ref mut image_paths) => {
          if let Some(image_path) = image_paths.pop_front() {
            spawn_sprite(
              &mut commands,
              &images,
              &asset_server,
              &image_path,
              scroller_entity,
            );
            image_paths.push_back(image_path);
          };
        }
        ScrollerGenerator::SpriteRandomSequence(ref mut image_paths) => {
          if let Some(image_path) = image_paths.choose(&mut rng) {
            spawn_sprite(
              &mut commands,
              &images,
              &asset_server,
              image_path,
              scroller_entity,
            );
          };
        }
      }
    }
  }
}
