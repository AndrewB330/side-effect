use crate::core::materials::player_material::PlayerMaterial;
use crate::core::objects::shape::PlayerShapeVisualBundleCache;
use bevy::prelude::*;

mod bonus;
mod player;
mod wall;

pub struct SceneBuilder<'w, 's, 'a> {
    commands: Commands<'w, 's>,
    meshes: ResMut<'a, Assets<Mesh>>,
    materials: ResMut<'a, Assets<ColorMaterial>>,
    player_materials: ResMut<'a, Assets<PlayerMaterial>>,
    psv: ResMut<'a, PlayerShapeVisualBundleCache>,
    asset_server: Res<'a, AssetServer>,
}

impl<'w, 's, 'a> SceneBuilder<'w, 's, 'a> {
    pub const BONUS_DEPTH: f32 = 0.6;
    pub const PLAYER_DEPTH: f32 = 0.5;
    pub const WALL_DEPTH: f32 = 0.2;

    pub fn new(
        commands: Commands<'w, 's>,
        asset_server: Res<'a, AssetServer>,
        meshes: ResMut<'a, Assets<Mesh>>,
        materials: ResMut<'a, Assets<ColorMaterial>>,
        player_materials: ResMut<'a, Assets<PlayerMaterial>>,
        psv: ResMut<'a, PlayerShapeVisualBundleCache>,
    ) -> SceneBuilder<'w, 's, 'a> {
        SceneBuilder {
            commands,
            meshes,
            materials,
            player_materials,
            psv,
            asset_server,
        }
    }
}
