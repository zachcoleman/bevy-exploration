use bevy::prelude::*;

use crate::enemy;

pub struct HealthPointsPlugin;
impl Plugin for HealthPointsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HealthPoints>()
            .register_type::<Regen>()
            .add_system(regen_health)
            .add_system(update_alpha);
    }
}

#[derive(Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct HealthPoints {
    pub hp: usize,
    pub max_hp: usize,
}

#[derive(Clone, Component, Default, Reflect)]
#[reflect(Component)]
pub struct Regen {
    pub hp: usize,
    pub timer: Timer,
}

pub fn regen_health(time: Res<Time>, mut query: Query<(&mut HealthPoints, &mut Regen)>) {
    for (mut hp, mut regen) in query.iter_mut() {
        regen.timer.tick(time.delta());
        if regen.timer.just_finished() && hp.hp < hp.max_hp {
            hp.hp += regen.hp;
        }
    }
}

pub fn update_alpha(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&HealthPoints, &Handle<StandardMaterial>), With<enemy::Enemy>>,
) {
    for (hp, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material
                .base_color
                .set_a(hp.hp as f32 / (2. * hp.max_hp as f32) + 0.5);
        }
    }
}
