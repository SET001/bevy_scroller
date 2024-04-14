use bevy::prelude::*;

use crate::{pre_generator, ScrollerGenerator, SpawnerInput};

pub trait ScrollerApp {
  fn add_scroller_generator<
    T: ScrollerGenerator + Component + Clone,
    M,
    S: IntoSystem<SpawnerInput<T>, (), M>,
  >(
    &mut self,
    system: S,
  ) -> &mut Self;
}

impl ScrollerApp for App {
  fn add_scroller_generator<
    T: ScrollerGenerator + Component + Clone,
    M,
    S: IntoSystem<SpawnerInput<T>, (), M>,
  >(
    &mut self,
    system: S,
  ) -> &mut Self {
    self.add_systems(FixedUpdate, pre_generator::<T>.pipe(system));
    self
  }
}
