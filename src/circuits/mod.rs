use bevy::prelude::*;

mod circuit_component;
pub use circuit_component::CircuitComponent;

pub struct CircuitPlugin;

impl Plugin for CircuitPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}