use crate::core::objects::player::{Player, Shape};
use crate::core::scene_builder::SceneBuilder;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct WallBundle {
    wall: Wall,
    rigid_body: RigidBody,
    collider: Collider,

    #[bundle]
    mesh: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Component, Default, Debug)]
pub struct Wall;

impl<'w, 's, 'a> SceneBuilder<'w, 's, 'a> {
    pub fn spawn_wall_from_to(&mut self, mut from: Vec2, mut to: Vec2) {
        let size = from.max(to) - from.min(to);
        let translation = (from + to) * 0.5;
        self.commands.spawn_bundle(WallBundle {
            wall: Wall::default(),
            collider: Collider::cuboid(size.x * 0.5, size.y * 0.5),
            rigid_body: RigidBody::Fixed,
            mesh: MaterialMesh2dBundle {
                mesh: self.meshes.add(shape::Quad::new(size).into()).into(),
                material: self.materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
                transform: Transform::from_translation(Vec3::new(
                    translation.x,
                    translation.y,
                    Self::WALL_DEPTH,
                )),
                ..default()
            },
        });
    }
}
