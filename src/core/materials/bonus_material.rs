use crate::core::objects::shape::MAX_SIDES;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "e228a544-e3ca-4e1e-ba9d-4d8bc1bd8c19"]
pub struct BonusMaterial {
    #[uniform(0)]
    pub effect_index: u32,

    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,
}

impl Material2d for BonusMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/bonus_material.wgsl".into()
    }
}
