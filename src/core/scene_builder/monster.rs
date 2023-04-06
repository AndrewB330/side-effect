use crate::core::objects::collision_groups::BONUS_CG;
use crate::core::objects::monster::Monster;
use crate::core::scene_builder::SceneBuilder;
use crate::core::{
    materials::monster_material::MonsterMaterial, objects::collision_groups::MONSTER_CG,
};
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy_rapier2d::prelude::*;

impl<'w, 's, 'a> SceneBuilder<'w, 's, 'a> {
    pub fn spawn_monster(&mut self, position: Vec2) {
        let monster_material = MonsterMaterial {
            monster_index: 0,
            animation_tick: 0,
            state: 0,
            texture: Some(self.asset_server.load("images/monster.png")),
        };

        let points = vec![
            Vec2::new(0.3, -0.5),
            Vec2::new(0.44, -0.2),
            Vec2::new(0.44, 0.2),
            Vec2::new(0.3, 0.5),
            Vec2::new(-0.3, 0.5),
            Vec2::new(-0.44, 0.2),
            Vec2::new(-0.44, -0.2),
            Vec2::new(-0.3, -0.5),
        ];

        let collider = Collider::convex_polyline(points).unwrap();

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
            RigidBody::Dynamic,
            ActiveEvents::COLLISION_EVENTS,
            LockedAxes::ROTATION_LOCKED,
            collider,
            MONSTER_CG,
            ReadMassProperties::default(),
            ExternalImpulse::default(),
            Velocity::default(),
            Monster { patrol: true },
        ));
    }
}
