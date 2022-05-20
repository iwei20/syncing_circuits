use bevy::prelude::*;

mod circuits;
use circuits::CircuitPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();

}
