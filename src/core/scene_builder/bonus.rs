use crate::core::objects::bonus::Bonus;

use crate::core::objects::side_effect::SideEffect;
use crate::core::scene_builder::SceneBuilder;
use crate::core::objects::collision_groups::BONUS_CG;
use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody, Sensor, ActiveEvents};

impl<'w, 's, 'a> SceneBuilder<'w, 's, 'a> {
    pub fn spawn_effect_bonus(&mut self, position: Vec2, effect: SideEffect) {
        let file_name = match effect {
            SideEffect::None => todo!(),
            SideEffect::Sticky => "images/bonus_sticky.png",
            SideEffect::Slippery => "images/bonus_slippery.png",
            SideEffect::Shield => "images/bonus_shield.png",
            SideEffect::Thorns => todo!(),
            SideEffect::Flashlight => todo!(),
            SideEffect::Laser => todo!(),
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
            Sprite {
                custom_size: Some(Vec2::new(0.5, 0.5)),
                ..default()
            },
            self.asset_server.load::<Image, &str>(file_name),
            RigidBody::Fixed,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            Collider::ball(0.25),
            BONUS_CG,
        ));
    }
}
