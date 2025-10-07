use bevy::render::render_resource::AsBindGroup;
use bevy::{prelude::*, shader::ShaderRef};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MaterialPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shader.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let size = 10.0;

    info!("Here");

    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection::default()),
        Transform::from_xyz(0.0, 250.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::IDENTITY,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Plane3d::default().mesh().size(100.0, 100.0)))),
        MeshMaterial3d(materials.add(CustomMaterial {
            texture: Some(asset_server.load("spritesheet.png")),
            color: LinearRgba::WHITE,
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::IDENTITY,
    ));
}
