use std::any::TypeId;

use bevy::{ecs::system::SystemId, prelude::*, utils::HashMap};

use crate::{pre_generator, ScrollerGenerator, SpawnerInput};

#[derive(Resource, Default)]
pub struct ScrollerGenerators {
  generators: HashMap<TypeId, SystemId<Entity>>,
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

impl ScrollerApp for App {
  fn add_scroller_generator<T, M, S>(&mut self, system: S) -> &mut Self
  where
    T: ScrollerGenerator + Component + Clone,
    S: IntoSystem<SpawnerInput<T::Item>, (), M>,
  {
    let registered_system = self.register_system(pre_generator::<T>.pipe(system));

    let mut generators = self.world_mut().resource_mut::<ScrollerGenerators>();
    generators
      .generators
      .insert(TypeId::of::<T>(), registered_system);
    self
  }
}
