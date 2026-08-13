mod audio;
mod game;
mod ui;

use audio::SoundManager;
use game::state::GameState;
use macroquad::prelude::*;
use ui::hud::Hud;

fn window_conf() -> Conf {
    Conf {
        window_title: "Biomass - Sci-Fi Tactical Containment".to_string(),
        window_width: 1400,
        window_height: 1050,
        high_dpi: true,
        window_resizable: true,
        fullscreen: false,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let sound_manager = SoundManager::new().await;
    let mut state = GameState::new();
    let mut hud = Hud::new().await;

    loop {
        let dt = get_frame_time();

        // 1. Update Game State (animations, turn phases, isolation checks)
        let state_sound = state.update(dt);
        if let Some(snd) = state_sound {
            sound_manager.play(snd);
        }

        // 2. Draw HUD & Handle Input
        let hud_sound = hud.draw_and_handle_input(&mut state);
        if let Some(snd) = hud_sound {
            sound_manager.play(snd);
        }

        // 3. Render World (Board, Hexes, Biomass, Effects)
        next_frame().await;
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_plugin_exports {
    #[no_mangle]
    pub extern "C" fn biomass_audio_crate_version() -> u32 {
        1
    }

    #[no_mangle]
    pub extern "C" fn biomass_storage_crate_version() -> u32 {
        1
    }

    #[no_mangle]
    pub extern "C" fn macroquad_audio_crate_version() -> u32 {
        1
    }

    #[no_mangle]
    pub extern "C" fn sapp_jsutils_crate_version() -> u32 {
        1
    }

    #[no_mangle]
    pub extern "C" fn quad_net_crate_version() -> u32 {
        1
    }
}
