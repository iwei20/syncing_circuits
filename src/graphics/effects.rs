use bevy::{
    core::Time,
    ecs::{
        event::Events,
        system::{lifetimeless::SRes, SystemParamItem},
    },
    math::Vec3,
    prelude::{
        default, shape, Assets, Bundle, Commands, Component, Mesh, Plugin, Query, Res, ResMut,
        Transform,
    },
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
            BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferInitDescriptor,
            BufferSize, BufferUsages, ShaderStages,
        },
        renderer::{RenderDevice, RenderQueue},
        RenderApp, RenderStage,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle},
    window::{WindowResized, Windows},
};

use super::DLRCCircuit;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(Material2dPlugin::<NoiseMaterial>::default())
            .add_startup_system(spawn_foreground)
            .add_system(resize_notificator);

        app.sub_app_mut(RenderApp)
            .add_system_to_stage(RenderStage::Extract, extract_info)
            .add_system_to_stage(RenderStage::Prepare, prepare_material);
    }
}

#[derive(Bundle)]
pub struct ForegroundBundle {
    foreground: Foreground,
    #[bundle]
    mesh2dbundle: MaterialMesh2dBundle<NoiseMaterial>,
}

#[derive(Component)]
pub struct Foreground;

fn spawn_foreground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NoiseMaterial>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    commands.spawn_bundle(ForegroundBundle {
        foreground: Foreground,
        mesh2dbundle: MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            material: materials.add(NoiseMaterial {
                time: 0.0,
                alpha: 0.75,
            }),
            transform: Transform::from_scale(Vec3::new(window.width(), window.height(), 25.0))
                .with_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        },
    });
}

fn resize_notificator(
    resize_event: Res<Events<WindowResized>>,
    mut query: Query<(&Foreground, &mut Transform)>,
) {
    let mut reader = resize_event.get_reader();
    for (_foreground, mut transform) in query.iter_mut() {
        for e in reader.iter(&resize_event) {
            *transform = Transform::from_scale(Vec3::new(e.width, e.height, 25.0))
                .with_translation(Vec3::new(0.0, 0.0, 10.0));
        }
    }
}
/// A transparent material that should scramble background
#[derive(TypeUuid, Clone, AsStd140)]
#[uuid = "cd3d98e9-bc74-4e0b-9f2e-cd9372bfcdcb"]
pub struct NoiseMaterial {
    time: f32,
    alpha: f32,
}

pub struct NoiseMaterialGPU {
    bind_group: BindGroup,
    buffer: Buffer,
}

impl Material2d for NoiseMaterial {
    fn bind_group(
        material: &<Self as RenderAsset>::PreparedAsset,
    ) -> &bevy::render::render_resource::BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(
        render_device: &bevy::render::renderer::RenderDevice,
    ) -> bevy::render::render_resource::BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Noise Material"),
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(NoiseMaterial::std140_size_static() as u64),
                },
                count: None,
            }],
        })
    }

    fn fragment_shader(
        asset_server: &bevy::prelude::AssetServer,
    ) -> Option<bevy::prelude::Handle<bevy::prelude::Shader>> {
        asset_server.watch_for_changes().unwrap();
        Some(asset_server.load("random.wgsl"))
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
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("Time Buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            contents: extracted_asset.as_std140().as_bytes(),
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: Some("Noise Material"),
            layout: &pipeline.material2d_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Ok(NoiseMaterialGPU { bind_group, buffer })
    }
}

#[derive(AsStd140)]
struct ExtractedInfo {
    seconds_since_startup: f32,
    power: f32,
}

fn extract_info(mut commands: Commands, time: Res<Time>, query_circs: Query<&DLRCCircuit>) {
    let total_power = query_circs
        .iter()
        .map(|circuit| circuit.0.lightbulb_power())
        .sum::<f64>() as f32;
    commands.insert_resource(ExtractedInfo {
        seconds_since_startup: time.seconds_since_startup() as f32,
        power: total_power,
    });
}

fn prepare_material(
    extracted_info: Res<ExtractedInfo>,
    mut materials: ResMut<RenderAssets<NoiseMaterial>>,
    render_queue: Res<RenderQueue>,
) {
    for material in materials.values_mut() {
        render_queue.write_buffer(&material.buffer, 0, extracted_info.as_std140().as_bytes());
    }
}
