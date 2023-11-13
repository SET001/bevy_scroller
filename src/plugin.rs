pub struct ScrollerPlugin;

use crate::{
  poisson_generator, scroller::*, sprite_spawner, PoissonSpriteGenerator,
  RandomSequenceSpriteGenerator, ScrollerApp, SequenceSpriteGenerator, SingleSpriteGenerator,
};
use bevy::prelude::*;

#[cfg(feature = "dev")]
use crate::scroller::scroller_debug;

impl Plugin for ScrollerPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Scroller>()
      // .register_type::<ScrollerGenerator>()
      .register_type::<ScrollerSize>()
      .register_type::<ScrollerDirection>()
      // .register_type::<PoissonScrollerGenerator>()
      .register_type::<SequenceSpriteGenerator>()
      .register_type::<Vec<String>>()
      .register_type::<Vec<Entity>>()
      .add_scroller_generator::<SingleSpriteGenerator, _, _>(sprite_spawner)
      .add_scroller_generator::<SequenceSpriteGenerator, _, _>(sprite_spawner)
      .add_scroller_generator::<RandomSequenceSpriteGenerator, _, _>(sprite_spawner)
      .add_scroller_generator::<PoissonSpriteGenerator, _, _>(poisson_generator)
      .add_systems(PreUpdate, (init, on_items_added).chain())
      .add_systems(
        Update,
        (
          update,
          #[cfg(feature = "dev")]
          scroller_debug,
        ),
      )
      .add_systems(PostUpdate, delete_items);
  }
}
