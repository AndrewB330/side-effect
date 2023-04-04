use crate::core::objects::shape::MAX_SIDES;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "e228a544-e3ca-4e1e-ba9d-4a8bc1bd8c19"]
pub struct MonsterMaterial {
    #[uniform(0)]
    pub monster_index: u32,

    #[uniform(1)]
    pub animation_tick: u32,

    #[uniform(2)]
    pub overlay: u32,

    #[texture(3)]
    #[sampler(4)]
    pub texture: Option<Handle<Image>>,
}

impl Material2d for MonsterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/monster_material.wgsl".into()
    }
}
