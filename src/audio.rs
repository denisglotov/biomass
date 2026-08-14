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
        fn play_sound_fanfare();
        fn play_sound_loss();
        fn play_sound_click();
        fn play_sound_error();
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
                    SoundTrigger::GrandFanfare => play_sound_fanfare(),
                    SoundTrigger::LossAlert => play_sound_loss(),
                    SoundTrigger::ButtonClick => play_sound_click(),
                    SoundTrigger::InvalidMove => play_sound_error(),
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
        snd_fanfare: Option<Sound>,
        snd_loss: Option<Sound>,
        snd_click: Option<Sound>,
        snd_error: Option<Sound>,
    }

    impl SoundBackend {
        pub async fn new() -> Self {
            Self {
                snd_wall: load_sound_from_bytes(include_bytes!("../assets/wall.wav"))
                    .await
                    .ok(),
                snd_tick: load_sound_from_bytes(include_bytes!("../assets/tick.wav"))
                    .await
                    .ok(),
                snd_pop: load_sound_from_bytes(include_bytes!("../assets/pop.wav"))
                    .await
                    .ok(),
                snd_win: load_sound_from_bytes(include_bytes!("../assets/win.wav"))
                    .await
                    .ok(),
                snd_fanfare: load_sound_from_bytes(include_bytes!("../assets/fanfare.wav"))
                    .await
                    .ok(),
                snd_loss: load_sound_from_bytes(include_bytes!("../assets/loss.wav"))
                    .await
                    .ok(),
                snd_click: load_sound_from_bytes(include_bytes!("../assets/click.wav"))
                    .await
                    .ok(),
                snd_error: load_sound_from_bytes(include_bytes!("../assets/error.wav"))
                    .await
                    .ok(),
            }
        }

        pub fn play(&self, trigger: SoundTrigger) {
            let sound = match trigger {
                SoundTrigger::WallPlace => &self.snd_wall,
                SoundTrigger::BiomassTick => &self.snd_tick,
                SoundTrigger::IsolationPop => &self.snd_pop,
                SoundTrigger::WinFanfare => &self.snd_win,
                SoundTrigger::GrandFanfare => {
                    if self.snd_fanfare.is_some() {
                        &self.snd_fanfare
                    } else {
                        &self.snd_win
                    }
                }
                SoundTrigger::LossAlert => &self.snd_loss,
                SoundTrigger::ButtonClick => &self.snd_click,
                SoundTrigger::InvalidMove => &self.snd_error,
            };

            if let Some(snd) = sound {
                play_sound_once(snd);
            }
        }
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
