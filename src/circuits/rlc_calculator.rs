type Float = f64;

#[derive(Debug)]
/// Helper struct for calculating RLC series circuit current.
pub struct RLCCalculator {
    pub startcharge: Float,
    pub resistance: Float,
    pub inductance: Float,
    pub capacitance: Float,
    time_since_first_tick: Float,
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
            time_since_first_tick: 0.0,
            //should get set later when time_since_first_tick is 0
            q: 0.0,
            dqdt: 0.0,
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
        if self.time_since_first_tick == 0.0 {
            self.q = self.startcharge;
            self.dqdt = 0.0;
            let w_squared = (self.inductance * self.capacitance).recip();
            let modifier = self.resistance * 0.5 * self.inductance.recip();
            //-Qw^2
            self.d2qdt2 = -self.startcharge * (w_squared - modifier * modifier);
        }

        const FIDELITY: i32 = 100;
        let dt = delta_t / FIDELITY as Float;
        let mut new_q = self.q;
        let mut new_dqdt = self.dqdt;
        let mut new_d2qdt2 = self.d2qdt2;

        for _ in 0..FIDELITY {
            new_q += new_dqdt * dt;
            new_dqdt += new_d2qdt2 * dt;
            new_d2qdt2 = -(new_q / self.capacitance + self.resistance * new_dqdt) / self.inductance;
        }
        
        self.q = new_q;
        self.dqdt = new_dqdt;
        self.d2qdt2 = new_d2qdt2;
        self.time_since_first_tick += delta_t;
    }

    ///starts the simulation over from scratch
    pub fn reset(&mut self) {
        self.time_since_first_tick = 0.0;
    }
}
