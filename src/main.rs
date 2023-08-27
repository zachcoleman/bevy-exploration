use bevy::prelude::*;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
// use bevy_framepace::FramepacePlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::*;

mod assets;
mod camera;
mod enemy;
mod grid;
mod health;
mod map;
mod orb;
mod quad_tree;
mod tower;

fn main() {
    App::new()
        // default plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy-exploration".to_string(),
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }))

        // resources
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(AmbientLight {
            brightness: 0.5,
            color: Color::rgb(1., 1., 1.),
        })

        // startup systems
        .add_startup_system(assets::asset_loading.in_base_set(StartupSet::PreStartup))
        .add_startup_system(quad_tree::add_quad_tree.in_base_set(StartupSet::PreStartup))
        .add_startup_systems(
            (
                map::spawn_basic_scene,
                grid::spawn_grid,
                apply_system_buffers,
                tower::spawn_default_towers,
                quad_tree::visualize_quad_tree_leaves,
            ).chain(),
        )

        /*** CoreStage::Update ***/
        // third party plugins
        // .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(FramepacePlugin)
        .add_plugins(DefaultPickingPlugins)

        // my plugins
        .add_plugin(camera::CameraPlayerPlugin)
        .add_plugin(health::HealthPointsPlugin)
        .add_plugin(enemy::EnemyPlugin)
        .add_plugin(tower::TowerPlugin)
        .add_plugin(orb::OrbPlugin)

        // core systems
        .add_systems(
            (
                enemy::index_enemies,
                orb::index_orbs,
                quad_tree::update_leaf_node_color,
                enemy::take_damage,
                orb::despawn_reach_ground,
                quad_tree::clear_quad_tree,
            ).chain()
        )
        .run();
}
