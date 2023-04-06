#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings

@group(1) @binding(0)
var<uniform> monster: u32;
@group(1) @binding(1)
var<uniform> animation_tick: u32;
@group(1) @binding(2)
var<uniform> state: u32;
@group(1) @binding(3)
var texture: texture_2d<f32>;
@group(1) @binding(4)
var texture_sampler: sampler;

@group(2) @binding(0)
var<uniform> mesh: Mesh2d;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    #import bevy_sprite::mesh2d_vertex_output
};

fn mix_colors(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return vec4(a.rgb * (1.0 - b.a) + b.rgb * b.a, a.a * (1.0 - b.a) + b.a);
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var uv = in.uv;
    if ((state & 1u) == 0u) {
        uv.x = 1.0 - uv.x;
    }
    let color = textureSample(texture, texture_sampler, vec2((uv.x + f32(animation_tick / 5u % 8u) + 2.0) / 10.0, (uv.y + f32(monster)) / 1.0));
    let overlay = textureSample(texture, texture_sampler, vec2((uv.x + f32(state / 2u)) / 10.0, (uv.y + f32(monster)) / 1.0));

    return mix_colors(color, overlay);
}
