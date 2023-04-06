use crate::core::scene_builder::scene_boundaries::SceneBoundaries;
use crate::core::scene_builder::SceneBuilder;
use crate::core::CorePlugin;
use crate::states::GameWorldState;
use bevy::core_pipeline::bloom::BloomSettings;

use bevy::prelude::*;

use crate::core::materials::player_material::PlayerMaterial;
use crate::core::objects::shape::PlayerShapeVisualBundleCache;
use crate::core::objects::side_effect::SideEffect;
use bevy::core_pipeline::tonemapping::Tonemapping;

use crate::core::materials::bonus_material::BonusMaterial;
use crate::core::materials::monster_material::MonsterMaterial;
use std::env;

pub struct SideEffectGamePlugin;

impl Plugin for SideEffectGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameWorldState>();
        app.add_plugin(CorePlugin);

        if env::var("LOCAL_BUILD") == Ok("2".to_string()) {
            app.add_startup_system(setup_dev);
        }

        app.add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            transform: Transform::from_xyz(0.0, 0.0, 1000.0 - 0.1)
                .with_scale(Vec3::new(0.008, 0.008, 1.0)),
            ..default()
        })
        .insert(BloomSettings::default());
}

fn setup_dev(
    commands: Commands,
    asset_server: Res<AssetServer>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    player_materials: ResMut<Assets<PlayerMaterial>>,
    bonus_materials: ResMut<Assets<BonusMaterial>>,
    monster_materials: ResMut<Assets<MonsterMaterial>>,
    boundaries: ResMut<SceneBoundaries>,
    psv: ResMut<PlayerShapeVisualBundleCache>,
    mut next_game_state: ResMut<NextState<GameWorldState>>,
) {
    let mut scene_builder = SceneBuilder::new(
        commands,
        asset_server,
        meshes,
        materials,
        player_materials,
        bonus_materials,
        monster_materials,
        boundaries,
        psv,
    );

    scene_builder.spawn_player(Vec2::new(0.0, 1.0), 0);
    scene_builder.spawn_effect_bonus(Vec2::new(2.0, -1.75), SideEffect::Sticky);
    scene_builder.spawn_effect_bonus(Vec2::new(3.0, -1.75), SideEffect::Shield);
    scene_builder.spawn_effect_bonus(Vec2::new(4.0, -1.75), SideEffect::Slippery);
    scene_builder.spawn_effect_bonus(Vec2::new(5.0, -1.75), SideEffect::Shield);
    /*scene_builder.spawn_effect_bonus(Vec2::new(2.0, -1.2), SideEffect::Thorns);
    scene_builder.spawn_effect_bonus(Vec2::new(3.0, -1.2), SideEffect::Flashlight);
    scene_builder.spawn_effect_bonus(Vec2::new(4.0, -1.2), SideEffect::Laser);
    scene_builder.spawn_effect_bonus(Vec2::new(5.0, -1.2), SideEffect::Spring);*/

    scene_builder.spawn_monster(Vec2::new(2.0, 1.2));

    scene_builder.spawn_wall_from_to(Vec2::new(-5.0, -2.0), Vec2::new(5.0, -3.0));
    //scene_builder.spawn_wall_from_to(Vec2::new(2.0, 0.0), Vec2::new(5.0, -0.5));
    scene_builder.spawn_wall_from_to(Vec2::new(-6.0, -2.0), Vec2::new(-5.0, 2.0));
    scene_builder.spawn_wall_from_to(Vec2::new(5.0, -2.0), Vec2::new(6.0, 2.0));
    scene_builder.spawn_wall_from_to(Vec2::new(2.0, -1.0), Vec2::new(3.0, -2.0));

    next_game_state.set(GameWorldState::GameWorld);
}
