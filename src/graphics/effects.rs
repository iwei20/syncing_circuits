use bevy::{render::{render_resource::{ShaderStage, ShaderStages, BindGroup, BindGroupLayoutDescriptor, BindGroupDescriptor}, render_asset::{RenderAsset, PrepareAssetError}, renderer::RenderDevice}, prelude::{Plugin, ResMut, Assets, Mesh, Commands, shape, Transform, default, Res}, sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle}, asset::Asset, reflect::TypeUuid, ecs::system::{SystemParamItem, lifetimeless::SRes}, pbr::MaterialPipeline, window::{Windows, Window}, math::Vec3};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(Material2dPlugin::<NoiseMaterial>::default())
           .add_startup_system(spawn_foreground);
    }
}

fn spawn_foreground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NoiseMaterial>>,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        material: materials.add(NoiseMaterial),
        transform: Transform::from_scale(Vec3::new(window.width(), window.height(), 25.0)).with_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
}
/// A transparent material that should scramble background
#[derive(TypeUuid, Clone)]
#[uuid = "cd3d98e9-bc74-4e0b-9f2e-cd9372bfcdcb"]
pub struct NoiseMaterial;

pub struct NoiseMaterialGPU {
    bind_group: BindGroup
}


impl Material2d for NoiseMaterial {
    fn bind_group(material: &<Self as RenderAsset>::PreparedAsset) -> &bevy::render::render_resource::BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &bevy::render::renderer::RenderDevice) -> bevy::render::render_resource::BindGroupLayout {
        render_device.create_bind_group_layout( &BindGroupLayoutDescriptor {
            label: None,
            entries: &[]
        })
    }

    fn fragment_shader(asset_server: &bevy::prelude::AssetServer) -> Option<bevy::prelude::Handle<bevy::prelude::Shader>> {
        Some(asset_server.load("random.frag"))
    }
}

impl RenderAsset for NoiseMaterial {
    type ExtractedAsset = Self;
    type PreparedAsset = NoiseMaterialGPU;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<Self>>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[]
        });
        Ok(NoiseMaterialGPU {bind_group})
    }
}