use bevy::prelude::*;

use crate::quad_tree;

pub struct OrbPlugin;
impl Plugin for OrbPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Orb>().add_system(move_orb);
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Orb {
    pub direction: Vec3,
    pub target: Vec3,
    pub speed: f32,
}

pub fn move_orb(time: Res<Time>, mut query: Query<(&mut Transform, &Orb)>) {
    for (mut transform, orb) in query.iter_mut() {
        transform.translation += orb.direction * orb.speed * time.delta_seconds();
    }
}

pub fn index_orbs(
    quad_tree: Res<quad_tree::QuadTree>,
    mut query: Query<(Entity, &Transform), With<Orb>>,
) {
    for (entity, transform) in query.iter_mut() {
        let position = Vec2::new(transform.translation.x, transform.translation.z);
        if position.x.is_nan() || position.y.is_nan() {
            continue;
        }
        if quad_tree.contains(position){
            // quad_tree.insert(entity, position).unwrap();
            quad_tree.submit_for_insert(entity, position).unwrap();
        }
    }
}


pub fn despawn_reach_ground(mut commands: Commands, query: Query<(Entity, &Transform, &Orb)>) {
    for (entity, transform, _orb) in query.iter() {
        // old: despawn if orb reaches target
        // let distance = (orb.target - transform.translation).length();
        // if distance < 0.1 {
        //     if let Some(ec) = commands.get_entity(entity) {
        //         ec.despawn_recursive();
        //     }
        // }

        // despawn if orb is below the ground
        if transform.translation.y < 0. {
            if let Some(ec) = commands.get_entity(entity) {
                ec.despawn_recursive();
            }
        }
    }
}
