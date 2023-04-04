use crate::core::materials::player_material::PlayerMaterial;
use bevy::prelude::shape;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
use bevy::utils::HashMap;
use bevy_rapier2d::prelude::Collider;

pub struct ShapePlugin;

impl Plugin for ShapePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerShapeVisualBundleCache>();
        //app.add_startup_system(init_cache.in_base_set(StartupSet::PreStartup));
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PlayerShape {
    #[default]
    Square,
    Pentagon,
    Hexagon,
}

pub const MAX_SIDES: usize = 4;

impl PlayerShape {
    pub fn get_default_material(&self) -> ColorMaterial {
        match *self {
            PlayerShape::Square => Color::rgb_u8(23u8, 120u8, 255u8).into(),
            PlayerShape::Pentagon => Color::rgb_u8(255u8, 182u8, 23u8).into(),
            PlayerShape::Hexagon => Color::rgb_u8(182u8, 255u8, 23u8).into(),
        }
    }

    pub fn get_default_collider(&self) -> Collider {
        match *self {
            _ => Collider::round_cuboid(0.4, 0.4, 0.1),
        }
    }

    pub fn get_default_mesh(&self) -> Mesh {
        match *self {
            _ => shape::Quad::new(Vec2::new(1.0, 1.0)).into(),
        }
    }

    pub fn number_of_sides(&self) -> u32 {
        match *self {
            PlayerShape::Square => 4,
            PlayerShape::Pentagon => 5,
            PlayerShape::Hexagon => 6,
        }
    }

    pub fn get_texture_file(&self) -> Option<&'static str> {
        match *self {
            PlayerShape::Square => Some("images/square.png"),
            _ => None,
        }
    }

    pub fn list() -> Vec<PlayerShape> {
        return vec![
            PlayerShape::Square,
            PlayerShape::Pentagon,
            PlayerShape::Hexagon,
        ];
    }
}

#[derive(Bundle, Clone)]
pub struct PlayerShapeVisualBundle {
    pub mesh: Mesh2dHandle,
    pub material: Handle<PlayerMaterial>,
    pub collider: Collider,
    //pub sprite: Sprite,
    //pub texture: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct PlayerShapeVisualBundleCache {
    pub cache: HashMap<PlayerShape, PlayerShapeVisualBundle>,
}

/*fn init_cache(mut psv: ResMut<PlayerShapeVisualBundleCache>, mut materials: ResMut<Assets<ColorMaterial>>, mut meshes: ResMut<Assets<Mesh>>,  asset_server: Res<AssetServer>) {
    for shape in PlayerShape::list() {
        psv.cache.insert(
            shape,
            PlayerShapeVisualBundle {
                collider: shape.get_default_collider(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                texture: shape
                    .get_texture_file()
                    .map_or(Handle::default(), |v| asset_server.load(v)),
            },
        );
    }
}*/
