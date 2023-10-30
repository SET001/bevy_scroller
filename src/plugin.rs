pub struct ScrollerPlugin;

use crate::{generator::generator, scroller::*, ScrollerGenerator};
use bevy::prelude::*;

#[cfg(feature = "dev")]
use crate::scroller::scroller_debug;

impl Plugin for ScrollerPlugin {
  fn build(&self, app: &mut App) {
    app
      .register_type::<Scroller>()
      .register_type::<ScrollerGenerator>()
      .register_type::<ScrollerSize>()
      .register_type::<ScrollerDirection>()
      .register_type::<Vec<String>>()
      .register_type::<Vec<Entity>>()
      .add_systems(PreUpdate, init)
      .add_systems(
        Update,
        (
          update,
          #[cfg(feature = "dev")]
          scroller_debug,
          (generator, apply_deferred, on_items_added).chain(),
        ),
      )
      .add_systems(PostUpdate, (wait_items, delete_items));
  }
}
