use crate::core::objects::shape::MAX_SIDES;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{AsBindGroup, AsBindGroupShaderType, ShaderRef, ShaderType};
use bevy::sprite::Material2d;

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "e228a544-e3ca-4e1e-ba9d-4d8bc1ad8c19"]
#[uniform(0, PlayerMaterialUniform)]
pub struct PlayerMaterial {
    pub color: Color,
    pub effect_index: [u32; MAX_SIDES],

    #[texture(1)]
    #[sampler(2)]
    pub texture: Option<Handle<Image>>,

    #[texture(3)]
    #[sampler(4)]
    pub emissive: Option<Handle<Image>>,

    #[texture(5)]
    #[sampler(6)]
    pub overlay: Option<Handle<Image>>,

    #[texture(7)]
    #[sampler(8)]
    pub player_effect_texture: Handle<Image>,
}

// The GPU representation of the uniform data of a [`ColorMaterialCustom`].
#[derive(Clone, Default, ShaderType)]
pub struct PlayerMaterialUniform {
    pub color: Vec4,
    pub effect_index: [UVec4; MAX_SIDES],
    pub flags: u32,
}

// NOTE: These must match the bit flags in bevy_sprite/src/mesh2d/color_material.wgsl!
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ColorMaterialFlagsCustom: u32 {
        const TEXTURE           = (1 << 0);
        const EMISSIVE          = (1 << 1);
        const OVERLAY           = (1 << 2);
        const NONE              = 0;
        const UNINITIALIZED     = 0xFFFF;
    }
}

impl AsBindGroupShaderType<PlayerMaterialUniform> for PlayerMaterial {
    fn as_bind_group_shader_type(&self, _images: &RenderAssets<Image>) -> PlayerMaterialUniform {
        let mut flags = ColorMaterialFlagsCustom::NONE;
        if self.texture.is_some() {
            flags |= ColorMaterialFlagsCustom::TEXTURE;
        }
        if self.emissive.is_some() {
            flags |= ColorMaterialFlagsCustom::EMISSIVE;
        }
        if self.overlay.is_some() {
            flags |= ColorMaterialFlagsCustom::OVERLAY;
        }

        PlayerMaterialUniform {
            color: self.color.as_linear_rgba_f32().into(),
            effect_index: self.effect_index.clone().map(|i| UVec4::splat(i)),
            flags: flags.bits(),
        }
    }
}

impl Material2d for PlayerMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/player_material.wgsl".into()
    }
}
