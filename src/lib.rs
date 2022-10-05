pub mod bundles;

use bevy::prelude::*;
use bevy::transform::TransformSystem;

pub use bundles::*;

#[derive(Component, Debug, Default, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component, Default, PartialEq)]
pub struct IndependentTransform(pub Transform);

impl From<Transform> for IndependentTransform {
    #[inline]
    fn from(transform: Transform) -> Self {
        Self(transform)
    }
}

impl From<IndependentTransform> for Transform {
    #[inline]
    fn from(independent_transform: IndependentTransform) -> Self {
        independent_transform.0
    }
}

#[derive(Bundle, Clone)]
pub struct IndependentTransformBundle {
    independent: IndependentTransform,
    global: GlobalTransform,
}

impl IndependentTransformBundle {
    #[inline]
    pub const fn from_independent_transform(independent_transform: IndependentTransform) -> Self {
        Self {
            independent: independent_transform,
            ..Self::identity()
        }
    }

    #[inline]
    pub const fn identity() -> Self {
        Self {
            independent: IndependentTransform(Transform::identity()),
            global: GlobalTransform::identity(),
        }
    }
}

impl From<Transform> for IndependentTransformBundle {
    #[inline]
    fn from(transform: Transform) -> Self {
        Self::from_independent_transform(transform.into())
    }
}

impl From<IndependentTransform> for IndependentTransformBundle {
    #[inline]
    fn from(independent_transform: IndependentTransform) -> Self {
        Self::from_independent_transform(independent_transform)
    }
}

pub fn update_independent_transforms(
    mut children: Query<(&IndependentTransform, Changed<IndependentTransform>, &mut GlobalTransform, &Parent), Without<Transform>>,
    parents: Query<(&GlobalTransform, Changed<GlobalTransform>), Without<IndependentTransform>>
) {
    children.for_each_mut(|(
        independent_transform, 
        is_independent_transform_changed,
        mut global_transform, 
        parent
    )| {
        if let Ok((parent_transform, is_parent_transform_changed)) = parents.get(parent.get()) {
            if is_parent_transform_changed || is_independent_transform_changed {
                *global_transform = independent_transform.0.into();
                *global_transform.translation_mut() += parent_transform.translation_vec3a();
            }
        } else if is_independent_transform_changed {
            *global_transform = independent_transform.0.into();
        }
    });
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum IndependentTransformSystem {
    PropagateTranslation,
}

pub struct IndependentTransformPlugin;

impl Plugin for IndependentTransformPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<IndependentTransform>()
        .add_startup_system_to_stage(
            StartupStage::PostStartup,
            update_independent_transforms
                .label(IndependentTransformSystem::PropagateTranslation)
                .label(TransformSystem::TransformPropagate)
                .after(bevy::transform::transform_propagate_system)
        )
        .add_system_to_stage(
            CoreStage::PostUpdate, 
            update_independent_transforms
                .label(IndependentTransformSystem::PropagateTranslation)
                .label(TransformSystem::TransformPropagate)
                .after(bevy::transform::transform_propagate_system)
                
        );
    }
}