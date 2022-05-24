use super::Rlc;
type Float = f32;

pub struct DisconnectedLightbulbCircuit {
    circuit: Rlc,
}

impl DisconnectedLightbulbCircuit {
    pub fn with_constants(resistance: Float, inductance: Float, capacitance: Float) -> Self {
        Self {
            circuit: Rlc::with_constants(resistance, inductance, capacitance)
        }
    }

    pub fn from_rlc(circuit: Rlc) -> Self {
        Self {
            circuit
        }
    }

    pub fn lightbulb_power(&self, q0: Float, t: Float) -> Float {
        let current = self.circuit.current(q0, t);
        current * current * self.circuit.resistance
    }
}