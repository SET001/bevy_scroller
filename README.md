<div align="center">

# `bevy_scroller`

A small [Bevy](https://github.com/bevyengine/bevy) plugin to scroll things.

[![crates.io](https://img.shields.io/crates/v/bevy_scroller)](https://crates.io/crates/bevy_scroller)
[![crates.io](https://img.shields.io/crates/d/bevy_scroller)](https://crates.io/crates/bevy_scroller)
[![docs.rs](https://docs.rs/bevy_scroller/badge.svg)](https://docs.rs/bevy_scroller)

![parallax](assets/examples/parallax.gif)
</div>

## About

Scroll predefined image sets, random images from sets, single image or even write own generators to generate scroller items (might be not images at all). You can also create parallaxes with it.

## Features

1. Change scroll direction
1. Support different scroll item sizes
1. Render to texture
1. Pre-build generators:
    1. Single - repeat single image
    1. Sequence - repeat sequence of iamge
    1. Random Sequence - scroller will consist of random iamge from sequence
    1. Custom generators - set up your own system to generate scroller items. With this, you can scroll not only images but anything

## Todo

- [ ] make it work with spritesheets
- [ ] scroller run conditions (when player moved, for instance)
- [ ] change scroll direction on the go
- [ ] some cases might be optimised with using shaders.

## Usage

spawn a scroller-entity with:
1. ScrollerSize component
1. Scroller component
1. If you want any of pre-build generators, attach ScrollerGenerator component

```rust
  commands.spawn((
    ScrollerGenerator::SpriteSingle("scroller_image.png".into()),
    ScrollerSize {
      size: Vec2::new(500., 300.),
    },
    ScrollerBundle {
      scroller: Scroller {
        speed: 1.,
        direction: ScrollerDirection::Forward,
        ..default()
      },
      ..default()
    },
  ));
```

## Examples

| example| preview | description |
|----|-----|---------------|
| [single](examples/parallax.rs) | | shows a basic usage |
| [sequence](examples/sequence.rs) | | shows a usage of sequence generator |
| [random_sequence](examples/random_sequence.rs) | | shows random sequence generator |
| [multiple](examples/multiple.rs) | | example of muptiple scrollers |
| [mirrors](examples/mirrors.rs) | ![parallax](assets/examples/mirrors.gif) | example of how you can render scroller to texture and then use that texture to show this same scroller in other parts of applications |
| [parallax](examples/parallax.rs) | ![parallax](assets/examples/parallax.gif) | showing how you can set up a parallax with this plugin |
| [poisson](examples/poisson.rs) | ![parallax](assets/examples/poisson.gif) | use of poisson generator to fill space with sprites and scroll them all. Set up radius to ensure that no entity generated closer than that radius. |
| [tilemaps](examples/tilemap.rs) | ![tilemap](assets/examples/tilemap.gif) | Show how to use scrollers with tilemaps. It uses custom generator to generate scroller items with tilemaps based on [bevy_ecs_tilemap](https://github.com/StarArawn/bevy_ecs_tilemap) |

## Credits

- [gems](https://opengameart.org/content/gems-set-01)
- [parallax](https://ansimuz.itch.io/mountain-dusk-parallax-background)
- tilemap - [Cute Forest](https://aamatniekss.itch.io/free-pixelart-tileset-cute-forest) and [Ocean Background](https://opengameart.org/content/ocean-background)
