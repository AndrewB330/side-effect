use bevy::prelude::{Mesh, Vec2};
use bevy::prelude::shape;

#[derive(Clone, Debug, Default, PartialEq, Hash)]
pub enum PlayerShape {
    #[default]
    Square,
    Circle,
    Triangle,
    Star,
    Pentagon,
}

impl PlayerShape {
    pub fn get_default_mesh(&self) -> Mesh {
        match *self {
            PlayerShape::Square => {
                shape::Quad::new(Vec2::new(1.0, 1.0)).into()
            }
            PlayerShape::Circle => {
                shape::Circle::new(0.5).into()
            }
            PlayerShape::Triangle => {
                todo!()
            }
            PlayerShape::Star => {
                todo!()
            }
            PlayerShape::Pentagon => {
                todo!()
            }
        }
    }
}