#import bevy_pbr::forward_io::VertexOutput

const COLOR_MULTIPLIER: vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 1.0);
const CROP_SIZE_PX: vec2<f32> = vec2<f32>(16.0, 16.0);

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> material_color: vec4<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var material_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let tex_size = vec2<f32>(textureDimensions(material_color_texture));

    let uv_px = mesh.uv * (CROP_SIZE_PX - vec2<f32>(1.0, 1.0));
    let uv = (uv_px + vec2<f32>(0.5, 0.5)) / tex_size;

    let sampled = textureSample(material_color_texture, material_color_sampler, uv);
    return material_color * sampled * COLOR_MULTIPLIER;
}
