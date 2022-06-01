use bevy::{render::{render_resource::{ShaderStage, ShaderStages}, render_asset::RenderAsset}, prelude::Plugin, sprite::Material2d, asset::Asset, reflect::TypeUuid};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
    }
}

/// A transparent material that should scramble background
#[derive(TypeUuid)]
#[uuid = "cd3d98e9-bc74-4e0b-9f2e-cd9372bfcdcb"]
pub struct NoiseMaterial;

impl Material2d for NoiseMaterial {
    fn bind_group(material: &<Self as RenderAsset>::PreparedAsset) -> &bevy::render::render_resource::BindGroup {
        todo!()
    }

    fn bind_group_layout(render_device: &bevy::render::renderer::RenderDevice) -> bevy::render::render_resource::BindGroupLayout {
        todo!()
    }
}

impl Asset for NoiseMaterial {

}

impl RenderAsset for NoiseMaterial {

}