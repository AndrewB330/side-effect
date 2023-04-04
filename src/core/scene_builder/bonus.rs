use crate::core::objects::bonus::Bonus;

use crate::core::materials::bonus_material::BonusMaterial;
use crate::core::objects::collision_groups::BONUS_CG;
use crate::core::objects::side_effect::SideEffect;
use crate::core::scene_builder::SceneBuilder;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RigidBody, Sensor};

impl<'w, 's, 'a> SceneBuilder<'w, 's, 'a> {
    pub fn spawn_effect_bonus(&mut self, position: Vec2, effect: SideEffect) {
        let bonus_material = BonusMaterial {
            effect_index: effect.to_index(),
            texture: Some(self.asset_server.load("images/bonus.png")),
        };

        self.commands.spawn((
            Bonus {
                effect: Some(effect),
            },
            VisibilityBundle::default(),
            TransformBundle::from_transform(Transform::from_xyz(
                position.x,
                position.y,
                SceneBuilder::BONUS_DEPTH,
            )),
            self.bonus_materials.add(bonus_material),
            Mesh2dHandle(
                self.meshes
                    .add(shape::Quad::new(Vec2::new(0.5, 0.5)).into()),
            ),
            RigidBody::Fixed,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Collider::ball(0.25),
            BONUS_CG,
        ));
    }
}
