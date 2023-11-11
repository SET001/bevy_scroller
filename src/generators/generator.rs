use std::fmt::Debug;

use bevy::prelude::*;

use crate::Scroller;

pub trait GeneratedItem {
  fn size(&self) -> Vec2;
}
pub trait ScrollerGenerator: Default {
  type I: GeneratedItem + Debug;
  fn gen_item(&mut self) -> Self::I;
}

// pub type PreGeneratorOutput<T: ScrollerGenerator> = Vec<(Entity, Scroller, T::I)>;
// pub type GeneratorInput<T> = In<PreGeneratorOutput<T>>;

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
