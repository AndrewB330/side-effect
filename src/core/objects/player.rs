use std::f32::consts::PI;
use crate::core::direction::SceneDirection;

use crate::states::GameWorldState;
use bevy::prelude::*;

use crate::core::objects::shape::{PlayerShape, PlayerShapeVisualBundleCache, MAX_SIDES};
use crate::core::objects::side_effect::SideEffect;
use crate::core::objects::collision_groups::PLAYER_CG;

use bevy_rapier2d::prelude::*;
use crate::core::materials::player_material::PlayerMaterial;

#[derive(Component, Clone, Debug, Default)]
pub struct Player {
    pub id: u32,
    pub current_shape_index: usize,
    pub time_since_move: f32,
    pub time_since_spin: f32,
    pub time_since_landed: f32,
    pub time_since_in_air: f32,
    pub time_since_jump: f32,
    pub available_shapes: Vec<PlayerShape>,
    pub effects: [SideEffect; MAX_SIDES],
    pub small_collider: Collider,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            move_player.run_if(in_state(GameWorldState::GameWorld)),
            jump_player.run_if(in_state(GameWorldState::GameWorld)),
            update_side_effects.run_if(in_state(GameWorldState::GameWorld)),
        ));
    }
}

impl Player {
    pub fn get_current_shape(&self) -> PlayerShape {
        self.available_shapes[self.current_shape_index]
    }

    pub fn get_max_speed(&self) -> f32 {
        2.0
    }

    pub fn get_max_angular_speed(&self) -> f32 {
        3.0
    }

    pub fn get_max_acceleration(&self) -> f32 {
        9.0
    }

    pub fn get_max_angular_acceleration(&self) -> f32 {
        35.0
    }

    pub fn get_jump_impulse(&self) -> f32 {
        4.0
    }

    pub fn get_friction(&self) -> f32 {
        0.2
    }

    pub fn get_center_offset(&self) -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    pub fn get_side_centers(&self) -> Vec<Vec2> {
        return vec![Vec2::NEG_Y * 0.5, Vec2::X * 0.5, Vec2::Y * 0.5, Vec2::NEG_X * 0.5]
    }

    pub fn get_side_directions(&self) -> Vec<Vec2> {
        return vec![Vec2::NEG_Y, Vec2::X, Vec2::Y, Vec2::NEG_X]
    }
}

fn update_side_effects(
    mut players: Query<(&Player, &Handle<PlayerMaterial>), Changed<Player>>,
    mut materials: ResMut<Assets<PlayerMaterial>>,
) {
    for (player, handle) in &players {
        if let Some(material) = materials.get_mut(handle) {
            for i in 0..MAX_SIDES {
                material.effect_index[i as usize] = player.effects[i as usize].to_index();
            }
        }
    }
}

fn change_shape(
    mut commands: Commands,
    mut players: Query<(Entity, &mut Player, &mut Friction)>,
    keys: Res<Input<KeyCode>>,
    meshes_cache: Res<PlayerShapeVisualBundleCache>,
) {
    for (entity, mut player, mut friction) in players.iter_mut() {
        if keys.just_pressed(KeyCode::E) {
            player.current_shape_index =
                (player.current_shape_index + 1) % player.available_shapes.len();
            friction.coefficient = player.get_friction();
            commands.entity(entity).insert(
                meshes_cache
                    .cache
                    .get(&player.available_shapes[player.current_shape_index])
                    .unwrap()
                    .clone(),
            );
        }
    }
}

fn move_player(
    mut players: Query<(
        Entity,
        &mut ExternalImpulse,
        &Velocity,
        &ReadMassProperties,
        &mut Player,
        &Transform,
    )>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    config: ResMut<RapierConfiguration>,
    context: Res<RapierContext>,
) {
    let gravity_direction = SceneDirection::from_gravity_direction(&config);

    for (entity, mut impulse, velocity, mass, mut player, transform) in players.iter_mut() {
        /// Move left-right
        {
            player.time_since_move += time.delta_seconds();
            let mut target_velocity = 0.0;
            let right = gravity_direction.get_vec().perp();

            if keys.any_pressed([KeyCode::A]) {
                player.time_since_move = 0.0;
                target_velocity -= player.get_max_speed();
            }

            if keys.any_pressed([KeyCode::D]) {
                player.time_since_move = 0.0;
                target_velocity += player.get_max_speed();
            }

            let delta_velocity = target_velocity - velocity.linvel.dot(right);
            let k1 = ((delta_velocity.abs() - player.get_max_speed() * 1.0).max(0.0)
                / player.get_max_speed())
                .clamp(0.0, 2.0);

            let dv = delta_velocity
                .abs()
                .min(player.get_max_acceleration() * time.delta_seconds() * (1.0 + k1));

            let move_force = if player.time_since_move < 0.2 {
                1.0
            } else {
                0.0// 1.0 / (5.0 + player.time_since_move * 5.0)
            };

            let add_impulse = ExternalImpulse::at_point(
                right * delta_velocity.signum() * dv * mass.0.mass,
                transform.translation.truncate() + player.get_center_offset(),
                transform.translation.truncate(),
            );

            impulse.impulse += add_impulse.impulse * move_force;
            impulse.torque_impulse += add_impulse.torque_impulse * move_force;
        }

        /// Spin clockwise-counterclockwise
        {
            player.time_since_spin += time.delta_seconds();
            let mut target_angular_velocity = 0.0;

            if keys.any_pressed([KeyCode::W]) {
                player.time_since_spin = 0.0;
                target_angular_velocity += player.get_max_angular_speed();
            }

            if keys.any_pressed([KeyCode::S]) {
                player.time_since_spin = 0.0;
                target_angular_velocity -= player.get_max_angular_speed();
            }

            let delta_angular_velocity = target_angular_velocity - velocity.angvel;
            let k2 = ((delta_angular_velocity.abs() - player.get_max_angular_speed() * 1.0).max(0.0)
                / player.get_max_angular_speed())
                .clamp(0.0, 2.0);

            let dv_angular = delta_angular_velocity
                .abs()
                .min(player.get_max_angular_acceleration() * time.delta_seconds() * (1.0 + k2));

            let spin_force = if player.time_since_spin == 0.0 {
                1.0
            } else {
                0.0//1.0 / (5.0 + player.time_since_spin * 5.0)
            };

            impulse.torque_impulse += delta_angular_velocity.signum() * dv_angular * mass.0.principal_inertia * spin_force;
        }

        /// Snap to 90-s angles
        if player.time_since_landed < 0.2 || player.time_since_in_air > 0.05 {
            let angle: f32 = (get_angle_from_quat(transform.rotation) % (2.0 * PI) + 2.0 * PI) % (2.0 * PI);
            let target_angle = (angle / (PI * 0.5)).round() * (PI * 0.5);

            let delta_angle = target_angle - angle;


            let dv_angular = delta_angle
                .abs()
                .min(player.get_max_angular_acceleration() * time.delta_seconds());

            let spin_force = if player.time_since_spin > 0.1 {
                1.0
            } else {
                0.0
            };

            impulse.torque_impulse += delta_angle.signum() * dv_angular * mass.0.principal_inertia * spin_force;
            impulse.torque_impulse -= 0.1 * velocity.angvel * mass.0.principal_inertia * spin_force;
        }

        for i in 0..MAX_SIDES {
            if player.effects[i] == SideEffect::Sticky {
                let dir = Vec2::from_angle(get_angle_from_quat(transform.rotation)).rotate(player.get_side_directions()[i]);
                let collider_nearby = find_obstacle(
                    entity,
                    &player.small_collider,
                    dir,
                    transform,
                    &context,
                );

                if let Some((e, toi)) = collider_nearby {
                    if toi < 0.05 && player.time_since_jump > 0.05 {
                        impulse.impulse += dir * 1.0;
                    }
                }
            }
        }

    }
}

fn get_angle_from_quat(q: Quat) -> f32 {
    let (axis, angle) = q.to_axis_angle();
    if axis.dot(Vec3::Z) < 0.0 {
        -angle
    } else {
        angle
    }
}

fn find_obstacle(
    entity: Entity,
    collider: &Collider,
    direction: Vec2,
    transform: &Transform,
    context: &RapierContext,
) -> Option<(Entity, f32)> {
    let filter =  QueryFilter::new().groups(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1)).exclude_collider(entity);
    context
        .cast_shape(
            transform.translation.truncate(),
            get_angle_from_quat(transform.rotation),
            direction,
            collider,
            100.0,
            filter,
        )
        .map(|(e, toi)| (e, toi.toi))
}

fn jump_player(
    mut players: Query<(
        Entity,
        &mut ExternalImpulse,
        &Collider,
        &mut Player,
        &Velocity,
        &Transform,
        &ReadMassProperties,
    )>,
    context: Res<RapierContext>,
    keys: Res<Input<KeyCode>>,
    config: Res<RapierConfiguration>,
    time: Res<Time>,
) {
    let gravity_direction = SceneDirection::from_gravity_direction(&config);

    for (entity, mut ext_impulse, collider, mut player, velocity, transform, mass) in players.iter_mut()
    {
        player.time_since_jump += time.delta_seconds();
        let collider_below = find_obstacle(
            entity,
            &player.small_collider,
            gravity_direction.get_vec(),
            transform,
            &context,
        );

        if let Some((_, dist)) = collider_below {
            if dist < 0.05 {//} && velocity.linvel.dot(gravity_direction.get_vec()).abs() < 1.5 && velocity.angvel.abs() < 0.8 {
                player.time_since_landed += time.delta_seconds();
            } else {
                player.time_since_landed = 0.0;
            }
            player.time_since_in_air = 0.0;
        } else {
            player.time_since_landed = 0.0;
            player.time_since_in_air += time.delta_seconds();
        }

        if keys.any_pressed([KeyCode::Space]) {
            if player.time_since_landed > 0.2 {
                player.time_since_landed = 0.0;
                player.time_since_jump = 0.0;
                ext_impulse.impulse +=
                    -gravity_direction.get_vec() * player.get_jump_impulse() * mass.0.mass;
            }
        }
    }
}
