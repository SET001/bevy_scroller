# bevy_scroller

Bevy plugin to scroll things. It allow to write generators with custom logic for item generation
You can also create parallaxes with it.

## Features
1. Change scroll direction
1. Support dfferent scroll item sizes
1. Render to texture
1. Pre-build generators:
    1. Single - repeat single image
    1. Sequence - repeat sequence of iamge
    1. Random Sequence - scroller will consist of random iamge from sequence
    1. Custom generators - set up your own system to generate scroller items. With this, you can scroll not only images but anything

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

| example| image | description |
|----|-----|---------------|
| [single](examples/parallax.rs) | | shows a basic usage |
| [sequence](examples/sequence.rs) | | shows a usage of sequence generator |
| [random_sequence](examples/random_sequence.rs) | | shows random sequence generator |
| [multiple](examples/multiple.rs) | | example of muptiple scrollers |
| [mirrors](examples/mirrors.rs) | | example of how you can render scroller to texture and then use that texture to show this same scroller in other parts of applications |
| [parallax](examples/parallax.rs) | ![parallax](assets/examples/parallax.gif) | showing how you can set up a parallax with this plugin |

## Credits
- [gems](https://opengameart.org/content/gems-set-01)
- [parallax](https://ansimuz.itch.io/mountain-dusk-parallax-background)
