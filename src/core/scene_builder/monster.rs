use crate::core::objects::bonus::Bonus;

use crate::core::materials::bonus_material::BonusMaterial;
use crate::core::materials::monster_material::MonsterMaterial;
use crate::core::objects::collision_groups::BONUS_CG;
use crate::core::objects::side_effect::SideEffect;
use crate::core::scene_builder::SceneBuilder;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RigidBody, Sensor};

impl<'w, 's, 'a> SceneBuilder<'w, 's, 'a> {
    pub fn spawn_monster(&mut self, position: Vec2) {
        let monster_material = MonsterMaterial {
            monster_index: 0,
            animation_tick: 0,
            overlay: 1,
            texture: Some(self.asset_server.load("images/monster.png")),
        };

        self.commands.spawn((
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_xyz(
                position.x,
                position.y,
                SceneBuilder::BONUS_DEPTH,
            )),
            self.monster_materials.add(monster_material),
            Mesh2dHandle(
                self.meshes
                    .add(shape::Quad::new(Vec2::new(1.0, 1.0)).into()),
            ),
            RigidBody::Fixed,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Collider::ball(0.5),
            BONUS_CG,
        ));
    }
}
