use crate::ScrollerGenerator;
use bevy::{
  ecs::{component::StorageType, system::SystemId},
  prelude::*,
  reflect::Reflect,
  render::{
    camera::RenderTarget,
    render_resource::{
      Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    view::RenderLayers,
  },
};
use bevy_render_layers_manager::RenderLayerManager;

#[derive(Reflect, Default, Debug, Clone, Component)]
pub enum Direction {
  #[default]
  Forward,
  Backward,
}
#[derive(Copy, Clone, Default, Component, Reflect, Deref, DerefMut)]
pub struct Size(pub Vec2);

impl From<Vec2> for Size {
  fn from(value: Vec2) -> Self {
    Self(value)
  }
}

#[derive(Component, Default)]
pub enum FillMode {
  #[default]
  Filled,
  NotFilled,
}

#[derive(Debug, Component, Clone, Reflect)]
pub struct Scroller {
  pub speed: f32,
  pub is_paused: bool,
  // items: Vec<Item>,
  pub items_width: f32,
  pub last_item: Option<Entity>,
  render_layer: usize,
}

impl Scroller {
  pub fn new(speed: f32) -> Self {
    Self {
      speed,
      // items: vec![],
      items_width: 0.,
      is_paused: false,
      last_item: None,
      render_layer: 0,
    }
  }
}
pub struct UnnamedScrollerIndex(pub u32);
impl Default for UnnamedScrollerIndex {
  fn default() -> Self {
    Self(1)
  }
}

#[derive(Copy, Clone, Debug, Reflect)]
pub struct ScrollerItem {
  pub size: Vec2,
  pub parent: Entity,
  limit_position: f32,
}

impl ScrollerItem {
  pub fn new(size: Vec2, parent: Entity) -> Self {
    Self {
      size,
      parent,
      limit_position: 0.,
    }
  }
}

impl Component for ScrollerItem {
  const STORAGE_TYPE: bevy::ecs::component::StorageType = StorageType::Table;
  fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
    hooks.on_remove(|mut world, entity, _component_id| {
      let item = *world.get::<ScrollerItem>(entity).unwrap();
      let mut scroller = world.get_mut::<Scroller>(item.parent).unwrap();
      scroller.items_width -= item.size.x;
    });
  }
}

pub fn on_add(
  trigger: Trigger<OnAdd, ScrollerItem>,
  mut commands: Commands,
  mut q_item: Query<(&mut ScrollerItem, &Transform)>,
  mut q_scroller: Query<(&mut Scroller, &Size)>,
) {
  let (mut item, _) = q_item.get_mut(trigger.entity()).unwrap();
  let (mut scroller, size) = q_scroller.get_mut(item.parent).unwrap();
  // commands.entity(item.parent).add_child(trigger.entity());``
  item.limit_position = -(size.x + item.size.x) / 2.;
  let item_size = item.size;
  commands.entity(item.parent).add_child(trigger.entity());
  scroller.items_width += item.size.x;

  let position = match scroller.last_item {
    Some(last_item) => {
      let (last_item, transform) = q_item.get(last_item).unwrap();
      transform.translation.x + (last_item.size.x + item_size.x) / 2.
    }
    None => -(size.x - item.size.x) / 2.,
  };

  commands.entity(trigger.entity()).insert((
    Name::new("Scroller item"),
    Transform::from_translation(Vec2::new(position, 0.).extend(0.)),
    RenderLayers::from_layers(&[scroller.render_layer]),
  ));

  info!("scroller position calculated to be: {position}",);

  info!(
    "scroller items_width increased to: {}",
    scroller.items_width
  );
  scroller.last_item = Some(trigger.entity());
}

#[derive(Component)]
pub struct ScrollerSpawner(pub SystemId<Entity>);

#[derive(Bundle)]
pub struct ScrollerBundle<G>
where
  G: Component + ScrollerGenerator,
{
  pub scroller: Scroller,
  pub size: Size,
  pub direction: Direction,
  pub spatial: SpatialBundle,
  pub generator: G,
  pub spawner: ScrollerSpawner,
  pub fill_mode: FillMode,
}

#[derive(Event)]
pub struct FillScroller;

pub fn init(
  mut commands: Commands,
  mut scroller_index: Local<UnnamedScrollerIndex>,
  mut images: ResMut<Assets<Image>>,
  mut q_added_scroller: Query<
    (
      Entity,
      &mut Scroller,
      Option<&Name>,
      Option<&Direction>,
      Option<&Size>,
    ),
    Added<Scroller>,
  >,
  render_manager: Res<RenderLayerManager>,
) {
  for (entity, mut scroller, maybe_name, maybe_direction, maybe_size) in q_added_scroller.iter_mut()
  {
    if maybe_direction.is_none() {
      warn!("Scroller without direction");
    }
    if maybe_size.is_none() {
      warn!("Scroller without size");
    }
    if let (Some(direction), Some(size)) = (maybe_direction, maybe_size) {
      if maybe_name.is_none() {
        let name = format!("Scroller #{}", scroller_index.0);
        commands.entity(entity).insert(Name::new(name.clone()));
        scroller_index.0 += 1;
      };

      let size = Extent3d {
        width: size.x as u32,
        height: size.y as u32,
        ..default()
      };

      let mut image = Image {
        texture_descriptor: TextureDescriptor {
          label: None,
          size,
          dimension: TextureDimension::D2,
          format: TextureFormat::Bgra8UnormSrgb,
          mip_level_count: 1,
          sample_count: 1,
          usage: TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST
            | TextureUsages::RENDER_ATTACHMENT,
          view_formats: &[],
        },
        ..default()
      };
      image.resize(size);
      let image_handle = images.add(image);

      scroller.render_layer = render_manager.get();

      commands.entity(entity).with_children(|parent| {
        parent.spawn((
          Camera2dBundle {
            camera: Camera {
              // render before the "main pass" camera
              order: -1,
              target: RenderTarget::Image(image_handle.clone()),
              ..default()
            },
            ..default()
          },
          // InGameCamera,
          RenderLayers::from_layers(&[scroller.render_layer]),
        ));
        parent.spawn((
          SpriteBundle {
            texture: image_handle,
            ..default()
          },
          Name::new("Scroller texture"),
        ));
      });
    }
  }
}

pub fn fill_items(
  mut commands: Commands,
  q_scroller: Query<(Entity, &mut Scroller, &Size, &ScrollerSpawner)>,
  q_item: Query<(&Transform, &ScrollerItem)>,
) {
  for (entity, scroller, size, spawner) in q_scroller.iter() {
    let edge = match scroller.last_item {
      Some(last_item) => {
        let (position, item) = q_item.get(last_item).unwrap();
        position.translation.x + item.size.x / 2.
      }
      None => 0.,
    };
    let free_space = size.x - edge;
    if free_space > 0. {
      commands.run_system_with_input(spawner.0, entity);
    }
  }
}

pub fn update(
  q_scroller: Query<(&Scroller, &Children)>,
  mut q_item: Query<&mut Transform, With<ScrollerItem>>,
  time_fixed: Res<Time<Virtual>>,
) {
  let step: f32 = 1. / 60.;
  let delta = time_fixed.delta_seconds();
  for (scroller, children) in q_scroller.iter() {
    for child in children.iter() {
      if let Ok(mut transform) = q_item.get_mut(*child) {
        let update_step = delta / step * scroller.speed;
        transform.translation.x -= update_step;
      };
    }
  }
}

pub fn delete(q_item: Query<(&Transform, Entity, &ScrollerItem)>, mut commands: Commands) {
  for (item_transform, entity, item) in q_item.iter() {
    if item_transform.translation.x < item.limit_position {
      commands.entity(entity).despawn_recursive();
    }
  }
}
