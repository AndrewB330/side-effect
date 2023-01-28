use crate::core::objects::player::{Player, Shape};
use crate::core::scene_builder::SceneBuilder;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy_rapier2d::prelude::*;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    player: Player,
    collider: Collider,
    mesh: Mesh2dHandle,
    rigid_body: RigidBody,
    velocity: Velocity,
    axes: LockedAxes,
    impulse: ExternalImpulse,
    mass: ReadMassProperties,
    collision_groups: CollisionGroups,
    friction: Friction,
    material: Handle<ColorMaterial>,
    #[bundle]
    visibility: VisibilityBundle,
    #[bundle]
    transform: TransformBundle,
    density: ColliderMassProperties,
}

impl<'w, 's, 'a> SceneBuilder<'w, 's, 'a> {
    pub fn spawn_player(&mut self, position: Vec2, id: u32) {
        let player = Player {
            id,
            shape: Shape::Square,
            max_speed: 2.,
            max_acceleration: 18.0,
            jump_impulse: 6.,
        };

        self.commands.spawn(PlayerBundle {
            player,
            collider: Collider::cuboid(0.5, 0.5),
            mesh: self.meshes
                .add(shape::Quad::new(Vec2::new(1.0, 1.0)).into())
                .into(),
            material: self.materials.add(Color::rgb_u8(33, 41, 128).into()),
            rigid_body: RigidBody::Dynamic,
            axes: LockedAxes::ROTATION_LOCKED,
            friction: Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            transform: TransformBundle::from_transform(Transform::from_xyz(
                position.x,
                position.y,
                SceneBuilder::PLAYER_DEPTH,
            )),
            density: ColliderMassProperties::Density(1.0),
            ..default()
        });
    }
}
