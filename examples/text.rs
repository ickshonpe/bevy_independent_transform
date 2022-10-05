use bevy::math::vec3;
use bevy::prelude::*;
use bevy_independent_transform::IndependentTransform;
use bevy_independent_transform::IndependentTransformPlugin;
use bevy_independent_transform::bundles::IndependentText2dBundle;

const INDEPENDENT_TEXT: &str = "Text from an IndependentText2dBundle.\nThis text's entity is a child of the sprite entity\nbut it isn't rotated or scaled.\nPress <SPACE>.";
const DEPENDENT_TEXT: &str = "Text from a Text2dBundle\nRotation and scale are relative to the sprite entity.\nPress <SPACE>.";

const TRANSFORM: Transform = Transform {
    translation: vec3(0., 80., 0.),
    scale: Vec3::ONE,
    rotation: Quat::from_array([0.0, 0.0, 0.12467473, 0.9921977]),
};

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(64.0 * Vec2::ONE),
            ..Default::default()
        },
        texture: asset_server.load("sprite.png"),
        ..Default::default()
    })
    .with_children(|builder| {
        builder.spawn_bundle(IndependentText2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: INDEPENDENT_TEXT.to_string(), 
                    style: TextStyle { font: asset_server.load("FiraMono-Regular.ttf"), font_size: 14., color: Color::WHITE }
                }],
                alignment: TextAlignment::CENTER,
            },
            transform: TRANSFORM.into(),
            ..Default::default()
    });
    });
}

fn update(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Sprite>>
) {
    query.for_each_mut(|mut transform| {
        transform.translation.x = 200. * (time.seconds_since_startup() as f32 * 5f32.recip()).sin(); 
        transform.rotate_z(0.25 * time.delta_seconds());
        transform.scale = ((1. + (0.35 * time.seconds_since_startup() as f32).sin() * 0.25) * Vec2::ONE).extend(1.0);
    });
}

fn control(
    mut commands: Commands,
    mut flag: Local<bool>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Text)>
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        *flag = !*flag;
        query.for_each_mut(|(id, mut text)| {
            text.sections[0].value = if *flag {
                    DEPENDENT_TEXT
                } else {
                    INDEPENDENT_TEXT
                }.to_string();

            if *flag {
                commands.entity(id).remove::<IndependentTransform>();
                commands.entity(id).insert(TRANSFORM);
            } else {
                commands.entity(id).remove::<Transform>();
                commands.entity(id).insert(IndependentTransform(TRANSFORM));
            }
        });
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(IndependentTransformPlugin)
    .add_startup_system(setup)
    .add_system(update)
    .add_system(control)
    .run();
}