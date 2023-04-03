use crate::core::materials::player_material::PlayerMaterial;
use crate::core::objects::shape::ShapePlugin;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier2d::prelude::*;
use objects::player::PlayerPlugin;

use self::objects::bonus::BonusPlugin;

pub mod direction;
pub mod materials;
pub mod objects;
pub mod scene_builder;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 30.0,
                time_scale: 1.0,
                substeps: 5,
            },
            gravity: Vec2::NEG_Y * 9.8,
            ..default()
        });

        app.add_plugin(DebugLinesPlugin::default());
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
        app.add_plugin(Material2dPlugin::<PlayerMaterial>::default());
        app.add_plugin(PlayerPlugin);
        app.add_plugin(ShapePlugin);
        app.add_plugin(BonusPlugin);
    }
}
