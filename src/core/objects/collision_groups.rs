use bevy_rapier2d::geometry::CollisionGroups;
use bevy_rapier2d::prelude::Group;

pub const WALL_BIT: Group = Group::GROUP_1;
pub const PLAYER_BIT: Group = Group::GROUP_2;
pub const BONUS_BIT: Group = Group::GROUP_3;

pub const WALL_CG: CollisionGroups = CollisionGroups::new(WALL_BIT, Group::ALL);
pub const PLAYER_CG: CollisionGroups = CollisionGroups::new(PLAYER_BIT, Group::ALL);
pub const BONUS_CG: CollisionGroups = CollisionGroups::new(BONUS_BIT, Group::ALL);
