// Síntese de áudio biofísico e monitorização hospitalar em Rust/WASM via Web Audio API.
use web_sys::{AudioContext, AudioNode, BiquadFilterType, OscillatorType};

pub struct AudioManager {
    ctx: Option<AudioContext>,
}

impl AudioManager {
    pub fn new() -> Self {
        // AudioContext é instanciado na primeira chamada ou sob demanda
        let ctx = AudioContext::new().ok();
        Self { ctx }
    }

    fn ensure_context(&mut self) -> Option<&AudioContext> {
        if self.ctx.is_none() {
            self.ctx = AudioContext::new().ok();
        }
        if let Some(ref ctx) = self.ctx {
            let _ = ctx.resume();
        }
        self.ctx.as_ref()
    }

    // Bip da UTI (Monitor Multiparamétrico Hospitalar - Onda R)
    pub fn play_uti_beep(&mut self) {
        if let Some(ctx) = self.ensure_context() {
            let now = ctx.current_time();
            let duration = 0.075; // 75 ms

            if let (Ok(osc), Ok(gain)) = (ctx.create_oscillator(), ctx.create_gain()) {
                osc.set_type(OscillatorType::Sine);
                osc.frequency().set_value(880.0); // Lá 5 (880 Hz) clássico hospitalar

                // Envelope sonoro com ataque de 2ms e decaimento exponencial
                let _ = gain.gain().set_value_at_time(0.0001, now);
                let _ = gain.gain().linear_ramp_to_value_at_time(0.12, now + 0.003);
                let _ = gain.gain().exponential_ramp_to_value_at_time(0.0001, now + duration);

                if let Ok(_) = osc.connect_with_audio_node(&gain) {
                    if let Ok(_) = gain.connect_with_audio_node(&ctx.destination() as &AudioNode) {
                        let _ = osc.start_with_when(now);
                        let _ = osc.stop_with_when(now + duration + 0.01);
                    }
                }
            }
        }
    }

    // 1ª Bulha Cardíaca (B1 - "Tum" / Fechamento da Valva Mitral)
    // Sintetizada com pitch sweep de 140 Hz para 85 Hz e ganho calibrado para transdutores de celulares/laptops
    pub fn play_b1(&mut self) {
        if let Some(ctx) = self.ensure_context() {
            let now = ctx.current_time();
            let duration = 0.125; // 125 ms

            if let (Ok(osc), Ok(filter), Ok(gain)) = (ctx.create_oscillator(), ctx.create_biquad_filter(), ctx.create_gain()) {
                osc.set_type(OscillatorType::Triangle);
                // Pitch sweep descendente simula o impacto mecânico valvar e tensão miocárdica
                let _ = osc.frequency().set_value_at_time(140.0, now);
                let _ = osc.frequency().exponential_ramp_to_value_at_time(85.0, now + duration);

                filter.set_type(BiquadFilterType::Lowpass);
                filter.frequency().set_value(260.0);
                filter.q().set_value(1.5);

                let _ = gain.gain().set_value_at_time(0.0001, now);
                let _ = gain.gain().linear_ramp_to_value_at_time(0.55, now + 0.012);
                let _ = gain.gain().exponential_ramp_to_value_at_time(0.0001, now + duration);

                if osc.connect_with_audio_node(&filter).is_ok()
                    && filter.connect_with_audio_node(&gain).is_ok()
                    && gain.connect_with_audio_node(&ctx.destination() as &AudioNode).is_ok()
                {
                    let _ = osc.start_with_when(now);
                    let _ = osc.stop_with_when(now + duration + 0.01);
                }
            }
        }
    }

    // 2ª Bulha Cardíaca (B2 - "Tá" / Fechamento da Valva Aórtica / Incisura Dicrótica)
    // Sintetizada com estalido de 240 Hz descendente para 160 Hz, mais curto e seco
    pub fn play_b2(&mut self) {
        if let Some(ctx) = self.ensure_context() {
            let now = ctx.current_time();
            let duration = 0.065; // 65 ms

            if let (Ok(osc), Ok(filter), Ok(gain)) = (ctx.create_oscillator(), ctx.create_biquad_filter(), ctx.create_gain()) {
                osc.set_type(OscillatorType::Sine);
                // Estalido aórtico com componente de maior frequência
                let _ = osc.frequency().set_value_at_time(240.0, now);
                let _ = osc.frequency().exponential_ramp_to_value_at_time(160.0, now + duration);

                filter.set_type(BiquadFilterType::Bandpass);
                filter.frequency().set_value(220.0);
                filter.q().set_value(1.2);

                let _ = gain.gain().set_value_at_time(0.0001, now);
                let _ = gain.gain().linear_ramp_to_value_at_time(0.42, now + 0.006);
                let _ = gain.gain().exponential_ramp_to_value_at_time(0.0001, now + duration);

                if osc.connect_with_audio_node(&filter).is_ok()
                    && filter.connect_with_audio_node(&gain).is_ok()
                    && gain.connect_with_audio_node(&ctx.destination() as &AudioNode).is_ok()
                {
                    let _ = osc.start_with_when(now);
                    let _ = osc.stop_with_when(now + duration + 0.01);
                }
            }
        }
    }
}
