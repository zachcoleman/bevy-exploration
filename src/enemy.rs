use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use rand::Rng;

use crate::camera;
use crate::health;
use crate::orb;
use crate::quad_tree;
use crate::tower;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .add_system(spawn_enemy)
            .add_system(enemy_move);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy;

pub fn spawn_enemy(
    keys: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    camera: Query<&camera::CameraLookAt>,
) {
    let camera = camera.iter().next().unwrap();
    let location = camera.target + Vec3::new(0.0, 0.5, 0.);
    let rng = &mut rand::thread_rng();
    for key in keys.get_just_pressed() {
        match key {
            KeyCode::X => {
                for _ in 0..10 {
                    let (jitter_x, jitter_z) = (rng.gen_range(-4.0..4.0), rng.gen_range(-4.0..4.0));
                    let loc = location + Vec3::new(jitter_x, 0., jitter_z);
                    commands.spawn((
                        PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.75 })),
                            material: materials.add(StandardMaterial {
                                base_color: Color::rgb(0.8, 0.1, 0.1).into(),
                                alpha_mode: AlphaMode::Blend,
                                ..Default::default()
                            }),
                            transform: Transform::from_translation(loc)
                                .looking_at(Vec3::new(0., 0.5, 0.), Vec3::Y),
                            ..Default::default()
                        },
                        health::HealthPoints { hp: 20, max_hp: 20 },
                        health::Regen {
                            hp: 1,
                            timer: Timer::from_seconds(2., TimerMode::Repeating),
                        },
                        Enemy,
                        Name::new("Enemy"),
                    ));
                }
            }
            _ => {}
        }
    }
}

pub fn enemy_move(time: Res<Time>, mut query: Query<(&mut Transform, Entity), With<Enemy>>) {
    // get all transforms and entities for referencing against
    let vec_transforms: Vec<(Transform, Entity)> = query
        .iter()
        .map(|t| { (*t.0, t.1)})
        .collect();

    // calculate the new position for each enemy
    for (mut t1, e1) in query.iter_mut() {
        let (mut separation_direction, mut cohesion_direction) = (Vec3::ZERO, Vec3::ZERO);
        for (t2, e2) in vec_transforms.iter() {
            // skip self
            if e1 == *e2 {
                continue;
            }
            // separation
            let distance = (t1.translation - t2.translation).length();
            if distance < 1.5 {
                let direction = (t1.translation - t2.translation).normalize();
                separation_direction += direction;
                separation_direction = separation_direction.normalize();
            }
            // cohesion
            let distance = (t1.translation - t2.translation).length();
            if distance < 6. {
                let direction = (t2.translation - t1.translation).normalize();
                cohesion_direction += direction;
                cohesion_direction = cohesion_direction.normalize();
            }
            // alignment
            // not needed for now since velocity is always the same
        }
        // move towards the origin
        let origin_direction = (Vec3::Y * 0.5 - t1.translation).normalize();

        // combine all the vectors 
        let move_vec = (
            5. * origin_direction + 
            1. * separation_direction + 
            1. * cohesion_direction
        ).normalize();
        
        // apply changes
        t1.translation += move_vec.normalize() * 0.5 * time.delta_seconds();
    }
}

pub fn index_enemies(
    quad_tree: Res<quad_tree::QuadTree>,
    mut query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.x.is_nan() || transform.translation.z.is_nan() {
            continue;
        }
        for (dx, dz) in &[
            (-0.5, -0.5),
            (0.5, -0.5),
            (-0.5, 0.5),
            (0.5, 0.5),
        ] {
            let tmp = Vec2::new(transform.translation.x + dx, transform.translation.z + dz);
            if quad_tree.contains(tmp) {
                quad_tree.submit_for_insert(entity, tmp).unwrap();
                // quad_tree.insert(
                //     entity,
                //     Vec2::new(transform.translation.x + dx, transform.translation.z + dz),
                // ).unwrap();
            }
        }
    }
}

/// This is the main function that handles the collision detection and damage taking.
/// The primary pattern being followed here (for components):
pub fn take_damage(
    mut commands: Commands,
    quad_tree: Res<quad_tree::QuadTree>,
    mut enemy_query: Query<(Entity, &mut health::HealthPoints, &Transform), With<Enemy>>,
    orb_query: Query<(Entity, &Transform, Option<&tower::Damage>), With<orb::Orb>>,
) {
    let mut enemies_to_despawn = HashSet::new();
    let mut orbs_to_despawn = HashSet::new();
    let mut enemies: HashMap<Entity, (&Transform, Mut<health::HealthPoints>)> = HashMap::new();
    let mut orbs: HashMap<Entity, (&Transform, Option<&tower::Damage>)> = HashMap::new();

    enemy_query
        .iter_mut()
        .for_each(|(entity, healthpoints, transform)| {
            enemies.insert(entity, (transform, healthpoints));
        });
    orb_query.iter().for_each(|(entity, transform, damage)| {
        orbs.insert(entity, (transform, damage));
    });

    for node in quad_tree.get_leaf_nodes() {
        match node.objects {
            Some(obj_refs) => {
                for (enemy, (enemy_transform, enemy_hp)) in
                    enemies.iter_mut().filter(|(e, _)| obj_refs.contains(e))
                {
                    for (orb, (orb_transform, damage)) in
                        orbs.iter_mut().filter(|(o, _)| obj_refs.contains(o))
                    {
                        let distance =
                            (orb_transform.translation - enemy_transform.translation).length();
                        if distance < 0.5 {
                            orbs_to_despawn.insert(*orb);

                            // Depending on the effects of the orb, we can do different things here.
                            if let Some(damage) = damage {
                                enemy_hp.hp = enemy_hp.hp.saturating_sub(damage.hp);
                            }
                        }
                    }

                    if enemy_hp.hp == 0 {
                        enemies_to_despawn.insert(*enemy);
                    }
                }
            }
            None => {}
        }
    }

    for enemy in enemies_to_despawn {
        commands.entity(enemy).despawn_recursive();
    }
    for orb in orbs_to_despawn {
        commands.entity(orb).despawn_recursive();
    }
}

pub fn take_damage_n2(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &mut health::HealthPoints, &Transform), With<Enemy>>,
    orb_query: Query<(Entity, &Transform, Option<&tower::Damage>), With<orb::Orb>>,
) {
    let mut enemies_to_despawn = HashSet::new();
    let mut orbs_to_despawn = HashSet::new();
    let mut enemies: HashMap<Entity, (&Transform, Mut<health::HealthPoints>)> = HashMap::new();
    let mut orbs: HashMap<Entity, (&Transform, Option<&tower::Damage>)> = HashMap::new();

    enemy_query
        .iter_mut()
        .for_each(|(entity, healthpoints, transform)| {
            enemies.insert(entity, (transform, healthpoints));
        });
    orb_query.iter().for_each(|(entity, transform, damage)| {
        orbs.insert(entity, (transform, damage));
    });

    for (enemy, (enemy_transform, enemy_hp)) in enemies.iter_mut() {
        for (orb, (orb_transform, damage)) in orbs.iter_mut() {
            let distance =
                (orb_transform.translation - enemy_transform.translation).length();
            if distance < 0.5 {
                orbs_to_despawn.insert(*orb);

                // Depending on the effects of the orb, we can do different things here.
                if let Some(damage) = damage {
                    enemy_hp.hp = enemy_hp.hp.saturating_sub(damage.hp);
                }
            }
        }
        if enemy_hp.hp == 0 {
            enemies_to_despawn.insert(*enemy);
        }
    }

    for enemy in enemies_to_despawn {
        commands.entity(enemy).despawn_recursive();
    }
    for orb in orbs_to_despawn {
        commands.entity(orb).despawn_recursive();
    }
}