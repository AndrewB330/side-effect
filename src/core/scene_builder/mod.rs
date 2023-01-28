use bevy::prelude::*;

mod player;
mod wall;

pub struct SceneBuilder<'w, 's, 'a> {
    commands: Commands<'w, 's>,
    meshes: ResMut<'a, Assets<Mesh>>,
    materials: ResMut<'a, Assets<ColorMaterial>>,
}

impl<'w, 's, 'a> SceneBuilder<'w, 's, 'a> {
    pub const PLAYER_DEPTH: f32 = 0.5;
    pub const WALL_DEPTH: f32 = 0.2;

    pub fn new(
        commands: Commands<'w, 's>,
        meshes: ResMut<'a, Assets<Mesh>>,
        materials: ResMut<'a, Assets<ColorMaterial>>,
    ) -> SceneBuilder<'w, 's, 'a> {
        SceneBuilder {
            commands,
            meshes,
            materials,
        }
    }
}
