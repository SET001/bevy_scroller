#[derive(Resource)]
pub struct ScrollerPluginConfig {
  render_layers_range: Range<i32>,
}

pub struct ScrollerPlugin {
  pub render_layers_range: Range<i32>,
}

impl Default for ScrollerPlugin {
  fn default() -> Self {
    Self {
      render_layers_range: 1000..9999,
    }
  }
}

use std::ops::Range;

use crate::{
  scroller::*, sprite_spawner, spritesheet_spawner, RandomSequenceSpriteGenerator, ScrollerApp,
  ScrollerGenerators, SequenceSpriteGenerator, SequenceSpriteSheetGenerator, SingleSpriteGenerator,
};
use bevy::prelude::*;

// #[cfg(feature = "dev")]
// use crate::scroller::scroller_debug;

impl Plugin for ScrollerPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ScrollerPluginConfig {
        render_layers_range: self.render_layers_range.clone(),
      })
      // .register_type::<Scroller>()
      .register_type::<Size>()
      .register_type::<Direction>()
      .register_type::<Scroller>()
      .register_type::<ScrollerItem>()
      // .register_type::<SingleSpriteGenerator>()
      // .register_type::<SequenceSpriteGenerator>()
      // .register_type::<RandomSequenceSpriteGenerator>()
      .register_type::<SequenceSpriteSheetGenerator>()
      .register_type::<Vec<String>>()
      .register_type::<Vec<Entity>>()
      .init_resource::<ScrollerGenerators>()
      .add_scroller_generator::<SingleSpriteGenerator, _, _>(sprite_spawner)
      // .add_scroller_generator::<SequenceSpriteGenerator, _, _>(sprite_spawner)
      // .add_scroller_generator::<SequenceSpriteSheetGenerator, _, _>(spritesheet_spawner)
      // .add_scroller_generator::<RandomSequenceSpriteGenerator, _, _>(sprite_spawner)
      .add_systems(
        Update,
        (
          //     // on_items_added,
          //     // on_scroller_resize,
          //     // delete_items,
          init, fill_items, update,
          delete, // #[cfg(feature = "dev")]
                 // scroller_debug,
        )
          .chain(),
      )
      .observe(on_add);
    #[cfg(feature = "poisson")]
    {
      use crate::{poisson_generator, PoissonSpriteGenerator};
      app
        .register_type::<PoissonSpriteGenerator>()
        .add_scroller_generator::<PoissonSpriteGenerator, _, _>(poisson_generator);
    }
  }
}
