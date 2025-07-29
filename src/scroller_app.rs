use std::any::TypeId;

use bevy::{ecs::system::SystemId, prelude::*, utils::HashMap};

use crate::{pre_generator, ScrollerGenerator, SpawnerInput};

#[derive(Resource, Default)]
pub struct ScrollerGenerators {
  pub generators: HashMap<TypeId, SystemId<Entity>>,
}

impl ScrollerGenerators {
  pub fn get<T: 'static>(&self) -> Option<&SystemId<Entity>> {
    self.generators.get(&TypeId::of::<T>())
  }
}

pub trait ScrollerApp {
  fn add_scroller_generator<
    T: ScrollerGenerator + Component + Clone,
    M,
    S: IntoSystem<SpawnerInput<T::Item>, (), M>,
  >(
    &mut self,
    system: S,
  ) -> &mut Self;
}

pub fn spawner() {}
impl ScrollerApp for App {
  fn add_scroller_generator<T, M, G>(&mut self, generator: G) -> &mut Self
  where
    T: ScrollerGenerator + Component + Clone,
    G: IntoSystem<SpawnerInput<T::Item>, (), M>,
  {
    let registered_system = self.register_system(pre_generator::<T>.pipe(spawner));

    let mut generators = self.world_mut().resource_mut::<ScrollerGenerators>();
    generators
      .generators
      .insert(TypeId::of::<T>(), registered_system);
    self
  }
}
