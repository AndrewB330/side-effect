#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings

struct ColorMaterial {
    color: vec4<f32>,
    sides: array<vec4<u32>, 10>,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
};

const COLOR_MATERIAL_FLAGS_TEXTURE_BIT: u32 = 1u;
const COLOR_MATERIAL_FLAGS_EMISSIVE_BIT: u32 = 2u;
const COLOR_MATERIAL_FLAGS_OVERLAY_BIT: u32 = 4u;

@group(1) @binding(0)
var<uniform> material: ColorMaterial;
@group(1) @binding(1)
var texture: texture_2d<f32>;
@group(1) @binding(2)
var texture_sampler: sampler;
@group(1) @binding(3)
var emissive: texture_2d<f32>;
@group(1) @binding(4)
var emissive_sampler: sampler;
@group(1) @binding(5)
var overlay: texture_2d<f32>;
@group(1) @binding(6)
var overlay_sampler: sampler;
@group(1) @binding(7)
var player_effect_texture: texture_2d<f32>;
@group(1) @binding(8)
var player_effect_texture_sampler: sampler;

@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    #import bevy_sprite::mesh2d_vertex_output
};

fn hsl2rgb(c: vec3<f32>) -> vec3<f32>
{
    var x = c.x*6.0+vec3(0.0,4.0,2.0);
    var d = vec3(x.x % 6.0,x.y % 6.0,x.z % 6.0);
    var rgb = clamp(abs(d - 3.0) - 1.0, vec3(0.0), vec3(1.0));
    return c.z + c.y * (rgb - 0.5)*(1.0 - abs(2.0*c.z - 1.0));
}

fn rot(uv: vec2<f32>) -> vec2<f32> {
    return vec2(1.0 - uv.y, uv.x);
}

fn unrot(uv: vec2<f32>) -> vec2<f32> {
    return vec2(uv.y, 1.0 - uv.x);
}

fn get_effect_color(side_index: i32, uvi: vec2<f32>) -> vec4<f32> {
    var uv = unrot(uvi);
    if (side_index >= 1) {uv = rot(uv);};
    if (side_index >= 2) {uv = rot(uv);};
    if (side_index >= 3) {uv = rot(uv);};

    var effect = material.sides[side_index].x;

    return textureSample(player_effect_texture, player_effect_texture_sampler, vec2(uv.x / 2.0, (uv.y + f32(effect)) / 4.0));
}

fn get_effect_corner_color(side_index: i32, uvi: vec2<f32>) -> vec4<f32> {
    var uv = unrot(uvi);
    if (side_index >= 1) {uv = rot(uv);};
    if (side_index >= 2) {uv = rot(uv);};
    if (side_index >= 3) {uv = rot(uv);};

    var effect = material.sides[side_index].x;

    var color = textureSample(player_effect_texture, player_effect_texture_sampler, vec2((uv.x + 1.0) / 2.0, (uv.y + f32(effect)) / 4.0));

    if (material.sides[side_index].x == material.sides[(side_index + 1) % 4].x) {
        return color;
    } else {
        return vec4(0.0);
    }
}

fn mix_colors(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return vec4(a.rgb * (1.0 - b.a) + b.rgb * b.a, a.a * (1.0 - b.a) + b.a);
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var output_color: vec4<f32> = material.color;
    if ((material.flags & COLOR_MATERIAL_FLAGS_TEXTURE_BIT) != 0u) {
        var texture_color = textureSample(texture, texture_sampler, in.uv);
        if ((material.flags & COLOR_MATERIAL_FLAGS_EMISSIVE_BIT) != 0u) {
            let emissive = textureSample(emissive, emissive_sampler, in.uv);
            texture_color = texture_color + vec4(emissive.rgb * emissive.a, 0.0) * 5.0;
        }
#ifdef VERTEX_COLORS
        output_color = output_color * texture_color * in.color;
#else
        output_color = output_color * texture_color;
#endif
        if ((material.flags & COLOR_MATERIAL_FLAGS_OVERLAY_BIT) != 0u) {
            let overlay = textureSample(overlay, overlay_sampler, in.uv);
            output_color = vec4(output_color.rgb * (1.0 - overlay.w) + overlay.rgb * overlay.w, output_color.w);
        }

        output_color = mix_colors(output_color, get_effect_color(0, in.uv));
        output_color = mix_colors(output_color, get_effect_color(1, in.uv));
        output_color = mix_colors(output_color, get_effect_color(2, in.uv));
        output_color = mix_colors(output_color, get_effect_color(3, in.uv));

        output_color = mix_colors(output_color, get_effect_corner_color(0, in.uv));
        output_color = mix_colors(output_color, get_effect_corner_color(1, in.uv));
        output_color = mix_colors(output_color, get_effect_corner_color(2, in.uv));
        output_color = mix_colors(output_color, get_effect_corner_color(3, in.uv));
    }

    if (output_color.a < 0.01) {
        output_color = vec4(output_color.rgb, 0.0);
    }

    return output_color;
}
