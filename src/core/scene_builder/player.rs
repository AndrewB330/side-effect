use crate::core::materials::player_material::PlayerMaterial;
use crate::core::objects::player::Player;
use crate::core::objects::shape::{PlayerShape, PlayerShapeVisualBundle, MAX_SIDES};
use crate::core::objects::side_effect::SideEffect;
use crate::core::scene_builder::SceneBuilder;
use bevy::prelude::*;

use bevy_rapier2d::prelude::*;
use crate::core::objects::collision_groups::PLAYER_CG;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    rigid_body: RigidBody,
    velocity: Velocity,
    axes: LockedAxes,
    impulse: ExternalImpulse,
    mass: ReadMassProperties,
    collision_groups: CollisionGroups,
    friction: Friction,
    #[bundle]
    visibility: VisibilityBundle,
    #[bundle]
    transform: TransformBundle,
    density: ColliderMassProperties,
    #[bundle]
    visual: PlayerShapeVisualBundle,
}

impl<'w, 's, 'a> SceneBuilder<'w, 's, 'a> {
    pub fn spawn_player(&mut self, position: Vec2, id: u32) {
        let player = Player {
            id,
            current_shape_index: 0,
            time_since_move: 0.0,
            time_since_spin: 0.0,
            time_since_landed: 0.0,
            time_since_in_air: 0.0,
            time_since_jump: 0.0,
            available_shapes: vec![PlayerShape::Square],
            effects: [SideEffect::None; MAX_SIDES],
            small_collider: Collider::round_cuboid(0.4, 0.4, 0.075),
        };

        let player_material = PlayerMaterial {
            color: Color::WHITE,
            effect_index: [0; MAX_SIDES],
            texture: Some(self.asset_server.load("images/square.png")),
            emissive: None,
            overlay: None,
            player_effect_texture: self.asset_server.load("images/effect.png"),
        };

        let parent = self.commands.spawn(PlayerBundle {
            player,
            rigid_body: RigidBody::Dynamic,
            velocity: Default::default(),
            axes: LockedAxes::empty(),
            impulse: Default::default(),
            mass: Default::default(),
            collision_groups: PLAYER_CG,
            friction: Friction {
                coefficient: 0.9,
                combine_rule: CoefficientCombineRule::Max,
            },
            visibility: VisibilityBundle::default(),
            transform: TransformBundle::from_transform(Transform::from_xyz(
                position.x,
                position.y,
                SceneBuilder::PLAYER_DEPTH,
            )),
            density: ColliderMassProperties::Density(1.0),
            visual: PlayerShapeVisualBundle {
                mesh: self
                    .meshes
                    .add(PlayerShape::Square.get_default_mesh())
                    .into(),
                material: self.player_materials.add(player_material),
                collider: Collider::round_cuboid(0.4, 0.4, 0.1),
            },
        }).id();

        let child = self.commands.spawn((
             VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_xyz(
                0.0,
                0.5,
                0.0,
            )),
            Collider::round_cuboid(0.4, 0.01, 0.05),
             Friction {
                coefficient: 0.01,
                combine_rule: CoefficientCombineRule::Min,
            },
             PLAYER_CG
        )).id();

        self.commands.entity(parent).add_child(child);
    }
}
