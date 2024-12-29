use std::f64::consts::PI as Π;
pub struct Transmitter {
    amplitude: f64,
    frequency: f64,
}

impl Transmitter {
    fn get_ω_in_rad(&self) -> f64 {
        2.0 * Π * self.frequency
    }
    
    fn get_ω_in_deg(&self) -> f64 {
        360.0 * self.frequency
    }
    
    fn set_amplitude(&mut self, amp: f64) {
        self.amplitude = amp;
    }
    
    fn set_frequency(&mut self, freq: f64) {
        self.frequency = freq;
    }
    
    fn get_signal_at_t(&self, t: f64) -> f64 {
        let ωt = self.get_ω_in_rad() * t;
        self.amplitude * ωt.cos()
    }
}
