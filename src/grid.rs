use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

use crate::assets;

#[derive(Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Cell {
    pub position: Vec2,
    pub is_occupied: bool,
}

pub fn spawn_grid(
    mut commands: Commands,
    assets: Res<assets::GameAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let n = 8;
    for x in -n..(n + 1) {
        for z in -n..(n + 1) {
            let mut z = z as f32;
            // offset every other row
            if x % 2 == 0 {
                z += 0.5;
            }
            z = z as f32 * f32::sqrt(3.);
            let x = x as f32 * 1.5;
            commands.spawn((
                PbrBundle {
                    mesh: assets.hexagon_mesh.clone(),
                    material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
                    transform: Transform::from_translation(Vec3::new(x as f32, 0.0, z as f32)),
                    ..Default::default()
                },
                Cell {
                    position: Vec2::new(x as f32, z as f32),
                    is_occupied: false,
                },
                Name::new("Cell"),
                PickableBundle::default(),
            ));
        }
    }
}
