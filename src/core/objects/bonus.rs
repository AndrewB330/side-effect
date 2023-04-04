use crate::{core::objects::side_effect::SideEffect, states::GameWorldState};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{CollisionEvent, RapierContext};

use super::player::Player;

pub struct BonusPlugin;

impl Plugin for BonusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((bonus_pickup.run_if(in_state(GameWorldState::GameWorld)),));
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct Bonus {
    pub effect: Option<SideEffect>,
}

impl Bonus {
    pub fn apply_to(&mut self, player: &mut Player, pt: &Transform, bt: &Transform) {
        let centers = player.get_side_centers();
        let mut nearest_dist = 1e9;
        let mut nearest_side = None;

        for i in 0..centers.len() {
            let dist = pt
                .transform_point(centers[i].extend(0.0))
                .distance(bt.translation);
            if dist < nearest_dist && player.effects[i] == SideEffect::None {
                nearest_dist = dist;
                nearest_side = Some(i);
            }
        }

        if let Some(side) = nearest_side {
            if let Some(effect) = self.effect.take() {
                player.effects[side] = effect;
            }
        }
    }

    pub fn is_used(&self) -> bool {
        self.effect.is_none()
    }
}

pub fn bonus_pickup(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut players: Query<(&Transform, &mut Player)>,
    mut bonuses: Query<(Entity, &Transform, &mut Bonus)>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(a, b, args) = collision_event {
            if let Ok((pt, mut player)) = players.get_mut(*a) {
                if let Ok((_, bt, mut bonus)) = bonuses.get_mut(*b) {
                    bonus.apply_to(&mut player, pt, bt);
                }
            } else if let Ok((pt, mut player)) = players.get_mut(*b) {
                if let Ok((_, bt, mut bonus)) = bonuses.get_mut(*a) {
                    bonus.apply_to(&mut player, pt, bt);
                }
            }
        }
    }

    for (entity, _, bonus) in &bonuses {
        if bonus.is_used() {
            commands.entity(entity).despawn()
        }
    }
}
