use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    shader::ShaderRef,
};

#[derive(Clone, Copy, Debug, ShaderType, Default)]
pub struct TextureCropParams {
    pub color: Vec4,
    pub crop_offset_px: Vec2,
    pub crop_size_px: Vec2,
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct MaterialTextureAtlas {
    #[uniform(0)]
    pub params: TextureCropParams,

    #[texture(1)]
    #[sampler(2)]
    texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,

    pub size: UVec2,
    pub textures: Vec<URect>,
    pub index: u32,
}

impl Material for MaterialTextureAtlas {
    fn fragment_shader() -> ShaderRef {
        "shader.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

impl MaterialTextureAtlas {
    pub fn from_grid(
        texture: Handle<Image>,
        tile_size: UVec2,
        columns: u32,
        rows: u32,
        padding: Option<UVec2>,
        offset: Option<UVec2>,
    ) -> Self {
        let padding = padding.unwrap_or_default();
        let offset = offset.unwrap_or_default();
        let mut sprites = Vec::new();
        let mut current_padding = UVec2::ZERO;

        for y in 0..rows {
            if y > 0 {
                current_padding.y = padding.y;
            }
            for x in 0..columns {
                if x > 0 {
                    current_padding.x = padding.x;
                }

                let cell = UVec2::new(x, y);
                let rect_min = (tile_size + current_padding) * cell + offset;

                sprites.push(URect {
                    min: rect_min,
                    max: rect_min + tile_size,
                });
            }
        }
        let grid_size = UVec2::new(columns, rows);
        let x = MaterialTextureAtlas {
            params: TextureCropParams {
                color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                crop_offset_px: Vec2::new(0.0, 0.0),
                crop_size_px: Vec2::new(16.0, 16.00),
                ..default()
            },
            texture: Some(texture),
            alpha_mode: AlphaMode::Blend,
            size: ((tile_size + current_padding) * grid_size) - current_padding,
            textures: sprites,
            index: 0,
        };

        info!("{:?}", x);
        x
    }
}
