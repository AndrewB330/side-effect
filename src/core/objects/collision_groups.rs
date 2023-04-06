use bevy_rapier2d::geometry::CollisionGroups;
use bevy_rapier2d::prelude::Group;

pub const WALL_BIT: Group = Group::GROUP_1;
pub const PLAYER_BIT: Group = Group::GROUP_2;
pub const BONUS_BIT: Group = Group::GROUP_3;
pub const MONSTER_BIT: Group = Group::GROUP_4;

pub const WALL_FILTER: Group = PLAYER_BIT.union(BONUS_BIT).union(MONSTER_BIT);
pub const PLAYER_FILTER: Group = WALL_BIT.union(BONUS_BIT).union(MONSTER_BIT);
pub const BONUS_FILTER: Group = WALL_BIT.union(PLAYER_BIT).union(BONUS_BIT);
pub const MONSTER_FILTER: Group = WALL_BIT.union(PLAYER_BIT);

pub const WALL_CG: CollisionGroups = CollisionGroups::new(WALL_BIT, WALL_FILTER);
pub const PLAYER_CG: CollisionGroups = CollisionGroups::new(PLAYER_BIT, PLAYER_FILTER);
pub const BONUS_CG: CollisionGroups = CollisionGroups::new(BONUS_BIT, BONUS_FILTER);
pub const MONSTER_CG: CollisionGroups = CollisionGroups::new(MONSTER_BIT, MONSTER_FILTER);
