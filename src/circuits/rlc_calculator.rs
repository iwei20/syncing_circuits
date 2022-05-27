type Float = f32;

#[derive(Debug)]
/// Helper struct for calculating RLC series circuit current.
pub struct RLCCalculator {
    pub startcharge: Float,
    pub resistance: Float,
    pub inductance: Float,
    pub capacitance: Float,
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
    pub fn with_constants(startcharge: Float, resistance: Float, inductance: Float, capacitance: Float) -> Self {
        RLCCalculator {
            startcharge,
            resistance,
            inductance,
            capacitance,
        }
    }

    /// Omega prime, or the angular frequency of an RLC circuit;
    /// sqrt(w^2 - (R/2L)^2)
    fn angular_freq(&self) -> Float {
        let w_squared = (self.inductance * self.capacitance).recip();
        let modifier = self.resistance * 0.5 * self.inductance.recip();
        (w_squared - modifier * modifier).sqrt()
    }

    /// Returns the current running through the represented RLC series circuit at the given time.
    /// Calculated by w'Rq0/(2L) e^{-Rt/2L} sin(w't)
    /// 
    /// # Arguments
    /// * `t` - the time from the start time at which the capacitor had charge q0.
    /// 
    /// # Returns
    /// * A `f32` giving the current running through the circuit, where positive current runs in the direction from the negative plate to the positive plate.
    /// 
    /// i.e.,
    /// ```text
    ///              i
    ///         - + -->
    ///     ----| |----
    ///     |    C    |
    ///     L         |
    ///     |         |
    ///     ----R------
    /// ```

    pub fn current(&self, t: Float) -> Float {
        let half_sqrt_l = 0.5 * self.inductance.recip();
        let c = self.angular_freq() * self.resistance * self.startcharge * half_sqrt_l;
        let tau = -self.resistance * half_sqrt_l;
        c * (tau * t).exp() * (self.angular_freq() * t).sin()
    } 


}
