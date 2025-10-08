#import bevy_pbr::forward_io::VertexOutput

struct Params {
    color: vec4<f32>,
    crop_offset_px: vec2<f32>,
    crop_size_px: vec2<f32>,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> params: Params;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var material_color_texture: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let tex_size   = vec2<f32>(textureDimensions(material_color_texture));
    let half_texel = vec2<f32>(0.5, 0.5);

    // Map quad UV [0,1] to pixel coords inside the chosen sub-rect.
    let uv_px = params.crop_offset_px
               + mesh.uv * (params.crop_size_px - vec2<f32>(1.0, 1.0))
               + half_texel;

    let uv = uv_px / tex_size;

    // If you get mip bleed on tiny sprites, use textureSampleLevel(..., 0.0).
    let sampled = textureSample(material_color_texture, material_color_sampler, uv);
    return params.color * sampled;
}
