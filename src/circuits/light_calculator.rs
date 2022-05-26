use bevy::prelude::info;

use super::RLCCalculator;
type Float = f32;

pub struct DisconnectLightCircuitCalculator {
    circuit: RLCCalculator,
}

impl DisconnectLightCircuitCalculator {
    pub fn with_constants(resistance: Float, inductance: Float, capacitance: Float) -> Self {
        Self {
            circuit: RLCCalculator::with_constants(resistance, inductance, capacitance)
        }
    }

    pub fn from_rlc(circuit: RLCCalculator) -> Self {
        Self {
            circuit
        }
    }

    pub fn lightbulb_power(&self, q0: Float, t: Float) -> Float {
        let current = self.circuit.current(q0, t);
        current * current * self.circuit.resistance
    }
}
