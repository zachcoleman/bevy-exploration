use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub hexagon_mesh: Handle<Mesh>,
    pub tower_mesh: Handle<Mesh>,
    pub wall_mesh: Handle<Mesh>,
}

pub fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        hexagon_mesh: assets.load("models/hexagon.glb#Mesh0/Primitive0"),
        tower_mesh: assets.load("models/tower.glb#Mesh0/Primitive0"),
        wall_mesh: assets.load("models/wall.glb#Mesh0/Primitive0"),
    });
}
