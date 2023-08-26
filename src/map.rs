use bevy::prelude::*;

pub fn spawn_basic_scene(mut commands: Commands) {
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 10000.,
                range: 1000.,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0., 30., 0.)),
            ..Default::default()
        })
        .insert(Name::new("Light"));
}
