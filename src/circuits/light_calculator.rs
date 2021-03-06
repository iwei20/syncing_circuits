use super::RLCCalculator;
type Float = f64;

#[derive(Debug)]
/// Helper struct for calculating the power of a lightbulb connected to a RLC series circuit.
pub struct DisconnectLightCircuitCalculator {
    pub circuit: RLCCalculator,
}

impl DisconnectLightCircuitCalculator {
    /// Returns a calculator representing an RLC series circuit connected to a lightbulb with the given constants and phase 0.
    ///
    /// # Arguments
    ///
    /// * `startcharge` - Q0, the starting and maximum charge present on the capacitor.
    /// * `resistance` - R, the resistance.
    /// * `inductance` - L, the inductance.
    /// * `capacitance` - C, the capacitance.
    ///
    /// # Returns
    /// A `DisconnectLightCircuitCalculator` representing an RLC series circuit connected to a lightbulb with the given constants and phase 0.
    pub fn with_constants(
        startcharge: Float,
        resistance: Float,
        inductance: Float,
        capacitance: Float,
    ) -> Self {
        Self {
            circuit: RLCCalculator::with_constants(
                startcharge,
                resistance,
                inductance,
                capacitance,
            ),
        }
    }

    /// Consumes a RLCCalculator to return a calculator representing that RLC circuit connected to a lightbulb.
    ///
    /// # Arguments
    ///
    /// * `circuit` - the RLC calculator to be consumed.
    ///
    /// # Returns
    /// A `DisconnectLightCircuitCalculator` representing the given RLC circuit connected to a lightbulb.
    pub fn from_rlc(circuit: RLCCalculator) -> Self {
        Self { circuit }
    }

    /// Calculates the current power, in watts, in the circuit
    ///
    ///# Returns
    ///A floating point number representing the power through the resistor in the RLC circuit
    pub fn lightbulb_power(&self) -> Float {
        let current = self.circuit.current();
        current * current * self.circuit.resistance
    }
}
