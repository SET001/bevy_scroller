#![allow(clippy::type_complexity)]

// mod linear_sprite_spawner;
// mod poisson_sprite_spawner;
mod generator;
mod plugin;
mod scroller;

pub use generator::*;
pub use plugin::ScrollerPlugin;
pub use scroller::{
  Scroller, ScrollerBundle, ScrollerDirection, ScrollerInitialized, ScrollerItem, ScrollerSize,
};

// use scroller::{
//   basic_scroller_init, bounded_scroller_init, scroller_on_items_added, ScrollerCameras,
// };
// // use linear_sprite_spawner::linear_sprite_spawner_init;
// // use poisson_sprite_spawner::poisson_sprite_spawner;
// // use scroller::scroller_init;

// pub use scroller::{
//   scroller_debug, scroller_delete_items, scroller_update, Scroller, ScrollerBundle, ScrollerItem,
//   ScrollerTypeBasic, ScrollerTypeBounded,
// };
