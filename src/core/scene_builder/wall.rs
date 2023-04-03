use crate::core::scene_builder::SceneBuilder;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use crate::core::objects::collision_groups::WALL_CG;

#[derive(Component, Default, Debug)]
pub struct Wall;

impl<'w, 's, 'a> SceneBuilder<'w, 's, 'a> {
    pub fn spawn_wall_from_to(&mut self, from: Vec2, to: Vec2) {
        let size = from.max(to) - from.min(to);
        let translation = (from + to) * 0.5;
        self.commands.spawn((
            Wall::default(),
            Collider::cuboid(size.x * 0.5, size.y * 0.5),
            RigidBody::Fixed,
            MaterialMesh2dBundle {
                mesh: self.meshes.add(shape::Quad::new(size).into()).into(),
                material: self.materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
                transform: Transform::from_translation(Vec3::new(
                    translation.x,
                    translation.y,
                    Self::WALL_DEPTH,
                )),
                ..default()
            },
            WALL_CG,
        ));
    }
}
