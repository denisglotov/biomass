use super::game::state::SoundTrigger;
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
        Self {
            snd_wall: load_sound_from_bytes(include_bytes!("../assets/wall.wav")).await.ok(),
            snd_tick: load_sound_from_bytes(include_bytes!("../assets/tick.wav")).await.ok(),
            snd_pop: load_sound_from_bytes(include_bytes!("../assets/pop.wav")).await.ok(),
            snd_win: load_sound_from_bytes(include_bytes!("../assets/win.wav")).await.ok(),
            snd_loss: load_sound_from_bytes(include_bytes!("../assets/loss.wav")).await.ok(),
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
