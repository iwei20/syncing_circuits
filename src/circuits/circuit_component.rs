use std::sync::atomic::AtomicPtr;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

/// A CircuitComponent's physical representation is a rigid, straight circuit element. Yes, this means wires represented by this can only be straight. Just connect multiple.
/// This struct is meant to aid in both rendering circuit elements and providing a graph node-like interface for circuit loop finding. 
#[derive(Component)]
pub struct CircuitComponent {
    comp_type: CircuitComponentType,
    ends_positions: (Transform, Transform),
    ends_connected: (Vec<AtomicPtr<Self>>, Vec<AtomicPtr<Self>>)
}

/// An enum that stores the relevant measure of information for this CircuitComponent
pub enum CircuitComponentType {
    Wire,
    Resistor { resistance: f32 },
    Capacitor { capacitance: f32 },
    Inductor { inductance: f32 },
    IdealBattery { voltage: f32 }
}

impl CircuitComponent {

    /// Creates a free-floating circuit component with the given type with the ends at the given component.
    pub fn new(comp_type: CircuitComponentType, ends_positions: (Transform, Transform)) -> Self {
        Self {
            comp_type,
            ends_positions,
            ends_connected: (Vec::new(), Vec::new())
        }
    }

    /// Applies a displacement to the stored position of the "left" end of this component; this may not actually be the left end, but is just one end we will describe as left.
    pub fn move_end_left(&mut self, dx: f32, dy: f32) {
        self.ends_positions.0.translation.x += dx;
        self.ends_positions.0.translation.y += dy;
    }

    /// Applies a displacement to the stored position of the "right" end of this component; this may not actually be the right end, but is just one end we will describe as right.
    pub fn move_end_right(&mut self, dx: f32, dy: f32) {
        self.ends_positions.1.translation.x += dx;
        self.ends_positions.1.translation.y += dy;
    }

    /// Adds a pointer to another component to the "left" end of this component; this may not actually be the left end, but is just one end we will describe as left.
    pub fn connect_left(&mut self, other_component: &mut Self) {
        self.ends_connected.0.push(AtomicPtr::new(other_component));
    }

    /// Returns all the components connected to the "left" end of this component; this may not actually be the left end, but is just one end we will describe as left.
    pub fn left_connections(&self) -> &Vec<AtomicPtr<CircuitComponent>> {
        &self.ends_connected.0
    }

    /// Adds a pointer to another component to the "right" end of this component; this may not actually be the right end, but is just one end we will describe as right.
    pub fn connect_right(&mut self, other_component: &mut Self) {
        self.ends_connected.1.push(AtomicPtr::new(other_component));
    }

    /// Returns all the components connected to the "right" end of this component; this may not actually be the right end, but is just one end we will describe as right.
    pub fn right_connections(&self) -> &Vec<AtomicPtr<CircuitComponent>> {
        &self.ends_connected.1
    }

    /// Makes two components point to each other; the left component will point to the right, and vice versa.
    pub fn connect(left: &mut Self, right: &mut Self) {
        left.connect_right(right);
        right.connect_left(left);
    }

    /// Returns the 2d mesh of this object at its location.
    pub fn render(current: f32) {
        todo!()
    }
}

