mod am_mod {
    use std::f64::consts::PI as π;
    
    
    pub struct Ranged {
        minimum: Option<f64>,
        maximum: Option<f64>,
        value: Option<f64>,
        default: f64,
    }
    
    impl Ranged {
        pub fn from_minimum_and_maximum(minimum: Option<f64>, maximum: Option<f64>) -> Self {
            let default = match minimum {
                Some(value) => value,
                None => 0.0
            };

            Self {
                minimum,
                maximum,
                value: None,
                default,
            }
        }
        
        pub fn create_unbounded(value: Option<f64>) -> Self {
            Self {
                minimum: None,
                maximum: None,
                value,
                default: 0.0,
            }
        }
        
        pub fn create_positive(value: Option<f64>) -> Self {
            Self {
                minimum: Some(0.0),
                maximum: None,
                value,
                default: 0.0,
            }
        }
        
        pub fn is_okay_to_set(&self, value: Option<f64>) -> bool {
            if Self::is_prop_valid(value) { return false }
            if Self::is_prop_valid(self.minimum) || value < self.minimum { return false }
            if Self::is_prop_valid(self.maximum) || value > self.maximum { return false }
            true
        }

        pub fn get(&self) -> f64 {
            match self.is_this_valid() {
                true => self.value.expect("This should be valid."),
                false => self.default,
            }
        }
        
        pub fn set(&mut self, value: f64) {
            match Self::is_prop_valid(Some(value)) {
                true => { self.value = Some(value) },
                false => (),
            }
        }

        pub fn set_default(&mut self, default: f64) {
            self.default = default;
        }
        
        pub fn is_this_valid(&self) -> bool {
            self.is_okay_to_set(self.value)
        }

        fn is_prop_valid(value: Option<f64>) -> bool {
            match value {
                Some(value) => {
                    value.is_normal()
                },
                None => false
            }
        }
    }

    pub enum Waveform {
        Flat,
        Sine { frequency: Ranged },
        Square { frequency: Ranged },
        Triangle { frequency: Ranged },
        Sawtooth { frequency: Ranged },
    }
    
    impl Waveform {
        pub fn get_ratio_at_time_and_frequency(&self, time: f64) -> f64 {
            match self {
                Self::Flat => 1.0,
                Self::Sine { frequency } => 
                    Self::get_sine_wave_point_at_time_and_frequency(time, frequency.get()),
                Self::Square { frequency } =>
                    Self::get_square_wave_point_at_time_any_frequency(time, frequency.get()),
                Self::Triangle { frequency } =>
                    Self::get_triangle_wave_point_at_time_any_frequency(time, frequency.get()),
                Self::Sawtooth { frequency } =>
                    Self::get_sawtooth_wave_point_at_time_any_frequency(time, frequency.get()),
            }
        }
            
        fn dc_wave() -> Self {
            Self::Flat
        }

        fn sine_wave(frequency: f64) -> Self {
            Self::Sine { frequency: Ranged::create_positive(Some(frequency)) }
        }

        fn square_wave(frequency: f64) -> Self {
            Self::Square { frequency: Ranged::create_positive(Some(frequency)) }
        }
        

        fn triangle_wave(frequency: f64) -> Self {
            Self::Triangle { frequency: Ranged::create_positive(Some(frequency)) }
        }

        fn sawtooth_wave(frequency: f64) -> Self {
            Self::Sawtooth { frequency: Ranged::create_positive(Some(frequency)) }
        }
        
        fn recreate_at_new_frequency(&self, new_frequency: f64) -> Self {
            match self {
                Waveform::Flat => Waveform::Flat,
                Waveform::Sawtooth { frequency: _ } => 
                    Waveform::Sawtooth { frequency: Ranged::create_positive(Some(new_frequency)) },
                Waveform::Triangle {frequency: _} => 
                    Waveform::Triangle { frequency: Ranged::create_positive(Some(new_frequency)) },
                Waveform::Sine { frequency: _ } => 
                    Waveform::Sine { frequency: Ranged::create_positive(Some(new_frequency)) },
                Waveform::Square { frequency: _ } => 
                    Waveform::Square { frequency: Ranged::create_positive(Some(new_frequency)) },
            }
        }
                


        fn get_sine_wave_point_at_time_and_frequency(time: f64, frequency: f64) -> f64 {
            let ωt = Self::get_ωt(time, frequency);
            ωt.cos()
        }
        
        fn get_square_wave_point_at_time_any_frequency(time: f64, frequency: f64) -> f64 {
            let ωt = Self::get_ωt(time, frequency);
            match ωt.is_sign_positive() {
                true => 1.0,
                false => 0.0,
            }
        }

        fn get_triangle_wave_point_at_time_any_frequency(time: f64, frequency: f64) -> f64 {
            let ωt = Self::get_ωt(time, frequency);
            1.0 - ((2.0/π) * ωt.acos())
        }

        fn get_sawtooth_wave_point_at_time_any_frequency(time: f64, frequency: f64) -> f64 {
            let p = time.recip();
            let time = time % (p / 2.0);
            Self::get_triangle_wave_point_at_time_any_frequency(time, frequency / 2.0)
        }
        
        fn get_ωt(time: f64, frequency:f64) -> f64 {
            Self::get_angular_frequency(frequency) * time
        }
        
        fn get_angular_frequency(frequency: f64) -> f64 {
            frequency * 2.0 * π
        }
    }

    pub struct Signal {
        frequency: Ranged,
        amplitude: Ranged,
        waveform: Waveform,
        enabled: bool,
    }
    
    impl Signal {
        pub fn from_frequency_range(minimum: f64, maximum: f64) -> Self {
            Self {
                frequency: Ranged::from_minimum_and_maximum(Some(minimum), Some(maximum)),
                amplitude: Ranged::create_unbounded(Some(0.0)),
                waveform: Waveform::Flat,
                enabled: false,
            }
        }
        
        pub fn set_frequency(&mut self, value_in_hz: f64) {
            self.frequency.set(value_in_hz);
            self.waveform = self.waveform.recreate_at_new_frequency(value_in_hz);
        }
        
        pub fn get_frequency(&self) -> f64 {
            self.frequency.get()
        }
        
        pub fn set_waveform(&mut self, waveform: Waveform) {
            self.waveform = waveform;
        }
        
        pub fn set_amplitude(&mut self, value_in_volts: f64) {
            self.amplitude.set(value_in_volts);
        }
        
        pub fn get_amplitude(&self) -> f64 {
            self.amplitude.get()
        }
        
        pub fn enable(&mut self) {
            self.enabled = true;
        }
        
        pub fn disable(&mut self) {
            self.enabled = false;
        }
        
        pub fn get_signal_at_time(&self, time: f64) -> f64 {
            match self.enabled {
                true => self.waveform.get_ratio_at_time_and_frequency(time) * self.get_amplitude(),
                false => 0.0
            }
        }
    }

    pub struct AmSignal {
        carrier: Signal,
        message: Signal,
        modindex: Ranged,
    }

    impl AmSignal {
        pub fn from_carrier(frequency: f64) -> Self {
            Self {
                carrier: Self::setup_carrier(frequency),
                message: Signal::from_frequency_range(0.0, 20e3),
                modindex: {
                    let mut r = Ranged::from_minimum_and_maximum(Some(0.0), Some(2.0));
                    r.set(1.0); r
                },
            }
        }
        
        fn setup_carrier(frequency: f64) -> Signal {
            let mut carrier = Signal::from_frequency_range(535e3, 1605e3);
            carrier.set_waveform(Waveform::sine_wave(frequency));
            carrier.set_amplitude(1.0);
            carrier.enable();
            carrier
        }
        
        pub fn set_carrier_amplitude(&mut self, value_in_volts: f64) {
            self.carrier.set_amplitude(value_in_volts);
        }

        pub fn set_modulation_frequency(&mut self, value_in_hz: f64) {
            self.message.set_frequency(value_in_hz);
        }
        
        pub fn enable_modulation(&mut self) {
            self.message.enable();
        }

        pub fn disable_modulation(&mut self) {
            self.message.disable();
        }

        pub fn set_modulation_index(&mut self, mod_index: f64) {
            self.modindex.set(mod_index);
        }
        
        pub fn get_amplitude_at_time(&self, time: f64) -> f64 {
            let mod_signal = self.message.get_signal_at_time(time) * self.modindex.get();
            self.carrier.get_signal_at_time(time) + mod_signal
        }
    }
}


fn main() {
    let am = am_mod::AmSignal::from_carrier(1e6);
    let time_t = 2.3e-6;
    let voltage_at_t =  am.get_amplitude_at_time(time_t);
    println!("Voltage at time {time_t} is {voltage_at_t}");
}
