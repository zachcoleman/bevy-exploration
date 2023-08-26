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
    pub target: Vec3,
    pub speed: f32,
}

pub fn move_orb(time: Res<Time>, mut query: Query<(&mut Transform, &Orb)>) {
    for (mut transform, orb) in query.iter_mut() {
        let direction = (orb.target - transform.translation).normalize();
        transform.translation += direction * orb.speed * time.delta_seconds();
    }
}

pub fn index_orbs(
    mut quad_tree: ResMut<quad_tree::QuadTree>,
    mut query: Query<(Entity, &Transform), With<Orb>>,
) {
    for (entity, transform) in query.iter_mut() {
        if transform.translation.x.is_nan() || transform.translation.z.is_nan() {
            continue;
        }
        quad_tree
            .insert(
                entity,
                Vec2::new(transform.translation.x, transform.translation.z),
            )
            .unwrap();
    }
}


pub fn despawn_reach_target(mut commands: Commands, query: Query<(Entity, &Transform, &Orb)>) {
    for (entity, transform, orb) in query.iter() {
        let distance = (orb.target - transform.translation).length();
        if distance < 0.1 {
            if let Some(ec) = commands.get_entity(entity) {
                ec.despawn_recursive();
            }
        }
    }
}
