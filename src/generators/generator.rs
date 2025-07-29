use std::{any::TypeId, fmt::Debug};

use bevy::{ecs::world, prelude::*};

use crate::{Scroller, ScrollerGenerators, ScrollerItem, Size};

pub trait GeneratedItem: Debug {
  fn size(&self) -> Vec2;
}
pub trait ScrollerGenerator {
  type Item: GeneratedItem + Debug;
  fn gen_item(&mut self) -> Self::Item;
}

pub type SpawnerInput<I> = (Entity, Vec<I>);

pub fn spawner() -> Box<Vec<impl Bundle>> {
  Box::new(vec![{}])
}

pub fn gen<G: 'static>(world: &mut World) {
  if let Some(generator) = world
    .get_resource_ref::<ScrollerGenerators>()
    .unwrap()
    .generators
    .get(&TypeId::of::<G>())
  {
    let spid = world.register_system(spawner);
    let asd = world.run_system(spid).unwrap();
    world.spawn_batch(asd.into_iter());
  }
}
pub fn generator<G>(
  In(entity): In<Entity>,
  mut query: Query<(&mut G, &Scroller, &Size)>,
  q_item: Query<(&Transform, &ScrollerItem)>,
  generators: Res<ScrollerGenerators>,
) -> Box<Vec<impl Bundle>>
where
  G: ScrollerGenerator + Component + Clone,
{
  let items = if let Ok((mut generator, scroller, size)) = query.get_mut(entity) {
    let edge = match scroller.last_item {
      Some(last_item) => {
        let (position, item) = q_item.get(last_item).unwrap();
        position.translation.x + item.size.x / 2.
      }
      None => 0.,
    };
    let mut free_space = size.x - edge;
    // let mut width = scroller.items_width;
    let mut items = vec![];
    if let Some(generator) = generators.generators.get(&TypeId::of::<G>()) {
      while free_space > 0. {
        world.run_system_once(generator).unwrap();
        // width += item.size().x;
        free_space -= item.size().x;
        items.push(item);
      }
    }
    items
  } else {
    vec![]
  };
  (entity, items)
}
