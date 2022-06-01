use bevy::{render::{render_resource::{ShaderStage, ShaderStages}, render_asset::{RenderAsset, PrepareAssetError}}, prelude::Plugin, sprite::Material2d, asset::Asset, reflect::TypeUuid, ecs::system::SystemParamItem};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
    }
}

/// A transparent material that should scramble background
#[derive(TypeUuid, Clone)]
#[uuid = "cd3d98e9-bc74-4e0b-9f2e-cd9372bfcdcb"]
pub struct NoiseMaterial;

pub struct NoiseMaterialGPU;

impl Material2d for NoiseMaterial {
    fn bind_group(material: &<Self as RenderAsset>::PreparedAsset) -> &bevy::render::render_resource::BindGroup {
        todo!()
    }

    fn bind_group_layout(render_device: &bevy::render::renderer::RenderDevice) -> bevy::render::render_resource::BindGroupLayout {
        todo!()
    }
}

impl RenderAsset for NoiseMaterial {
    type ExtractedAsset = NoiseMaterial;
    type PreparedAsset = NoiseMaterialGPU;
    type Param = ();

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        param: &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        Ok(NoiseMaterialGPU)
    }
}