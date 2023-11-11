use bevy::prelude::*;

use crate::{pre_generator, Scroller, ScrollerGenerator};

pub trait ScrollerApp {
  fn add_scroller_generator<
    T: ScrollerGenerator + Component + Clone,
    M,
    S: IntoSystem<Vec<(Entity, Scroller, Box<T::I>)>, (), M>,
  >(
    &mut self,
    system: S,
  ) -> &mut Self;
}

impl ScrollerApp for App {
  fn add_scroller_generator<
    T: ScrollerGenerator + Component + Clone,
    M,
    S: IntoSystem<Vec<(Entity, Scroller, Box<T::I>)>, (), M>,
  >(
    &mut self,
    system: S,
  ) -> &mut Self {
    self.add_systems(Update, pre_generator::<T>.pipe(system));
    self
  }
}
