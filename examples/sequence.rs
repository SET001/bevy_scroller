mod shared;

use bevy::{
  asset::{LoadState, LoadedFolder},
  prelude::*,
  window::PrimaryWindow,
};
use bevy_scroller::*;
use shared::get_app;
use std::collections::VecDeque;

#[derive(Resource)]
pub struct ScrollerImages(Handle<LoadedFolder>);

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum AppStates {
  #[default]
  Load,
  Run,
}
fn main() {
  get_app("sequence".into())
    .add_systems(Startup, startup)
    .add_systems(Update, wait_for_load.run_if(in_state(AppStates::Load)))
    .add_systems(OnEnter(AppStates::Run), run)
    .init_state::<AppStates>()
    .run();
}

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2dBundle::default());
  commands.insert_resource(ScrollerImages(asset_server.load_folder("gems")));
}

pub fn wait_for_load(
  scroller_images: Res<ScrollerImages>,
  asset_server: Res<AssetServer>,
  mut next_state: ResMut<NextState<AppStates>>,
) {
  if let Some(state) = asset_server.get_load_state(&scroller_images.0) {
    if state == LoadState::Loaded {
      *next_state = NextState(Some(AppStates::Run));
    }
  }
}

pub fn run(
  mut commands: Commands,
  windows: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
  images: Res<Assets<Image>>,
) {
  let items = (1..=7)
    .map(|i| {
      let path = format!("gems/{i}.png");
      let handle = asset_server.get_handle(path.clone()).unwrap();
      let image = images.get(handle).unwrap();
      SpriteScrollerItem {
        path,
        size: image.size().as_vec2(),
      }
    })
    .collect::<VecDeque<SpriteScrollerItem>>();

  let primary_window = windows.get_single().expect("no primary window");
  commands.spawn((
    ScrollerSize {
      size: Vec2::new(primary_window.width(), 128.),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 5.,
        ..default()
      },
      generator: SequenceSpriteGenerator { items },
      ..default()
    },
  ));
}
