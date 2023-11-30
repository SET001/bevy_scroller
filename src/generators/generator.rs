use std::fmt::Debug;

use bevy::prelude::*;

use crate::Scroller;

pub trait GeneratedItem: Debug {
  fn size(&self) -> Vec2;
}
pub trait ScrollerGenerator: Default {
  type I: GeneratedItem + Debug;
  fn gen_item(&mut self) -> Self::I;
}

pub type SpawnerInput<T> = Vec<(Entity, Scroller, Box<<T as ScrollerGenerator>::I>)>;

const GENERATIONS_LIMIT: u32 = 300;

pub fn pre_generator<T>(mut q_scroller: Query<(Entity, &Scroller, &mut T)>) -> SpawnerInput<T>
where
  T: ScrollerGenerator + Component + Clone,
{
  q_scroller
    .iter_mut()
    .flat_map(|(entity, scroller, mut generator)| {
      let mut length = scroller.get_free_space();
      let mut generations = 0;
      let mut to_generate = vec![];
      while length > 0. && generations <= GENERATIONS_LIMIT {
        let item = generator.gen_item();
        debug!("generated item is: {:?}", item);
        length -= item.size().x;
        generations += 1;
        to_generate.push((entity, scroller.clone(), Box::new(item)));
      }
      if generations > GENERATIONS_LIMIT {
        panic!("Reached item generation limit");
      }

      to_generate
    })
    .collect()
}
