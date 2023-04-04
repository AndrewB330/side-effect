use crate::core::materials::player_material::PlayerMaterial;
use crate::core::objects::player::Player;
use crate::core::objects::player::PlayerState;
use crate::core::objects::shape::{PlayerShape, PlayerShapeVisualBundle, MAX_SIDES};
use crate::core::objects::side_effect::SideEffect;
use crate::core::scene_builder::SceneBuilder;
use bevy::prelude::*;
use std::f32::consts::PI;

use crate::core::objects::collision_groups::PLAYER_CG;
use bevy_rapier2d::prelude::*;

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
        let mut player = Player {
            id,
            effects: [SideEffect::None; MAX_SIDES],
            side_entities: [None; MAX_SIDES],
            small_collider: Collider::round_cuboid(0.4, 0.4, 0.075),
            ..default()
        };

        let player_material = PlayerMaterial {
            color: Color::WHITE,
            effect_index: [0; MAX_SIDES],
            texture: Some(self.asset_server.load("images/square.png")),
            emissive: None,
            overlay: None,
            player_effect_texture: self.asset_server.load("images/effect.png"),
        };

        let parent = self.commands.spawn({}).id();

        let directions = player.get_side_directions();

        for i in 0..MAX_SIDES {
            let child = self
                .commands
                .spawn((
                    VisibilityBundle::default(),
                    TransformBundle::from_transform(
                        Transform::from_translation(directions[i].extend(0.0) * 0.475)
                            .with_rotation(Quat::from_axis_angle(Vec3::Z, PI * 0.5 * i as f32)),
                    ),
                    Collider::round_cuboid(0.45, 0.01, 0.01),
                    Friction {
                        coefficient: 0.3,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    Restitution {
                        coefficient: 0.0,
                        combine_rule: CoefficientCombineRule::Max,
                    },
                    PLAYER_CG,
                ))
                .id();
            player.side_entities[i] = Some(child);
            self.commands.entity(parent).add_child(child);
        }

        self.commands.entity(parent).insert(PlayerBundle {
            player,
            rigid_body: RigidBody::Dynamic,
            velocity: Default::default(),
            axes: LockedAxes::empty(),
            impulse: Default::default(),
            mass: Default::default(),
            collision_groups: PLAYER_CG,
            friction: Friction {
                coefficient: 0.3,
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
                    .add(shape::Quad::new(Vec2::new(2.0, 2.0)).into())
                    .into(),
                material: self.player_materials.add(player_material),
                collider: Collider::round_cuboid(0.4, 0.4, 0.075),
            },
        });
    }
}
