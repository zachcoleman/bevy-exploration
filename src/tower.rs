use bevy::prelude::*;
use bevy_mod_picking::{PickableBundle, Selection};

use crate::assets;
use crate::enemy;
use crate::orb;
use crate::grid;

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .register_type::<Shooting>()
            .register_type::<Range>()
            .register_type::<Damage>()
            .add_system(spawn_tower)
            .add_system(tower_shoot);
    }
}

#[derive(Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Tower;

#[derive(Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Shooting {
    pub timer: Timer,
}

#[derive(Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Range {
    pub range: f32,
}

#[derive(Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Damage {
    pub hp: usize,
}

pub fn spawn_tower(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    assets: Res<assets::GameAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    selected: Query<(&Transform, &Selection)>,
) {
    for key in keys.get_just_pressed() {
        match key {
            KeyCode::T => {
                let color = Color::rgba(0., 0.7, 0.7, 255.);
                let location = selected
                    .iter()
                    .filter(|t| t.1.selected())
                    .map(|t| t.0.translation)
                    .next();
                match location{
                    Some(location) => {
                        commands.spawn((
                            PbrBundle {
                                mesh: assets.tower_mesh.clone(),
                                material: materials.add(color.into()),
                                transform: Transform::from_translation(location + Vec3::new(0., 0.1, 0.)),
                                ..Default::default()
                            },
                            Tower,
                            Name::new("Tower"),
                            Shooting {
                                timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                            },
                            Range { range: 15.0 },
                            Damage { hp: 5 },
                            PickableBundle::default(),
                        ));
                    }
                    None => { continue; }
                }
            }
            _ => {}
        }
    }
}


pub fn spawn_default_towers(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<assets::GameAssets>,
    cell_query: Query<&Transform, With<grid::Cell>>,
) {
    for (_ux, cell) in cell_query.iter().enumerate() {
        if cell.translation.x.abs() < 3. && cell.translation.z.abs() < 3. {
            let color = Color::rgba(0., 0.7, 0.7, 255.);
            let location = cell.translation;
            commands.spawn((
                PbrBundle {
                    mesh: assets.tower_mesh.clone(),
                    material: materials.add(color.into()),
                    transform: Transform::from_translation(location + Vec3::new(0., 0.1, 0.)),
                    ..Default::default()
                },
                Tower,
                Shooting {
                    timer: Timer::from_seconds(1.0, TimerMode::Repeating),
                },
                Damage { hp: 5 },
                Range { range: 15.0 },
                PickableBundle::default(),
                Name::new("Tower"),
            ));
        }
    }
}


pub fn tower_shoot(
    mut commands: Commands,
    time: Res<Time>,
    mut tower_query: Query<(&mut Shooting, &Transform, &Range, Option<&Damage>), With<Tower>>,
    mut enemy_query: Query<&Transform, With<enemy::Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut shooting, transform, range, damage) in tower_query.iter_mut() {
        shooting.timer.tick(time.delta());
        if shooting.timer.finished() {
            // find the closest enemy
            let mut target: Option<&Transform> = None;
            for enemy_transform in enemy_query.iter_mut() {
                let distance = (enemy_transform.translation - transform.translation).length();
                if distance < range.range {
                    match target {
                        Some(targ) => {
                            if distance < (targ.translation - transform.translation).length() {
                                target = Some(enemy_transform);
                            }
                        }
                        None => {
                            target = Some(enemy_transform);
                        }
                    }
                }
            }
            if let Some(targ) = target {
                let start_pt = transform.translation + Vec3::new(0.0, 2., 0.);
                let target_pt = targ.translation;
                let mut orb = commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                        material: materials.add(Color::rgb(0.8, 0.1, 0.1).into()),
                        transform: Transform::from_translation(start_pt)
                            .looking_at(target_pt, Vec3::Y),
                        ..Default::default()
                    },
                    orb::Orb {
                        direction: (target_pt - start_pt).normalize(),
                        target: target_pt,
                        speed: 10.0,
                    },
                ));
                if let Some(dmg) = damage {
                    orb.insert(dmg.clone());
                }
            }
        }
    }
}
