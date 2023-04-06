use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalImpulse;

use super::player::Player;

#[derive(Component, Debug, Clone)]
pub struct Monster {}

impl Monster {}

pub fn move_monster(
    mut monsters: Query<(&mut Monster, &Transform, &ExternalImpulse)>,
    players: Query<&Transform, With<Player>>,
) {
    if players.is_empty() {
        return;
    }

    let nearest_player = players.single();

    for (monster, transform, impulse) in monsters.iter_mut() {}
}
