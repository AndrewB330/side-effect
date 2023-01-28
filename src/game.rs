use crate::core::scene_builder::SceneBuilder;
use crate::core::CorePlugin;
use crate::states::GameWorldState;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use std::env;

pub struct ShiftsNShapesPlugin;

impl Plugin for ShiftsNShapesPlugin {
    fn build(&self, app: &mut App) {
        if env::var("LOCAL_BUILD") == Ok("2".to_string()) {
            app.add_state(GameWorldState::GameWorld);
            app.add_startup_system(setup_dev);
        } else {
            app.add_state(GameWorldState::None);
        }

        app.add_startup_system(setup_camera);
        app.add_plugin(CorePlugin);
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1000.0 - 0.1)
                .with_scale(Vec3::new(0.01, 0.01, 1.0)),
            ..default()
        })
        .insert(BloomSettings::default());
}

fn setup_dev(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut scene_builder = SceneBuilder::new(commands, meshes, materials);

    scene_builder.spawn_player(Vec2::new(0.0, 1.0), 0);
    scene_builder.spawn_wall_from_to(Vec2::new(-2.0, -2.0), Vec2::new(5.0, -3.0));
}

/*fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(2.25, 2.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(2.25, 2.25, 0.75))),
        transform: Transform::from_translation(Vec3::new(-100., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(100., 0., 0.)),
        ..default()
    });
}*/
