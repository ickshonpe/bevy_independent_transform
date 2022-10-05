use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::text::Text2dBounds;
use bevy::text::Text2dSize;
use crate::IndependentTransform;

#[derive(Bundle, Clone)]
pub struct IndependentSpriteBundle {
    /// properties of the sprite to be drawn
    pub sprite: Sprite,
    /// non-hierarchical transform
    pub transform: IndependentTransform,
    pub global_transform: GlobalTransform,
    /// A handle to the sprite's texture
    pub texture: Handle<Image>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

impl Default for IndependentSpriteBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}

/// A Bundle of components for drawing a single sprite from a sprite sheet (also referred
/// to as a `TextureAtlas`)
#[derive(Bundle, Clone, Default)]
pub struct IndependentSpriteSheetBundle {
    /// The specific sprite from the texture atlas to be drawn
    pub sprite: TextureAtlasSprite,
    /// A handle to the texture atlas that holds the sprite images
    pub texture_atlas: Handle<TextureAtlas>,
    /// non-hierarchical transform
    pub transform: IndependentTransform,
    pub global_transform: GlobalTransform,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}

/// The bundle of components needed to draw text in a 2D scene via a 2D `Camera2dBundle`.
#[derive(Bundle, Clone, Debug, Default)]
pub struct IndependentText2dBundle {
    /// The text to be drawn
    pub text: Text,
    /// non-hierarchical transform
    pub transform: IndependentTransform,
    pub global_transform: GlobalTransform,
    /// The calculated size of text drawn in 2D scene
    pub text_2d_size: Text2dSize,
    /// The maximum width and height of text. 
    pub text_2d_bounds: Text2dBounds,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}