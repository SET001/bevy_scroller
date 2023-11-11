use std::{any::TypeId, collections::VecDeque, fmt::Debug};

use bevy::{
  ecs::system::{RunSystemOnce, SystemId},
  prelude::*,
  utils::HashMap,
};

use bevy_scroller_macros::ScrollerGenerator;
use rand::{seq::SliceRandom, thread_rng};

use crate::{Scroller, ScrollerItem};

pub trait GeneratedItem {
  fn size(&self) -> Vec2;
}
pub trait ScrollerGenerator: Default {
  type I: GeneratedItem + Debug;
  fn gen_item(&mut self) -> Self::I;
}

// pub type PreGeneratorOutput<T: ScrollerGenerator> = Vec<(Entity, Scroller, T::I)>;
// pub type GeneratorInput<T> = In<PreGeneratorOutput<T>>;

// pub enum ScrollerSpriteGenerator {
//   Single(String),
//   Sequence(VecDeque<String>),
//   RandomSequence(Vec<String>),
// }
// impl ScrollerGeneratorType for ScrollerSpriteSingleGenerator {}

// impl Default for ScrollerGenerator {
//   fn default() -> Self {
//     Self::SpriteSingle("".into())
//   }
// }

pub fn pre_generator<T>(
  mut q_pending_scroller: Query<(Entity, &Scroller, &mut T)>,
) -> Vec<(Entity, Scroller, Box<T::I>)>
where
  T: ScrollerGenerator + Component + Clone,
{
  q_pending_scroller
    .iter_mut()
    .flat_map(|(entity, scroller, mut generator)| {
      let mut length = scroller.get_free_space();
      let mut to_generate = vec![];
      while length > 0. {
        let item = generator.gen_item();
        debug!("generated item is: {:#?}", item);
        length -= item.size().x;
        to_generate.push((entity, scroller.clone(), Box::new(item)));
      }
      to_generate
    })
    .collect()
}

// pub fn sprite_generator(
//   In(input): GeneratorInput<ScrollerSingleSpriteGenerator>,
//   mut commands: Commands,
//   images: Res<Assets<Image>>,
//   asset_server: Res<AssetServer>,
//   // mut q_scroller: Query<(&Scroller, &mut ScrollerGenerator, Entity)>,
// ) {
//   if input.len() > 0 {
//     println!("should generate {} items", input.len());
//   }

//   input.into_iter().for_each(|(entity, scroller, generator)| {
//     spawn_sprite(
//       &mut commands,
//       &images,
//       &asset_server,
//       generator.sprite_path.clone(),
//       entity,
//     );
//   });
//   // let mut rng = thread_rng();
//   // for (scroller, mut generator, scroller_entity) in q_scroller.iter_mut() {
//   //   if scroller.new_item_needed() {
//   //     match *generator {
//   //       ScrollerGenerator::SpriteSingle(ref image_path) => {
//   //         spawn_sprite(
//   //           &mut commands,
//   //           &images,
//   //           &asset_server,
//   //           image_path,
//   //           scroller_entity,
//   //         );
//   //       }
//   //       ScrollerGenerator::SpriteSequence(ref mut image_paths) => {
//   //         if let Some(image_path) = image_paths.pop_front() {
//   //           spawn_sprite(
//   //             &mut commands,
//   //             &images,
//   //             &asset_server,
//   //             &image_path,
//   //             scroller_entity,
//   //           );
//   //           image_paths.push_back(image_path);
//   //         };
//   //       }
//   //       ScrollerGenerator::SpriteRandomSequence(ref mut image_paths) => {
//   //         if let Some(image_path) = image_paths.choose(&mut rng) {
//   //           spawn_sprite(
//   //             &mut commands,
//   //             &images,
//   //             &asset_server,
//   //             image_path,
//   //             scroller_entity,
//   //           );
//   //         };
//   //       }
//   //     }
//   //   }
//   // }
// }
