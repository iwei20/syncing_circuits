use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Component)]
pub struct CircuitComponent<'a, C: CircuitComponentType> {
    ends_positions: (Transform, Transform),
    ends_connected: 
        (
            Vec<&'a CircuitComponent>, Vec<&'a Self>),
    phantom_cct: PhantomData<C>
}

/// A trait that shows
trait CircuitComponentType {}
pub struct Wire;
pub struct Resistor { pub resistance: f32 }
pub struct Capacitor { pub capacitance: f32 }
pub struct Inductor { pub inductance: f32 }
pub struct IdealBattery { pub voltage: f32 }

impl<'a, C: CircuitComponentType> CircuitComponent<'a, C> {

    pub fn move_end_left(&mut self, dx: f32, dy: f32) {
        self.ends_positions.0.translation.x += dx;
        self.ends_positions.0.translation.y += dy;
    }

    pub fn move_end_right(&mut self, dx: f32, dy: f32) {
        self.ends_positions.1.translation.x += dx;
        self.ends_positions.1.translation.y += dy;
    }

    pub fn connect_left<CO: CircuitComponentType>(&mut self, other_component: &'a CircuitComponent<CO>) {
        self.ends_connected.0.push(other_component);
    }

    pub fn connect_right<CO: CircuitComponentType>(&mut self, other_component: &'a CircuitComponent<CO>) {
        self.ends_connected.1.push(other_component);
    }

    pub fn connect<C0: CircuitComponentType, C1: CircuitComponentType>(left: &mut CircuitComponent<C0>, right: &mut CircuitComponent<C1>) {
        left.connect_right(right);
        right.connect_left(left);
    }
}

