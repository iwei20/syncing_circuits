use super::RLCCalculator;
type Float = f32;

pub struct DisconnectLightCircuitCalculator {
    circuit: RLCCalculator,
}

impl DisconnectLightCircuitCalculator {
    pub fn with_constants(startcharge: Float, resistance: Float, inductance: Float, capacitance: Float) -> Self {
        Self {
            circuit: RLCCalculator::with_constants(startcharge, resistance, inductance, capacitance)
        }
    }

    pub fn from_rlc(circuit: RLCCalculator) -> Self {
        Self {
            circuit
        }
    }

    pub fn lightbulb_power(&self, t: Float) -> Float {
        let current = self.circuit.current(t);
        current * current * self.circuit.resistance
    }
}
