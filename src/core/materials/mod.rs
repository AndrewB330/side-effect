use bevy::prelude::*;

pub mod bonus_material;
pub mod monster_material;
pub mod player_material;

use bonus_material::*;
use monster_material::*;
use player_material::*;

pub fn update_materials(
    player_handles: Query<&Handle<PlayerMaterial>>,
    bonus_handles: Query<&Handle<BonusMaterial>>,
    monster_handles: Query<&Handle<MonsterMaterial>>,
    mut player_materials: ResMut<Assets<PlayerMaterial>>,
    mut bonus_materials: ResMut<Assets<BonusMaterial>>,
    mut monster_materials: ResMut<Assets<MonsterMaterial>>,
) {
    for h in &player_handles {
        if let Some(m) = player_materials.get_mut(h) {}
    }
    for h in &bonus_handles {
        if let Some(m) = bonus_materials.get_mut(h) {}
    }
    for h in &monster_handles {
        if let Some(m) = monster_materials.get_mut(h) {
            m.animation_tick += 1;
        }
    }
}
