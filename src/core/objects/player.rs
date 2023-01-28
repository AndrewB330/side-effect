use crate::shape::Quad;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy_rapier2d::prelude::*;
use crate::core::direction::SceneDirection;
use crate::states::GameWorldState;

#[derive(Clone, Debug, Default)]
pub enum Shape {
    #[default]
    Square,
    Circle,
    Triangle,
    Star,
    Pentagon,
}

#[derive(Component, Clone, Debug, Default)]
pub struct Player {
    pub id: u32,
    pub shape: Shape,
    pub max_speed: f32,
    pub max_acceleration: f32,
    pub jump_impulse: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameWorldState::GameWorld)
                .with_system(move_player)
                .with_system(jump_player),
        );
    }
}

fn move_player(
    mut players: Query<(
        &mut ExternalImpulse,
        &Velocity,
        &ReadMassProperties,
        &mut Player,
    )>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    context: Res<RapierContext>,
    config: ResMut<RapierConfiguration>,
) {
    let gravity_direction = SceneDirection::from_gravity_direction(&*config);

    for (mut impulse, velocity, mass, mut player) in players.iter_mut() {
        let mut target_velocity = 0.0;

        let right = gravity_direction.get_perp().get_vec();
        let mut dir = Vec2::ZERO;

        if keys.any_pressed([KeyCode::A]) {
            target_velocity -= player.max_speed;
            dir = -right;
        }

        if keys.any_pressed([KeyCode::D]) {
            target_velocity += player.max_speed;
            dir = right;
        }

        let delta_velocity = target_velocity - velocity.linvel.dot(right);
        let k = ((delta_velocity.abs() - player.max_speed * 1.0).max(0.0) / player.max_speed)
            .clamp(0.0, 2.0);

        let dv = delta_velocity
            .abs()
            .min(player.max_acceleration * time.delta_seconds() * (1.0 + k));

        impulse.impulse += right * delta_velocity.signum() * dv * mass.0.mass;
    }
}


fn find_obstacle(
    entity: Entity,
    direction: SceneDirection,
    gravity_direction: SceneDirection,
    position: Vec2,
    context: &RapierContext,
) -> Option<(Entity, f32)> {
    const INTERVALS: u32 = 5;

    let (du, dv) = if (direction.get_index() + gravity_direction.get_index()) % 2 == 0 {
        (0.5, 0.5)
    } else {
        (0.5, 0.5)
    };

    let mut res = None;

    for i in 0..INTERVALS {
        let t = (i as f32 / (INTERVALS - 1) as f32) * 1.8 - 0.9;
        let origin =
            position + direction.get_vec() * du + direction.get_perp().get_vec() * t * dv;
        let dir = direction.get_vec();

        let filter = QueryFilter::new()
            .exclude_collider(entity);

        if let Some((e, d)) = context.cast_ray(origin, dir, 100.0, true, filter) {
            if let Some((_, prev)) = res.clone() {
                if d < prev {
                    res = Some((e, d))
                }
            } else {
                res = Some((e, d));
            }
        }
    }

    res
}

fn jump_player(
    mut players: Query<(
        Entity,
        &mut ExternalImpulse,
        &Player,
        &Velocity,
        &GlobalTransform,
        &ReadMassProperties,
    )>,
    context: Res<RapierContext>,
    keys: Res<Input<KeyCode>>,
    config: Res<RapierConfiguration>,
) {
    let gravity_direction = SceneDirection::from_gravity_direction(&config);

    for (entity, mut ext_impulse, player, velocity, transform, mass) in players.iter_mut() {
        if keys.any_pressed([KeyCode::Space]) {
            let collider_below = find_obstacle(
                entity,
                gravity_direction,
                gravity_direction,
                transform.translation().truncate(),
                &context,
            );

            if let Some((_, dist)) = collider_below {
                if dist < 0.1 && velocity.linvel.dot(gravity_direction.get_vec()).abs() < 2.0 {
                    ext_impulse.impulse =
                        -config.gravity.normalize() * player.jump_impulse * mass.0.mass;
                }
            }
        }
    }
}
