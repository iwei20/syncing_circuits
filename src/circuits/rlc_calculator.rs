type Float = f32;

#[derive(Debug)]
/// Helper struct for calculating RLC series circuit current.
pub struct RLCCalculator {
    pub startcharge: Float,
    pub resistance: Float,
    pub inductance: Float,
    pub capacitance: Float,
    q: Float,
    dqdt: Float,
    d2qdt2: Float,
}

impl RLCCalculator {
    /// Returns a calculator representing an RLC series circuit with the given constants and phase 0.
    ///
    /// # Arguments
    ///
    /// * `startcharge` - Q0, the starting and maximum charge present on the capacitor.
    /// * `resistance` - R, the resistance.
    /// * `inductance` - L, the inductance.
    /// * `capacitance` - C, the capacitance.
    ///
    /// # Returns
    /// A `RLCCalculator` representing an RLC series circuit with the given constants and phase 0.
    pub fn with_constants(
        startcharge: Float,
        resistance: Float,
        inductance: Float,
        capacitance: Float,
    ) -> Self {
        RLCCalculator {
            startcharge,
            resistance,
            inductance,
            capacitance,
            q: startcharge,
            dqdt: 0.0,
            //initializing this to 0.0 is sketchy but might work ???
            d2qdt2: 0.0,
        }
    }

    ///resturns the current in the circuit
    pub fn current(&self) -> Float {
        -self.dqdt
    }

    ///increments the current circuit in time by delta_t
    ///try to keep delta_t small
    pub fn tick(&mut self, delta_t: Float) {
        let new_q = self.q + self.dqdt * delta_t;
        let new_dqdt = self.dqdt + self.d2qdt2 * delta_t;
        let new_d2qdt2 =
            -(self.q / self.capacitance + self.resistance * self.dqdt) / self.inductance;
        self.q = new_q;
        self.dqdt = new_dqdt;
        self.d2qdt2 = new_d2qdt2;
    }

    ///starts the simulation over from scratch
    pub fn reset(&mut self) {
        self.q = self.startcharge;
        self.dqdt = 0.0;
        //again with the sketchy starts
        self.d2qdt2 = 0.0;
    }
}
