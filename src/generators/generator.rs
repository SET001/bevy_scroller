use std::fmt::Debug;

use bevy::prelude::*;

use crate::{Scroller, ScrollerItem, Size};

pub trait GeneratedItem: Debug {
  fn size(&self) -> Vec2;
}
pub trait ScrollerGenerator {
  type Item: GeneratedItem + Debug;
  fn gen_item(&mut self) -> Self::Item;
}

pub type SpawnerInput<I> = (Entity, Vec<I>);

pub fn pre_generator<G>(
  In(entity): In<Entity>,
  mut query: Query<(&mut G, &Scroller, &Size)>,
  q_item: Query<(&Transform, &ScrollerItem)>,
) -> SpawnerInput<G::Item>
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
    while free_space > 0. {
      let item = generator.gen_item();
      // width += item.size().x;
      free_space -= item.size().x;
      items.push(item);
    }
    items
  } else {
    vec![]
  };
  (entity, items)
}
