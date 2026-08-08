use super::game::state::SoundTrigger;

#[cfg(target_arch = "wasm32")]
mod wasm_backend {
    use super::SoundTrigger;

    #[link(wasm_import_module = "env")]
    extern "C" {
        fn play_sound_wall();
        fn play_sound_tick();
        fn play_sound_pop();
        fn play_sound_win();
        fn play_sound_loss();
    }

    pub struct SoundBackend;

    impl SoundBackend {
        pub async fn new() -> Self {
            Self
        }

        pub fn play(&self, trigger: SoundTrigger) {
            unsafe {
                match trigger {
                    SoundTrigger::WallPlace => play_sound_wall(),
                    SoundTrigger::BiomassTick => play_sound_tick(),
                    SoundTrigger::IsolationPop => play_sound_pop(),
                    SoundTrigger::WinFanfare => play_sound_win(),
                    SoundTrigger::LossAlert => play_sound_loss(),
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native_backend {
    use super::SoundTrigger;
    use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};

    pub struct SoundBackend {
        snd_wall: Option<Sound>,
        snd_tick: Option<Sound>,
        snd_pop: Option<Sound>,
        snd_win: Option<Sound>,
        snd_loss: Option<Sound>,
    }

    impl SoundBackend {
        pub async fn new() -> Self {
            let snd_wall = generate_wav_sound(44100, 0.08, |t| {
                let freq = 1200.0 - t * 8000.0;
                (t * freq * 2.0 * std::f32::consts::PI).sin() * (1.0 - t / 0.08)
            })
            .await;

            let snd_tick = generate_wav_sound(44100, 0.12, |t| {
                let freq = 220.0 + (t * 1500.0).sin() * 50.0;
                (t * freq * 2.0 * std::f32::consts::PI).sin() * (1.0 - t / 0.12)
            })
            .await;

            let snd_pop = generate_wav_sound(44100, 0.18, |t| {
                let freq = 400.0 - t * 1800.0;
                (t * freq * 2.0 * std::f32::consts::PI).sin() * (1.0 - t / 0.18)
            })
            .await;

            let snd_win = generate_wav_sound(44100, 0.45, |t| {
                let note = if t < 0.15 {
                    523.25
                } else if t < 0.30 {
                    659.25
                } else {
                    783.99
                };
                (t * note * 2.0 * std::f32::consts::PI).sin() * (1.0 - t / 0.45)
            })
            .await;

            let snd_loss = generate_wav_sound(44100, 0.40, |t| {
                let freq = 350.0 - t * 600.0;
                let square = if (t * freq * 2.0 * std::f32::consts::PI).sin() > 0.0 {
                    0.5
                } else {
                    -0.5
                };
                square * (1.0 - t / 0.40)
            })
            .await;

            Self {
                snd_wall,
                snd_tick,
                snd_pop,
                snd_win,
                snd_loss,
            }
        }

        pub fn play(&self, trigger: SoundTrigger) {
            let sound = match trigger {
                SoundTrigger::WallPlace => &self.snd_wall,
                SoundTrigger::BiomassTick => &self.snd_tick,
                SoundTrigger::IsolationPop => &self.snd_pop,
                SoundTrigger::WinFanfare => &self.snd_win,
                SoundTrigger::LossAlert => &self.snd_loss,
            };

            if let Some(snd) = sound {
                play_sound_once(snd);
            }
        }
    }

    async fn generate_wav_sound<F>(sample_rate: u32, duration_secs: f32, synth: F) -> Option<Sound>
    where
        F: Fn(f32) -> f32,
    {
        let num_samples = (sample_rate as f32 * duration_secs) as usize;
        let data_len = num_samples * 2;
        let total_len = 44 + data_len;

        let mut bytes = Vec::with_capacity(total_len);

        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&((total_len - 8) as u32).to_le_bytes());
        bytes.extend_from_slice(b"WAVE");

        bytes.extend_from_slice(b"fmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&sample_rate.to_le_bytes());
        let byte_rate = sample_rate * 2;
        bytes.extend_from_slice(&byte_rate.to_le_bytes());
        bytes.extend_from_slice(&2u16.to_le_bytes());
        bytes.extend_from_slice(&16u16.to_le_bytes());

        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&(data_len as u32).to_le_bytes());

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let sample = synth(t).clamp(-1.0, 1.0);
            let pcm_16 = (sample * 32767.0) as i16;
            bytes.extend_from_slice(&pcm_16.to_le_bytes());
        }

        load_sound_from_bytes(&bytes).await.ok()
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_backend::SoundBackend;

#[cfg(not(target_arch = "wasm32"))]
pub use native_backend::SoundBackend;

pub struct SoundManager {
    backend: SoundBackend,
}

impl SoundManager {
    pub async fn new() -> Self {
        Self {
            backend: SoundBackend::new().await,
        }
    }

    pub fn play(&self, trigger: SoundTrigger) {
        self.backend.play(trigger);
    }
}
