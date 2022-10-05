# bevy_independent_transform

[![crates.io](https://img.shields.io/crates/v/bevy_independent_transform)](https://crates.io/crates/bevy_independent_transform)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/ickshonpe/bevy_independent_transform)
[![crates.io](https://img.shields.io/crates/d/bevy_independent_transform)](https://crates.io/crates/bevy_independent_transform)

Entities with an `IndependentTransform` have a position relative to their parent, but aren't part of the Bevy Transform hierarchy.

With `Transform` :

![image](/assets/dependent.png)

With `IndependentTransform`:
![image](/assets/independent_text.png)

Supports Bevy 0.8

#

## Setup

Add the dependency to your `Cargo.toml`

```toml
bevy_independent_transform = "0.1"
```

and the plugin to your `App`

```rust
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(IndependentTransformPlugin)
    // .. rest of App
    run()
}
```

## Usage

`IndependentTransform` is a newtype wrapping `Transform`.
Like `Transform`, an entity with a `IndependentTransform` must also have a `GlobalTransform`.

You can insert an `IndependentTransform` component directly, or spawn one of the bundles included with this crate:
* ```IndependentSpriteBundle```
* ```IndependentSpriteSheetBundle```
* ```IndependentText2dBundle```
* ```IndependentTransformBundle```

If an entity has both `IndependentTransform` and `Transform` components, the `IndependentTransform` component will be ignored.

## Examples
```
cargo run --example text
cargo run --example sprites
```