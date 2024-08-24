// use std::marker::PhantomData;

// use bevy::{
//   ecs::system::{PipeSystem, SystemParam},
//   prelude::*,
// };

// use crate::SpriteScrollerItem;
// trait ScrollerSpawner {
//   // fn get_spawner() -> IntoSystem<(), (), _>;
// }

// pub struct SpriteSpawner;

// impl SpriteSpawner {
//   // pub fn get_spawner(&self) -> impl PipeSystem<(Entity, Vec<SpriteScrollerItem>), O, M> {
//   //   spawner
//   // }
//   pub fn spawner(
//     In((entity, items)): In<(Entity, Vec<SpriteScrollerItem>)>,
//     mut commands: Commands,
//   ) {
//     items.into_iter().for_each(|item| {
//       commands.spawn((
//         // Item {
//         //   size: item.size(),
//         //   // parent: entity,
//         // },
//         SpriteBundle {
//           texture: item.texture,
//           visibility: Visibility::Hidden,
//           ..default()
//         },
//       ));
//     });
//   }
// }
// // impl ScrollerSpawner for SpriteSpawner {}

// fn spawner(In((entity, items)): In<(Entity, Vec<SpriteScrollerItem>)>, mut commands: Commands) {
//   items.into_iter().for_each(|item| {
//     commands.spawn((
//       // Item {
//       //   size: item.size(),
//       //   // parent: entity,
//       // },
//       SpriteBundle {
//         texture: item.texture,
//         visibility: Visibility::Hidden,
//         ..default()
//       },
//     ));
//   });
// }
