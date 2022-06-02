use bevy::{render::{render_resource::{BindGroup, BindGroupLayoutDescriptor, BindGroupDescriptor, Buffer, BindGroupLayoutEntry, ShaderStages, BindingType, BufferBindingType, BufferSize, BindGroupEntry, BufferDescriptor, BufferUsages}, render_asset::{RenderAsset, PrepareAssetError}, renderer::{RenderDevice, RenderQueue}, RenderStage}, prelude::{Plugin, ResMut, Assets, Mesh, Commands, shape, Transform, default, Res, Component, Bundle, Query}, sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle}, reflect::TypeUuid, ecs::{system::{SystemParamItem, lifetimeless::SRes}, event::Events}, window::{Windows, WindowResized}, math::Vec3, core::Time};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(Material2dPlugin::<NoiseMaterial>::default())
           .add_startup_system(spawn_foreground)
           .add_system(resize_notificator);
    }
}

#[derive(Bundle)]
pub struct ForegroundBundle {
    foreground: Foreground,
    #[bundle]
    mesh2dbundle: MaterialMesh2dBundle<NoiseMaterial>
}

#[derive(Component)]
pub struct Foreground;

fn spawn_foreground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NoiseMaterial>>,
    windows: Res<Windows>
) {
    let window = windows.get_primary().unwrap();
    commands.spawn_bundle(
        ForegroundBundle {
            foreground: Foreground,
            mesh2dbundle: MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                material: materials.add(NoiseMaterial),
                transform: Transform::from_scale(Vec3::new(window.width(), window.height(), 25.0)).with_translation(Vec3::new(0.0, 0.0, 10.0)),
                ..default()
            }
        }
    );
}

fn resize_notificator(resize_event: Res<Events<WindowResized>>, mut query: Query<(&Foreground, &mut Transform)>) {
    let mut reader = resize_event.get_reader();
    for (_foreground, mut transform) in query.iter_mut() {
        for e in reader.iter(&resize_event) {
            *transform = Transform::from_scale(Vec3::new(e.width, e.height, 25.0)).with_translation(Vec3::new(0.0, 0.0, 10.0));
        }
    }
}
/// A transparent material that should scramble background
#[derive(TypeUuid, Clone)]
#[uuid = "cd3d98e9-bc74-4e0b-9f2e-cd9372bfcdcb"]
pub struct NoiseMaterial;

pub struct NoiseMaterialGPU {
    bind_group: BindGroup,
    time_buffer: Buffer,
    time_group: BindGroup
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
        asset_server.watch_for_changes().unwrap();
        Some(asset_server.load("random.wgsl"))
    }
}

impl RenderAsset for NoiseMaterial {
    type ExtractedAsset = Self;
    type PreparedAsset = NoiseMaterialGPU;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<Self>>, SRes<RenderQueue>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        _extracted_asset: Self::ExtractedAsset,
        (render_device, pipeline, render_queue): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[]
        });
        let time_buffer = render_device.create_buffer(&BufferDescriptor {
            label: None,
            size: std::mem::size_of::<f32>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false
        });
        render_queue.write_buffer(&time_buffer, 0, bevy::core::cast_slice(&[0f32]));
        let time_group_layout_descriptor = BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer { 
                        ty: BufferBindingType::Uniform, 
                        has_dynamic_offset: false, 
                        min_binding_size: BufferSize::new(std::mem::size_of::<f32>() as u64), 
                    },
                    count: None,
                }
            ],
        };
        let time_group_layout = render_device.create_bind_group_layout(&time_group_layout_descriptor);
        let time_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &time_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 1,
                    resource: time_buffer.as_entire_binding(),
                }
            ],
        });

        Ok(NoiseMaterialGPU {
            bind_group,
            time_buffer,
            time_group
        })
    }
}