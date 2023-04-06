use crate::core::direction::SceneDirection;
use crate::core::materials::monster_material::MonsterMaterial;
use crate::states::GameWorldState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::player::Player;

#[derive(Component, Debug, Clone, Default)]
pub struct Monster {
    pub patrol: bool,
}

impl Monster {}

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((move_monster.run_if(in_state(GameWorldState::GameWorld)),));
    }
}

pub fn move_monster(
    mut monsters: Query<(
        &mut Monster,
        &Transform,
        &mut ExternalImpulse,
        &Velocity,
        &ReadMassProperties,
        &Handle<MonsterMaterial>,
    )>,
    players: Query<&Transform, With<Player>>,
    config: ResMut<RapierConfiguration>,
    time: Res<Time>,
    mut materials: ResMut<Assets<MonsterMaterial>>,
) {
    if players.is_empty() {
        return;
    }

    let nearest_player = players.single();
    let gravity_direction = SceneDirection::from_gravity_direction(&config);
    let right = gravity_direction.get_vec().perp();

    for (monster, transform, mut impulse, velocity, mass, handle) in monsters.iter_mut() {
        let mut state = 0;

        if velocity.linvel.x < 0.0 {
            state |= 1;
        }

        {
            let mut target_velocity = 0.0;
            if monster.patrol {
                if velocity.linvel.x > 0.05 || (velocity.linvel.x > -0.05 && rand::random::<bool>())
                {
                    target_velocity = 2.0;
                } else {
                    target_velocity = -2.0;
                }
            } else {
                if transform.translation.x > nearest_player.translation.x {
                    target_velocity -= 2.0;
                } else {
                    target_velocity += 2.0;
                }
            }

            let mut delta_velocity = target_velocity - velocity.linvel.dot(right);

            let limit = 12.0 * time.delta_seconds();

            delta_velocity = delta_velocity.clamp(-limit, limit);

            impulse.impulse += right * delta_velocity * mass.0.mass;
        }

        if let Some(m) = materials.get_mut(handle) {
            m.state = state;
        }
    }
}
