use bevy::{render::render_resource::{ShaderStage, ShaderStages}, prelude::Plugin};

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup_background);
    }
}

/// A transparent material that should scramble background?
pub struct NoiseMaterial;

pub fn setup_background(
    commands: &mut Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    window: Res<WindowDescriptor>,
) {
}