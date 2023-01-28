use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use objects::player::PlayerPlugin;

pub mod objects;
pub mod scene_builder;
pub mod direction;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugin(PlayerPlugin);

        app.insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 30.0,
                time_scale: 1.0,
                substeps: 1,
            },
            gravity: Vec2::NEG_Y * 9.8,
            ..default()
        });
    }
}
