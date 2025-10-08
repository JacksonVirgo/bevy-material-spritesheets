use bevy::render::render_resource::AsBindGroup;
use bevy::{prelude::*, shader::ShaderRef};
use bevy_material_spritesheets::materials::MaterialTextureAtlas;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MaterialPlugin::<MaterialTextureAtlas>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, time)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MaterialTextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera::default(),
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection::default()),
        Transform::from_xyz(0.0, 250.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::IDENTITY,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Plane3d::default().mesh().size(100.0, 100.0)))),
        MeshMaterial3d(materials.add(MaterialTextureAtlas::from_grid(
            asset_server.load("spritesheet.png"),
            UVec2::splat(16),
            2,
            2,
            None,
            None,
        ))),
        TimeStuff { timer: 1.0 },
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::IDENTITY,
    ));
}

#[derive(Component)]
pub struct TimeStuff {
    pub timer: f32,
}

pub fn time(
    mut materials: ResMut<Assets<MaterialTextureAtlas>>,
    mut q: Query<(&MeshMaterial3d<MaterialTextureAtlas>, &mut TimeStuff)>,
    time: Res<Time>,
) {
    for (mat_handle, mut timer) in q.iter_mut() {
        timer.timer -= time.delta_secs();
        if timer.timer <= 0.0 {
            info!("Time");
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                mat.index += 1;
                if mat.index > 3 {
                    mat.index = 0;
                }
                let i = (mat.index as usize).min(mat.textures.len() - 1);
                let r = mat.textures[i];

                mat.params.crop_offset_px = Vec2::new(r.min.x as f32, r.min.y as f32);
                mat.params.crop_size_px =
                    Vec2::new((r.max.x - r.min.x) as f32, (r.max.y - r.min.y) as f32);
            }
            timer.timer = 1.0;
        }
    }
}
