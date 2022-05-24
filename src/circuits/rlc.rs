type Float = f32;

#[derive(Debug)]
pub struct Rlc {
    resistance: Float,
    inductance: Float,
    capacitance: Float,
}

impl Rlc {
    pub fn with_constants(resistance: Float, inductance: Float, capacitance: Float) -> Self {
        Rlc {
            resistance,
            inductance,
            capacitance,
        }
    }

    pub fn new() -> Self {
        Rlc {
            resistance: Float::default(),
            inductance: Float::default(),
            capacitance: Float::default(),
        }
    }

    ///this is the omega prime of the circuit 
    fn angular_freq(&self) -> Float {
        let w_squared = (self.inductance * self.capacitance).recip();
        let modifier = self.resistance * 0.5 * self.inductance.recip();
        (w_squared - modifier * modifier).sqrt()
    }

    ///q0 is the intial charge on the capacitor
    ///t is the time from the start time at which the capacitor had charge q0
    ///this sends the magnitude and direction of the current,
    ///positave going to the right
    ///below shows the circuit a immediantly after a positive charge q0 was put in the capacitor
    ///
    ///            i
    ///           -->
    ///     ----C-----
    ///     |        |
    ///     L        |
    ///     |        |
    ///     ----R-----
    pub fn current(&self, q0: Float, t: Float) -> Float {
        //wRq0/(2L) e^{-Rt/2L} sin(wt + phi)
        let half_sqrt_l = 0.5 * self.inductance.recip();
        let c = self.angular_freq() * self.resistance * q0 * half_sqrt_l;
        let tau = -self.resistance * half_sqrt_l;
        c * (tau * t).exp() * (self.angular_freq() * t).sin()
    } 


}
