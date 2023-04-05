use crate::core::direction::SceneDirection;
use std::f32::consts::PI;

use crate::states::GameWorldState;
use bevy::prelude::*;



use crate::core::objects::shape::{MAX_SIDES};
use crate::core::objects::side_effect::SideEffect;

use crate::core::materials::player_material::PlayerMaterial;
use bevy_rapier2d::prelude::*;

#[derive(Component, Clone, Debug, Default)]
pub struct Player {
    pub id: u32,

    pub moving_state: PlayerState, // Player presses A or D

    pub landed_state: PlayerState,
    // Player has something below
    pub in_air_state: PlayerState, // Player in air - nothing below (no stick?)

    pub stick_to_wall_state: PlayerState,
    pub stick_to_anything_state: PlayerState,

    pub slippery_below_state: PlayerState,

    pub time_since_last_spin: f32,
    pub time_since_last_jump: f32,

    pub delayed_spin_torque: Option<f32>,

    pub effects: [SideEffect; MAX_SIDES],
    pub side_entities: [Option<Entity>; MAX_SIDES],
    pub small_collider: Collider,
}

#[derive(Default, Debug, Clone)]
pub struct PlayerState {
    pub is_active: bool,
    pub time_since_changed: f32,
}

impl PlayerState {
    pub fn advance(&mut self, time: &Time) {
        self.time_since_changed += time.delta_seconds();
    }

    pub fn set(&mut self, active: bool) {
        if active {
            self.activate();
        } else {
            self.deactivate();
        }
    }

    pub fn activate(&mut self) {
        if !self.is_active {
            self.is_active = true;
            self.time_since_changed = 0.0;
        }
    }

    pub fn deactivate(&mut self) {
        if self.is_active {
            self.is_active = false;
            self.time_since_changed = 0.0;
        }
    }

    pub fn time_since_activated(&self) -> f32 {
        if self.is_active {
            self.time_since_changed
        } else {
            0.0
        }
    }

    pub fn time_since_deactivated(&self) -> f32 {
        if self.is_active {
            0.0
        } else {
            self.time_since_changed
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            move_player.run_if(in_state(GameWorldState::GameWorld)),
            update_side_effects.run_if(in_state(GameWorldState::GameWorld)),
            update_side_entities.run_if(in_state(GameWorldState::GameWorld)),
        ));
    }
}

impl Player {
    pub fn get_max_speed(&self) -> f32 {
        if self.slippery_below_state.is_active {
            4.0
        } else {
            2.5
        }
    }

    pub fn get_max_angular_speed(&self) -> f32 {
        3.0
    }

    pub fn get_max_acceleration(&self) -> f32 {
        14.0
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
        return vec![
            Vec2::NEG_Y * 0.5,
            Vec2::X * 0.5,
            Vec2::Y * 0.5,
            Vec2::NEG_X * 0.5,
        ];
    }

    pub fn get_side_directions(&self) -> Vec<Vec2> {
        return vec![Vec2::NEG_Y, Vec2::X, Vec2::Y, Vec2::NEG_X];
    }
}

fn update_side_effects(
    players: Query<(&Player, &Handle<PlayerMaterial>), Changed<Player>>,
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

fn update_side_entities(
    players: Query<&Player>,
    mut sides: Query<(&mut Friction, &mut Restitution), Without<Player>>,
) {
    for player in &players {
        for i in 0..MAX_SIDES {
            let f_coefficient = match player.effects[i] {
                SideEffect::Sticky => 0.6,
                SideEffect::Slippery => 0.02,
                _ => 0.3,
            };
            let r_coefficient = match player.effects[i] {
                SideEffect::Spring => 0.75,
                _ => 0.0,
            };

            if let Ok((mut friction, mut restitution)) =
                sides.get_mut(player.side_entities[i].unwrap())
            {
                if friction.coefficient != f_coefficient {
                    friction.coefficient = f_coefficient;
                }
                if restitution.coefficient != r_coefficient {
                    restitution.coefficient = r_coefficient;
                }
            }
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
        let player: &mut Player = &mut player;

        player.moving_state.advance(&time);
        player.landed_state.advance(&time);
        player.in_air_state.advance(&time);
        player.stick_to_wall_state.advance(&time);
        player.stick_to_anything_state.advance(&time);
        player.time_since_last_spin += time.delta_seconds();
        player.time_since_last_jump += time.delta_seconds();

        // Move left-right
        {
            let mut target_velocity = 0.0;
            let right = gravity_direction.get_vec().perp();

            if keys.any_pressed([KeyCode::A]) {
                player.moving_state.activate();
                target_velocity -= player.get_max_speed();
            }

            if keys.any_pressed([KeyCode::D]) {
                player.moving_state.activate();
                target_velocity += player.get_max_speed();
            }

            let mut delta_velocity = target_velocity - velocity.linvel.dot(right);

            // For more "snappy" moves
            let bonus = ((delta_velocity.abs() - player.get_max_speed()) / player.get_max_speed())
                .clamp(0.0, 2.0);

            let limit = player.get_max_acceleration() * time.delta_seconds() * (1.0 + bonus);

            delta_velocity = delta_velocity.clamp(-limit, limit);

            let multiplier = if player.moving_state.time_since_deactivated() < 0.2
                && player.landed_state.is_active
            {
                1.0
            } else {
                1.0 / (2.0 + player.moving_state.time_since_deactivated() * 2.0)
            };

            if target_velocity != 0.0 || !player.slippery_below_state.is_active {
                *impulse += ExternalImpulse::at_point(
                    right * delta_velocity * mass.0.mass * multiplier,
                    transform.translation.truncate() + player.get_center_offset(),
                    transform.translation.truncate(),
                );
            }
        }

        // Schedule spin
        let force_jump = {
            if keys.just_pressed(KeyCode::W) {
                player.delayed_spin_torque = Some(3.0);
                true
            } else if keys.just_pressed(KeyCode::S) {
                player.delayed_spin_torque = Some(-3.0);
                true
            } else {
                false
            }
        };

        // Spin
        {
            if player.in_air_state.time_since_activated() > 0.02 {
                if let Some(torque) = player.delayed_spin_torque.take() {
                    if player.time_since_last_spin < 0.3 {
                        // Spin with lower torque if already spinning
                        impulse.torque_impulse += torque * 0.4 * mass.0.principal_inertia;
                    } else {
                        impulse.torque_impulse += torque * mass.0.principal_inertia;
                    }
                    player.time_since_last_spin = 0.0;
                }
            }
        }

        // Snap to 90-s angles
        {
            let angle: f32 =
                (get_angle_from_quat(transform.rotation) % (2.0 * PI) + 2.0 * PI) % (2.0 * PI);
            let target_angle = ((angle + velocity.angvel * 0.3) / (PI * 0.5)).round() * (PI * 0.5);

            let mut delta_angle = target_angle - angle;

            let limit = player.get_max_angular_acceleration() * time.delta_seconds() * 2.0;

            delta_angle = delta_angle.clamp(-limit, limit);

            let multiplier = if player.stick_to_anything_state.time_since_activated() > 0.1
                || player.time_since_last_spin < 0.2
            {
                0.5
            } else {
                5.0
            };

            // Apply torque in direction of nearest right angle
            impulse.torque_impulse += delta_angle * mass.0.principal_inertia * multiplier;
            // Apply damping torque
            impulse.torque_impulse -= 0.2 * velocity.angvel * mass.0.principal_inertia * multiplier;
        }

        // Jump
        {
            let collider_below = find_obstacle(
                entity,
                &player.small_collider,
                gravity_direction.get_vec(),
                transform,
                &context,
                0.027,
            );

            let collider_right = find_obstacle(
                entity,
                &player.small_collider,
                gravity_direction.get_vec().perp(),
                transform,
                &context,
                0.027,
            );

            let collider_left = find_obstacle(
                entity,
                &player.small_collider,
                -gravity_direction.get_vec().perp(),
                transform,
                &context,
                0.027,
            );

            if collider_below.is_some() {
                player.landed_state.activate();
                player.in_air_state.deactivate();
            } else {
                player.landed_state.deactivate();
                player.in_air_state.activate();
            }

            if keys.any_pressed([KeyCode::Space]) || force_jump {
                if (player.landed_state.time_since_activated() > 0.05
                    || player.stick_to_wall_state.time_since_activated() > 0.05)
                    && player.time_since_last_jump > 0.2
                {
                    player.landed_state.deactivate();
                    player.in_air_state.activate();
                    player.time_since_last_jump = 0.0;

                    let mut dir = -gravity_direction.get_vec();

                    // Side jump if stick to wall
                    if collider_right.is_some()
                        && player.stick_to_wall_state.time_since_activated() > 0.05
                    {
                        dir *= 1.0;
                        dir += -gravity_direction.get_vec().perp() * 0.5;
                    }

                    // Side jump if stick to wall
                    if collider_left.is_some()
                        && player.stick_to_wall_state.time_since_activated() > 0.05
                    {
                        dir *= 1.0;
                        dir += gravity_direction.get_vec().perp() * 0.5;
                    }

                    impulse.impulse += dir * player.get_jump_impulse() * mass.0.mass;
                }
            }
        }

        let mut stick_to_wall = false;
        let mut stick_to_something = false;
        let mut slippery_below = false;

        for i in 0..MAX_SIDES {
            let dir = Vec2::from_angle(get_angle_from_quat(transform.rotation))
                .rotate(player.get_side_directions()[i]);

            let collider_nearby = find_obstacle(
                entity,
                &player.small_collider,
                dir,
                transform,
                &context,
                0.045,
            );

            match player.effects[i] {
                SideEffect::None => {}
                SideEffect::Sticky => {
                    // Apply stick force if there is collider ti which we can stick and we did not just jump
                    if collider_nearby.is_some() && player.time_since_last_jump > 0.03 {
                        stick_to_something = true;
                        let stick_force = if dir.dot(Vec2::NEG_Y) > 0.8 {
                            // Stick to floor
                            5.0
                        } else if dir.dot(Vec2::Y) > 0.8 {
                            // Stick to ceiling
                            20.0
                        } else {
                            // Stick to wall
                            stick_to_wall = true;
                            20.0
                        };

                        let offset = if keys.pressed(KeyCode::A) {
                            Vec2::ZERO
                        } else if keys.pressed(KeyCode::D) {
                            Vec2::ZERO
                        } else {
                            Vec2::ZERO
                        }; // + dir * 0.4;

                        *impulse += ExternalImpulse::at_point(
                            dir * stick_force * mass.0.mass * time.delta_seconds(),
                            transform.translation.truncate() + offset,
                            transform.translation.truncate(),
                        );
                    }
                }
                SideEffect::Slippery => {
                    if collider_nearby.is_some() && dir.dot(Vec2::NEG_Y) > 0.4 {
                        slippery_below = true;
                    }
                }
                SideEffect::Shield => {}
                SideEffect::Thorns => {}
                SideEffect::Flashlight => {}
                SideEffect::Laser => {}
                SideEffect::Spring => {}
            }
        }

        player.stick_to_wall_state.set(stick_to_wall);
        player.stick_to_anything_state.set(stick_to_something);
        player.slippery_below_state.set(slippery_below);
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
    toi: f32,
) -> Option<(Entity, f32)> {
    let filter = QueryFilter::new()
        .groups(CollisionGroups::new(Group::GROUP_2, Group::GROUP_1))
        .exclude_collider(entity);
    context
        .cast_shape(
            transform.translation.truncate(),
            get_angle_from_quat(transform.rotation),
            direction,
            collider,
            toi,
            filter,
        )
        .map(|(e, toi)| (e, toi.toi))
}
