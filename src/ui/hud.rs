use crate::game::grid::{CellType, Edge, EdgeState};
use crate::game::state::{GamePhase, GameState, SoundTrigger};
use macroquad::prelude::*;

use super::fx::{
    CloneFxParams, Confetti, Particle, Shockwave, SplashBead, WetDropletJump, WetSplash,
};

pub struct Hud {
    pub font: Option<Font>,
    pub hovered_edge: Option<Edge>,
    pub suppressed_hover_edge: Option<Edge>,
    pub particles: Vec<Particle>,
    pub confetti: Vec<Confetti>,
    pub shockwaves: Vec<Shockwave>,
    pub wet_jumps: Vec<WetDropletJump>,
    pub wet_splashes: Vec<WetSplash>,
    pub pan_offset: (f32, f32),
    pub drag_start: Option<(f32, f32)>,
    pub is_dragging: bool,
    pub last_level_idx: usize,
    pub render_target: Option<RenderTarget>,
}

impl Hud {
    pub async fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        let font = load_ttf_font("assets/fonts/Symbola-Subset.ttf").await.ok();

        #[cfg(not(target_arch = "wasm32"))]
        let font = {
            let font_bytes = include_bytes!("../../assets/fonts/Symbola-Subset.ttf");
            load_ttf_font_from_bytes(font_bytes).ok()
        };

        Self {
            font,
            hovered_edge: None,
            suppressed_hover_edge: None,
            particles: Vec::new(),
            confetti: Vec::new(),
            shockwaves: Vec::new(),
            wet_jumps: Vec::new(),
            wet_splashes: Vec::new(),
            pan_offset: (0.0, 0.0),
            drag_start: None,
            is_dragging: false,
            last_level_idx: 0,
            render_target: None,
        }
    }

    pub fn draw_text_str(&self, text: &str, x: f32, y: f32, font_size: f32, color: Color) {
        if let Some(ref font) = self.font {
            draw_text_ex(
                text,
                x,
                y,
                TextParams {
                    font: Some(font),
                    font_size: font_size as u16,
                    color,
                    ..Default::default()
                },
            );
        } else {
            draw_text(text, x, y, font_size, color);
        }
    }

    pub fn measure_text_str(&self, text: &str, font_size: f32) -> TextDimensions {
        measure_text(text, self.font.as_ref(), font_size as u16, 1.0)
    }

    pub fn spawn_burst(&mut self, x: f32, y: f32, color: Color, count: usize) {
        for _ in 0..count {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(40.0, 140.0);
            let life = rand::gen_range(0.4, 0.8);
            self.particles.push(Particle {
                x,
                y,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                radius: rand::gen_range(3.0, 6.0),
                color,
                life,
                max_life: life,
            });
        }
    }

    pub fn spawn_shockwave(&mut self, cx: f32, cy: f32, color: Color, max_radius: f32) {
        self.shockwaves.push(Shockwave {
            cx,
            cy,
            radius: 5.0,
            max_radius,
            color,
            alpha: 1.0,
        });
    }

    pub fn spawn_clone_fx(&mut self, params: CloneFxParams) {
        let seed = rand::gen_range(0.0, 100.0);
        self.wet_jumps.push(WetDropletJump {
            from_r: params.from_r,
            from_c: params.from_c,
            to_r: params.to_r,
            to_c: params.to_c,
            from_x: params.from_x,
            from_y: params.from_y,
            to_x: params.to_x,
            to_y: params.to_y,
            life: 0.45,
            max_life: 0.45,
            cell_size: params.cell_size,
            seed,
            has_splashed: false,
        });

        // Parent cell mitosis ripple recoil
        self.spawn_shockwave(
            params.from_x,
            params.from_y,
            Color::from_rgba(0, 230, 118, 180),
            params.cell_size * 0.75,
        );
    }

    pub fn spawn_wet_splash(&mut self, cx: f32, cy: f32, cell_size: f32) {
        let mut beads = Vec::new();
        let colors = [
            Color::from_rgba(0, 230, 118, 255),
            Color::from_rgba(105, 240, 174, 255),
            Color::from_rgba(174, 234, 0, 255),
            Color::from_rgba(209, 250, 229, 255),
            Color::from_rgba(255, 255, 255, 255),
        ];

        for _ in 0..16 {
            let angle = rand::gen_range(0.0, std::f32::consts::TAU);
            let speed = rand::gen_range(35.0, 135.0);
            let vx = angle.cos() * speed;
            let vy = angle.sin() * speed - rand::gen_range(20.0, 50.0);
            let life = rand::gen_range(0.24, 0.40);
            let color = colors[rand::gen_range(0, colors.len())];

            beads.push(SplashBead {
                x: cx + angle.cos() * rand::gen_range(1.0, 6.0),
                y: cy + angle.sin() * rand::gen_range(1.0, 6.0),
                vx,
                vy,
                radius: rand::gen_range(2.0, 4.2),
                color,
                life,
                max_life: life,
            });
        }

        self.wet_splashes.push(WetSplash {
            cx,
            cy,
            cell_size,
            life: 0.40,
            max_life: 0.40,
            beads,
        });

        // Wet impact shockwaves
        self.spawn_shockwave(cx, cy, Color::from_rgba(0, 230, 118, 220), cell_size * 1.2);
        self.spawn_shockwave(
            cx,
            cy,
            Color::from_rgba(105, 240, 174, 240),
            cell_size * 0.8,
        );
        self.spawn_shockwave(
            cx,
            cy,
            Color::from_rgba(255, 255, 255, 230),
            cell_size * 0.45,
        );
    }

    pub fn spawn_confetti_burst(&mut self, cx: f32, cy: f32, count: usize, scale: f32) {
        let colors = [
            Color::from_rgba(255, 215, 0, 255),   // Radiant Gold
            Color::from_rgba(0, 230, 118, 255),   // Emerald Green
            Color::from_rgba(0, 229, 255, 255),   // Neon Cyan
            Color::from_rgba(255, 64, 129, 255),  // Hot Pink
            Color::from_rgba(255, 145, 0, 255),   // Electric Orange
            Color::from_rgba(213, 0, 249, 255),   // Bright Violet
            Color::from_rgba(255, 255, 255, 255), // Pure White
        ];
        for _ in 0..count {
            let angle = rand::gen_range(-std::f32::consts::PI * 0.95, -std::f32::consts::PI * 0.05);
            let speed = rand::gen_range(160.0, 480.0) * scale;
            let life = rand::gen_range(2.8, 5.2);
            let color = colors[rand::gen_range(0, colors.len())];
            self.confetti.push(Confetti {
                x: cx,
                y: cy,
                vx: angle.cos() * speed,
                vy: angle.sin() * speed,
                rotation: rand::gen_range(0.0, std::f32::consts::TAU),
                rotation_speed: rand::gen_range(-8.0, 8.0),
                width: rand::gen_range(9.0, 16.0) * scale,
                height: rand::gen_range(5.0, 10.0) * scale,
                color,
                life,
                max_life: life,
                sway_phase: rand::gen_range(0.0, std::f32::consts::TAU),
            });
        }
    }

    pub fn spawn_confetti_rain(&mut self, screen_w: f32, scale: f32, count: usize) {
        let colors = [
            Color::from_rgba(255, 215, 0, 255),   // Gold
            Color::from_rgba(0, 230, 118, 255),   // Emerald
            Color::from_rgba(0, 229, 255, 255),   // Cyan
            Color::from_rgba(255, 64, 129, 255),  // Pink
            Color::from_rgba(255, 145, 0, 255),   // Orange
            Color::from_rgba(213, 0, 249, 255),   // Violet
            Color::from_rgba(255, 255, 255, 255), // White
        ];
        for _ in 0..count {
            let x = rand::gen_range(0.0, screen_w);
            let y = rand::gen_range(-50.0, -10.0);
            let speed_y = rand::gen_range(80.0, 220.0) * scale;
            let speed_x = rand::gen_range(-40.0, 40.0) * scale;
            let life = rand::gen_range(3.5, 6.0);
            let color = colors[rand::gen_range(0, colors.len())];
            self.confetti.push(Confetti {
                x,
                y,
                vx: speed_x,
                vy: speed_y,
                rotation: rand::gen_range(0.0, std::f32::consts::TAU),
                rotation_speed: rand::gen_range(-6.0, 6.0),
                width: rand::gen_range(8.0, 15.0) * scale,
                height: rand::gen_range(4.0, 9.0) * scale,
                color,
                life,
                max_life: life,
                sway_phase: rand::gen_range(0.0, std::f32::consts::TAU),
            });
        }
    }

    pub fn draw_confetti(&self) {
        for c in &self.confetti {
            let alpha = (c.life / c.max_life).min(1.0).clamp(0.0, 1.0);
            let color = Color::new(c.color.r, c.color.g, c.color.b, alpha);

            let cos_r = c.rotation.cos();
            let sin_r = c.rotation.sin();
            let flutter = (c.rotation * 1.6).cos().abs().max(0.18);
            let hw = c.width * 0.5 * flutter;
            let hh = c.height * 0.5;

            let p1 = vec2(
                c.x + (-hw * cos_r - -hh * sin_r),
                c.y + (-hw * sin_r + -hh * cos_r),
            );
            let p2 = vec2(
                c.x + (hw * cos_r - -hh * sin_r),
                c.y + (hw * sin_r + -hh * cos_r),
            );
            let p3 = vec2(
                c.x + (hw * cos_r - hh * sin_r),
                c.y + (hw * sin_r + hh * cos_r),
            );
            let p4 = vec2(
                c.x + (-hw * cos_r - hh * sin_r),
                c.y + (-hw * sin_r + hh * cos_r),
            );

            draw_triangle(p1, p2, p3, color);
            draw_triangle(p1, p3, p4, color);
        }
    }

    pub fn update_fx(&mut self, dt: f32) {
        for p in self.particles.iter_mut() {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);

        let time = get_time() as f32;
        for c in self.confetti.iter_mut() {
            c.vy += 85.0 * dt;
            c.vx += (time * 3.0 + c.sway_phase).sin() * 25.0 * dt;
            c.vx *= 0.99;
            c.x += c.vx * dt;
            c.y += c.vy * dt;
            c.rotation += c.rotation_speed * dt;
            c.life -= dt;
        }
        self.confetti.retain(|c| c.life > 0.0);

        for sw in self.shockwaves.iter_mut() {
            sw.radius += (sw.max_radius - sw.radius) * 7.0 * dt;
            sw.alpha -= dt * 1.8;
        }
        self.shockwaves.retain(|sw| sw.alpha > 0.0);

        for jump in self.wet_jumps.iter_mut() {
            jump.life -= dt;
        }

        let mut new_splashes = Vec::new();
        for jump in self.wet_jumps.iter_mut() {
            let progress = (1.0 - (jump.life / jump.max_life)).clamp(0.0, 1.0);
            if progress >= 0.65 && !jump.has_splashed {
                jump.has_splashed = true;
                new_splashes.push((jump.to_x, jump.to_y, jump.cell_size));
            }
        }
        for (tx, ty, cs) in new_splashes {
            self.spawn_wet_splash(tx, ty, cs);
        }
        self.wet_jumps.retain(|j| j.life > 0.0);

        for splash in self.wet_splashes.iter_mut() {
            splash.life -= dt;
            for bead in splash.beads.iter_mut() {
                bead.x += bead.vx * dt;
                bead.y += bead.vy * dt;
                bead.vy += 160.0 * dt;
                bead.vx *= 0.94;
                bead.vy *= 0.94;
                bead.life -= dt;
            }
            splash.beads.retain(|b| b.life > 0.0);
        }
        self.wet_splashes.retain(|s| s.life > 0.0);
    }

    pub fn draw_and_handle_input(&mut self, state: &mut GameState) -> Option<SoundTrigger> {
        let dt = get_frame_time();
        self.update_fx(dt);

        let mut sound_trigger = None;

        let screen_w = screen_width();
        let screen_h = screen_height();
        let is_portrait = screen_h > screen_w;

        // Dynamic, responsive UI scaling factor:
        // Handles Android high-DPI (e.g. 1080x2400 portrait or 2400x1080 landscape),
        // WebAssembly, and Desktop targets smoothly without tiny elements on mobile.
        let scale = if is_portrait {
            (screen_w / 500.0).clamp(1.0, 4.0)
        } else {
            (screen_h / 650.0).clamp(1.0, 4.0)
        };

        // Scaled layout constants
        let header_h = if is_portrait {
            52.0 * scale
        } else {
            42.0 * scale
        };
        let stats_h = if is_portrait {
            52.0 * scale
        } else {
            42.0 * scale
        };
        let banner_h = if is_portrait {
            48.0 * scale
        } else {
            42.0 * scale
        };
        let grid_bottom_margin = 20.0 * scale;

        // 1. High-Contrast Deep Slate Ambient Background (#0f172a)
        clear_background(Color::from_rgba(15, 23, 42, 255));

        // 2. Draw Header
        let header_sound = self.draw_header(screen_w, header_h, scale);
        if header_sound.is_some() {
            sound_trigger = header_sound;
        }

        // 3. Draw Stats Bar
        let stats_y = header_h;
        self.draw_stats_bar(state, screen_w, stats_y, stats_h, scale);

        // 4. Draw Level Description Banner
        let level_banner_y = stats_y + stats_h;
        self.draw_level_banner(state, screen_w, level_banner_y, scale);

        // 5. Draw Grid & Interactive Workspace (fill all remaining screen space)
        #[cfg(not(target_arch = "wasm32"))]
        let grid_top = level_banner_y + banner_h + 6.0 * scale;
        #[cfg(target_arch = "wasm32")]
        let grid_top = level_banner_y + banner_h;
        let side_margin = 12.0 * scale;
        let viewport_w = (screen_w - side_margin * 2.0).max(200.0);
        let viewport_h = (screen_h - grid_top - grid_bottom_margin).max(200.0);
        let viewport_x = (screen_w - viewport_w) / 2.0;
        let viewport_y = grid_top;

        let grid_sound =
            self.draw_grid(state, viewport_x, viewport_y, viewport_w, viewport_h, scale);
        if grid_sound.is_some() {
            sound_trigger = grid_sound;
        }

        // 7. Draw Win / Loss Modal Overlay
        if state.phase == GamePhase::Victory || state.phase == GamePhase::Defeat {
            let modal_sound = self.draw_modal(state, screen_w, screen_h, scale);
            if modal_sound.is_some() {
                sound_trigger = modal_sound;
            }
        }

        sound_trigger
    }

    fn draw_header(&self, screen_w: f32, header_h: f32, scale: f32) -> Option<SoundTrigger> {
        let sound = None;
        // Dark Obsidian Header Bar
        draw_rectangle(
            0.0,
            0.0,
            screen_w,
            header_h,
            Color::from_rgba(15, 23, 42, 255),
        );
        draw_line(
            0.0,
            header_h,
            screen_w,
            header_h,
            3.0 * scale,
            Color::from_rgba(0, 229, 255, 255),
        );

        let title = "☣ BIOMASS";
        let font_size = 26.0 * scale;
        let title_dim = self.measure_text_str(title, font_size);

        let title_x = 16.0 * scale;
        self.draw_text_str(
            title,
            title_x,
            header_h * 0.68,
            font_size,
            Color::from_rgba(0, 230, 118, 255),
        );

        let subtitle = "TACTICAL BIOLOGICAL CONTAINMENT PROTOCOL";
        let subtitle_x = title_x + title_dim.width + 16.0 * scale;
        let subtitle_font_size = 12.0 * scale;
        let sub_dim = self.measure_text_str(subtitle, subtitle_font_size);

        // Render subtitle if horizontal space permits
        if subtitle_x + sub_dim.width < screen_w - 10.0 * scale {
            self.draw_text_str(
                subtitle,
                subtitle_x,
                header_h * 0.64,
                subtitle_font_size,
                Color::from_rgba(226, 232, 240, 255),
            );
        }

        sound
    }

    fn draw_stats_bar(&self, state: &GameState, screen_w: f32, y: f32, h: f32, scale: f32) {
        // Deep Slate Container (#1e293b)
        draw_rectangle(0.0, y, screen_w, h, Color::from_rgba(30, 41, 59, 255));
        draw_line(
            0.0,
            y + h,
            screen_w,
            y + h,
            2.0 * scale,
            Color::from_rgba(51, 65, 85, 255),
        );

        let item_w = screen_w / 4.0;

        // Turn
        self.draw_stat_item(
            "⌛ TURN",
            &state.turn_number.to_string(),
            0.0,
            y,
            item_w,
            h,
            WHITE,
            scale,
        );

        // Walls Remaining
        let walls_text = format!("{} / {}", state.walls_left, state.level.walls_per_turn);
        let walls_color = if state.walls_left > 0 {
            Color::from_rgba(0, 229, 255, 255)
        } else {
            Color::from_rgba(255, 171, 0, 255)
        };
        self.draw_stat_item(
            "🛡 WALLS",
            &walls_text,
            item_w,
            y,
            item_w,
            h,
            walls_color,
            scale,
        );

        // Active Biomass
        let bio_count = state.grid.count_biomass();
        let bio_text = bio_count.to_string();
        let bio_color = Color::from_rgba(0, 230, 118, 255);
        self.draw_stat_item(
            "☣ BIOMASS",
            &bio_text,
            item_w * 2.0,
            y,
            item_w,
            h,
            bio_color,
            scale,
        );

        // Max Capacity
        let max_text = state.level.max_threshold.to_string();
        self.draw_stat_item(
            "⚠ MAX",
            &max_text,
            item_w * 3.0,
            y,
            item_w,
            h,
            Color::from_rgba(255, 82, 82, 255),
            scale,
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_stat_item(
        &self,
        label: &str,
        value: &str,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        val_color: Color,
        scale: f32,
    ) {
        let label_size = 12.0 * scale;
        let val_size = 20.0 * scale;

        let label_dim = self.measure_text_str(label, label_size);
        let val_dim = self.measure_text_str(value, val_size);

        let label_x = x + (w - label_dim.width) / 2.0;
        let val_x = x + (w - val_dim.width) / 2.0;

        self.draw_text_str(
            label,
            label_x.max(x + 2.0),
            y + h * 0.38,
            label_size,
            Color::from_rgba(148, 163, 184, 255),
        );
        self.draw_text_str(value, val_x.max(x + 2.0), y + h * 0.82, val_size, val_color);
    }

    fn draw_level_banner(&self, state: &GameState, screen_w: f32, y: f32, scale: f32) {
        let text = format!("☣ {} — {}", state.level.title, state.level.description);
        let mut font_size = 22.0 * scale;
        let mut dimensions = self.measure_text_str(&text, font_size);

        // Dynamically shrink font size if text is too wide for screen
        let max_w = screen_w - 30.0 * scale;
        if dimensions.width > max_w {
            let ratio = (max_w / dimensions.width).clamp(0.65, 1.0);
            font_size *= ratio;
            dimensions = self.measure_text_str(&text, font_size);
        }

        let box_h = 42.0 * scale;
        let box_w = (dimensions.width + 32.0 * scale).min(screen_w - 20.0 * scale);
        let box_x = (screen_w - box_w) / 2.0;

        let draw_x = (screen_w - dimensions.width) / 2.0;
        self.draw_text_str(
            &text,
            draw_x.max(box_x + 8.0),
            y + box_h * 0.68,
            font_size,
            Color::from_rgba(255, 255, 255, 255),
        );
    }

    fn draw_button(&self, label: &str, rect: Rect, mouse_pos: (f32, f32), font_size: f32) {
        let hovered = rect.contains(mouse_pos.into());
        let bg_color = if hovered {
            Color::from_rgba(2, 132, 199, 255) // Vibrant Sky Blue
        } else {
            Color::from_rgba(30, 41, 59, 255) // Dark Slate
        };
        let border_color = if hovered {
            Color::from_rgba(0, 230, 118, 255) // Neon Green
        } else {
            Color::from_rgba(0, 229, 255, 255) // Bright Cyan
        };

        draw_rectangle(rect.x, rect.y, rect.w, rect.h, bg_color);
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, border_color);

        let mut current_font_size = font_size;
        let mut dim = self.measure_text_str(label, current_font_size);
        if dim.width > rect.w - 4.0 {
            current_font_size *= (rect.w - 4.0) / dim.width;
            dim = self.measure_text_str(label, current_font_size);
        }

        let text_x = rect.x + (rect.w - dim.width) / 2.0;
        let text_y = rect.y + (rect.h + dim.height) / 2.0 - 2.0;
        self.draw_text_str(label, text_x, text_y, current_font_size, WHITE);
    }

    fn draw_grid(
        &mut self,
        state: &mut GameState,
        viewport_x: f32,
        viewport_y: f32,
        viewport_w: f32,
        viewport_h: f32,
        scale: f32,
    ) -> Option<SoundTrigger> {
        let mut sound_trigger = None;

        let rows = state.grid.rows;
        let cols = state.grid.cols;

        // Reset pan offset if level index changed
        if state.current_level_idx != self.last_level_idx {
            self.last_level_idx = state.current_level_idx;
            self.pan_offset = (0.0, 0.0);
            self.drag_start = None;
            self.is_dragging = false;
            self.confetti.clear();
        }

        // Cell size scaling rule:
        // - On Android: Grid cell scaling caps at 6x6 so higher levels maintain large touchable tiles and scroll via touch swipe.
        // - On Web & Native Desktop: Grid cells scale to fit all levels (up to 10x10) inside the spacious window,
        //   clamping to min_cell_size if the user resizes to a very small window.
        #[cfg(target_os = "android")]
        let (max_cols, max_rows) = ((cols as f32).min(6.0), (rows as f32).min(6.0));

        #[cfg(not(target_os = "android"))]
        let (max_cols, max_rows) = (cols as f32, rows as f32);

        let min_cell_size = 48.0 * scale;
        let cell_size_w = (viewport_w - 12.0 * scale) / max_cols;
        let cell_size_h = (viewport_h - 12.0 * scale) / max_rows;
        let cell_size = cell_size_w.min(cell_size_h).max(min_cell_size);

        let grid_total_w = cols as f32 * cell_size;
        let grid_total_h = rows as f32 * cell_size;

        let mouse_pos = mouse_position();
        let mouse_down = is_mouse_button_down(MouseButton::Left);
        let mouse_pressed = is_mouse_button_pressed(MouseButton::Left);
        let mouse_released = is_mouse_button_released(MouseButton::Left);
        let (wheel_x, wheel_y) = mouse_wheel();

        // 1. Mouse wheel / Trackpad scrolling
        if wheel_y != 0.0 || wheel_x != 0.0 {
            let scroll_speed = 30.0 * scale;
            if wheel_y != 0.0 {
                self.pan_offset.1 += wheel_y * scroll_speed;
            }
            if wheel_x != 0.0 {
                self.pan_offset.0 -= wheel_x * scroll_speed;
            }
        }

        // 2. Drag / Swipe Panning Logic
        let in_viewport = mouse_pos.0 >= viewport_x
            && mouse_pos.0 <= viewport_x + viewport_w
            && mouse_pos.1 >= viewport_y
            && mouse_pos.1 <= viewport_y + viewport_h;

        if mouse_pressed && in_viewport {
            self.drag_start = Some(mouse_pos);
            self.is_dragging = false;
        }

        if mouse_down {
            if let Some(start) = self.drag_start {
                let dx = mouse_pos.0 - start.0;
                let dy = mouse_pos.1 - start.1;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq > (6.0 * scale) * (6.0 * scale) {
                    self.is_dragging = true;
                }

                if self.is_dragging {
                    self.pan_offset.0 += dx;
                    self.pan_offset.1 += dy;
                    self.drag_start = Some(mouse_pos);
                }
            }
        }

        let was_dragging = self.is_dragging;
        if mouse_released {
            self.drag_start = None;
            self.is_dragging = false;
        }

        let pad = 6.0 * scale;
        let box_w = grid_total_w + pad * 2.0;
        let box_h = grid_total_h + pad * 2.0;

        // 3. Pan clamping (including boundary padding so left/right/top/bottom borders are 100% scrollable and visible)
        if box_w <= viewport_w {
            self.pan_offset.0 = (viewport_w - grid_total_w) / 2.0;
        } else {
            let min_pan_x = viewport_w - grid_total_w - pad;
            let max_pan_x = pad;
            self.pan_offset.0 = self.pan_offset.0.clamp(min_pan_x, max_pan_x);
        }

        if box_h <= viewport_h {
            self.pan_offset.1 = (viewport_h - grid_total_h) / 2.0;
        } else {
            let min_pan_y = viewport_h - grid_total_h - pad;
            let max_pan_y = pad;
            self.pan_offset.1 = self.pan_offset.1.clamp(min_pan_y, max_pan_y);
        }

        // Recreate render target texture if viewport dimensions change
        let rt_w = (viewport_w as u32).max(1);
        let rt_h = (viewport_h as u32).max(1);
        if self.render_target.as_ref().is_none_or(|rt| {
            rt.texture.width() as u32 != rt_w || rt.texture.height() as u32 != rt_h
        }) {
            self.render_target = Some(render_target(rt_w, rt_h));
        }

        let rt = self.render_target.as_ref().unwrap();
        let render_texture = rt.texture.clone();

        // Set camera to render target texture for pixel-perfect smooth sub-pixel scrolling
        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, viewport_w, viewport_h));
        camera.render_target = Some(rt.clone());
        set_camera(&camera);

        clear_background(Color::from_rgba(15, 23, 42, 255));

        let gx = self.pan_offset.0;
        let gy = self.pan_offset.1;

        let t = get_time() as f32;

        // 4. Draw Solid Scrollable Reactor Core Boundary (attached to grid coordinates gx, gy)
        let box_x = gx - pad;
        let box_y = gy - pad;

        // Solid background plate behind grid
        draw_rectangle(
            box_x,
            box_y,
            box_w,
            box_h,
            Color::from_rgba(30, 41, 59, 255),
        );

        // Solid thick cyan outer frame
        draw_rectangle_lines(
            box_x,
            box_y,
            box_w,
            box_h,
            5.0 * scale,
            Color::from_rgba(0, 229, 255, 255),
        );

        // Subtle inner accent line
        draw_rectangle_lines(
            box_x + 2.0 * scale,
            box_y + 2.0 * scale,
            box_w - 4.0 * scale,
            box_h - 4.0 * scale,
            1.5 * scale,
            Color::from_rgba(2, 132, 199, 180),
        );

        // 5. Render Checkered Grid Cells
        for r in 0..rows {
            for c in 0..cols {
                let cx = gx + c as f32 * cell_size;
                let cy = gy + r as f32 * cell_size;

                // Coarse culling for performance on huge grids
                if cx + cell_size < -50.0
                    || cx > viewport_w + 50.0
                    || cy + cell_size < -50.0
                    || cy > viewport_h + 50.0
                {
                    continue;
                }

                let cell_type = state.grid.get_cell(r, c);

                match cell_type {
                    CellType::Empty => {
                        let tile_color = if (r + c) % 2 == 0 {
                            Color::from_rgba(248, 250, 252, 255)
                        } else {
                            Color::from_rgba(226, 232, 240, 255)
                        };

                        draw_rectangle(
                            cx + 1.0,
                            cy + 1.0,
                            cell_size - 2.0,
                            cell_size - 2.0,
                            tile_color,
                        );

                        draw_rectangle_lines(
                            cx + 1.0,
                            cy + 1.0,
                            cell_size - 2.0,
                            cell_size - 2.0,
                            1.0,
                            Color::from_rgba(2, 132, 199, 120),
                        );
                    }
                    CellType::Biomass => {
                        let active_target_jump =
                            self.wet_jumps.iter().find(|j| j.to_r == r && j.to_c == c);
                        let active_parent_jump = self
                            .wet_jumps
                            .iter()
                            .find(|j| j.from_r == r && j.from_c == c);

                        let (is_in_flight, bloom_s) = if let Some(jump) = active_target_jump {
                            let prog = (1.0 - (jump.life / jump.max_life)).clamp(0.0, 1.0);
                            if prog < 0.65 {
                                (true, 0.0)
                            } else {
                                (false, ((prog - 0.65) / 0.35).clamp(0.0, 1.0))
                            }
                        } else {
                            (false, 1.0)
                        };

                        if is_in_flight {
                            // Target cell is not yet occupied by biomass (droplet is still airborne)
                            let tile_color = if (r + c) % 2 == 0 {
                                Color::from_rgba(248, 250, 252, 255)
                            } else {
                                Color::from_rgba(226, 232, 240, 255)
                            };

                            draw_rectangle(
                                cx + 1.0,
                                cy + 1.0,
                                cell_size - 2.0,
                                cell_size - 2.0,
                                tile_color,
                            );

                            draw_rectangle_lines(
                                cx + 1.0,
                                cy + 1.0,
                                cell_size - 2.0,
                                cell_size - 2.0,
                                1.0,
                                Color::from_rgba(2, 132, 199, 120),
                            );
                        } else {
                            if bloom_s < 1.0 {
                                // Draw base empty checkered tile underneath
                                let tile_color = if (r + c) % 2 == 0 {
                                    Color::from_rgba(248, 250, 252, 255)
                                } else {
                                    Color::from_rgba(226, 232, 240, 255)
                                };
                                draw_rectangle(
                                    cx + 1.0,
                                    cy + 1.0,
                                    cell_size - 2.0,
                                    cell_size - 2.0,
                                    tile_color,
                                );
                            }

                            // Biomass background tile (fades in smoothly with landing)
                            let bg_alpha = (bloom_s * 1.8).min(1.0);
                            draw_rectangle(
                                cx + 1.0,
                                cy + 1.0,
                                cell_size - 2.0,
                                cell_size - 2.0,
                                Color::new(0.82, 0.98, 0.90, bg_alpha),
                            );
                            draw_rectangle_lines(
                                cx + 1.0,
                                cy + 1.0,
                                cell_size - 2.0,
                                cell_size - 2.0,
                                1.5,
                                Color::new(0.0, 0.9, 0.46, bg_alpha * 0.78),
                            );

                            // Elastic bloom scale for newly landing cell
                            let growth_scale = if bloom_s < 1.0 {
                                (1.0 - (-5.0 * bloom_s).exp()
                                    * (std::f32::consts::PI * 2.5 * bloom_s).cos())
                                .max(0.0)
                            } else {
                                1.0
                            };

                            // Parent cell recoil pulse
                            let parent_pulse = if let Some(p_jump) = active_parent_jump {
                                let p_prog =
                                    (1.0 - (p_jump.life / p_jump.max_life)).clamp(0.0, 1.0);
                                if p_prog < 0.35 {
                                    ((1.0 - p_prog / 0.35) * std::f32::consts::PI).sin()
                                        * (-3.0 * scale)
                                } else {
                                    0.0
                                }
                            } else {
                                0.0
                            };

                            let center_x = cx + cell_size / 2.0;
                            let center_y = cy + cell_size / 2.0;
                            let pulse =
                                (t * 4.0 + (r + c) as f32).sin() * (2.5 * scale) + parent_pulse;
                            let base_r = (cell_size * 0.28 + pulse) * growth_scale;
                            let core_alpha = (bloom_s * 2.0).min(1.0);

                            draw_circle(
                                center_x,
                                center_y,
                                base_r * 1.5,
                                Color::new(0.0, 0.9, 0.46, core_alpha * 0.2),
                            );
                            draw_circle(
                                center_x,
                                center_y,
                                base_r * 1.2,
                                Color::new(0.0, 0.9, 0.46, core_alpha * 0.4),
                            );
                            draw_circle(
                                center_x,
                                center_y,
                                base_r,
                                Color::new(0.0, 0.9, 0.46, core_alpha),
                            );

                            for i in 0..4 {
                                let angle = t * 2.8
                                    + (i as f32 * std::f32::consts::TAU / 4.0)
                                    + (1.0 - bloom_s) * 4.0;
                                let orbit_radius = base_r * 0.55;
                                let ox = center_x + angle.cos() * orbit_radius;
                                let oy = center_y + angle.sin() * orbit_radius;
                                draw_circle(
                                    ox,
                                    oy,
                                    base_r * 0.28,
                                    Color::new(0.41, 0.94, 0.68, core_alpha),
                                );
                            }

                            draw_circle(
                                center_x - base_r * 0.25,
                                center_y - base_r * 0.25,
                                base_r * 0.28,
                                Color::new(1.0, 1.0, 1.0, core_alpha),
                            );
                        }
                    }
                    CellType::Obstacle => {
                        draw_rectangle(
                            cx + 1.0,
                            cy + 1.0,
                            cell_size - 2.0,
                            cell_size - 2.0,
                            Color::from_rgba(100, 116, 139, 255),
                        );
                        draw_rectangle_lines(
                            cx + 2.0,
                            cy + 2.0,
                            cell_size - 4.0,
                            cell_size - 4.0,
                            2.0,
                            Color::from_rgba(51, 65, 85, 255),
                        );
                        draw_line(
                            cx + 4.0,
                            cy + 4.0,
                            cx + cell_size - 4.0,
                            cy + cell_size - 4.0,
                            2.0,
                            Color::from_rgba(15, 23, 42, 255),
                        );
                    }
                }
            }
        }

        // 6. Mouse Hover & Edge Detection (relative to grid inside viewport)
        self.hovered_edge = None;
        let rel_x = mouse_pos.0 - viewport_x - self.pan_offset.0;
        let rel_y = mouse_pos.1 - viewport_y - self.pan_offset.1;

        if state.phase == GamePhase::PlayerTurn
            && !was_dragging
            && in_viewport
            && rel_x >= 0.0
            && rel_x <= grid_total_w
            && rel_y >= 0.0
            && rel_y <= grid_total_h
        {
            let c = (rel_x / cell_size).floor() as usize;
            let r = (rel_y / cell_size).floor() as usize;

            if c < cols && r < rows {
                let dist_top = rel_y - (r as f32 * cell_size);
                let dist_bottom = ((r + 1) as f32 * cell_size) - rel_y;
                let dist_left = rel_x - (c as f32 * cell_size);
                let dist_right = ((c + 1) as f32 * cell_size) - rel_x;

                let min_dist = dist_top.min(dist_bottom).min(dist_left).min(dist_right);

                let edge = if min_dist == dist_top {
                    Edge::Horizontal { r, c }
                } else if min_dist == dist_bottom {
                    Edge::Horizontal { r: r + 1, c }
                } else if min_dist == dist_left {
                    Edge::Vertical { r, c }
                } else {
                    Edge::Vertical { r, c: c + 1 }
                };

                if state.placed_walls_this_turn.contains(&edge)
                    || (state.walls_left > 0 && state.grid.can_place_wall(edge))
                {
                    if self.suppressed_hover_edge == Some(edge) {
                        // Suppress hover highlight right after wall removal until cursor moves
                    } else {
                        self.hovered_edge = Some(edge);
                        if self.suppressed_hover_edge.is_some()
                            && self.suppressed_hover_edge != Some(edge)
                        {
                            self.suppressed_hover_edge = None;
                        }
                    }
                }
            }
        }

        // 7. Draw Hovered Edge Highlight & Click Handler
        if let Some(edge) = self.hovered_edge {
            let is_in_construction = state.placed_walls_this_turn.contains(&edge);
            let highlight_color = if is_in_construction {
                Color::from_rgba(255, 82, 82, 255) // Red warning highlight for removal
            } else {
                Color::from_rgba(0, 229, 255, 255) // Cyan highlight for placement
            };

            self.draw_edge_highlight(edge, gx, gy, cell_size, cell_size, highlight_color, scale);

            if mouse_released && !was_dragging {
                if is_in_construction {
                    if state.remove_placed_wall(edge) {
                        self.hovered_edge = None;
                        self.suppressed_hover_edge = Some(edge);
                        sound_trigger = Some(SoundTrigger::WallPlace);

                        let (wx, wy) = match edge {
                            Edge::Horizontal { r, c } => {
                                (gx + (c as f32 + 0.5) * cell_size, gy + r as f32 * cell_size)
                            }
                            Edge::Vertical { r, c } => {
                                (gx + c as f32 * cell_size, gy + (r as f32 + 0.5) * cell_size)
                            }
                        };
                        self.spawn_shockwave(
                            wx,
                            wy,
                            Color::from_rgba(255, 82, 82, 255),
                            cell_size * 1.1,
                        );
                        self.spawn_burst(wx, wy, Color::from_rgba(255, 171, 0, 255), 20);
                    }
                } else if state.walls_left > 0 && state.try_place_wall(edge) {
                    sound_trigger = Some(SoundTrigger::WallPlace);

                    let (wx, wy) = match edge {
                        Edge::Horizontal { r, c } => {
                            (gx + (c as f32 + 0.5) * cell_size, gy + r as f32 * cell_size)
                        }
                        Edge::Vertical { r, c } => {
                            (gx + c as f32 * cell_size, gy + (r as f32 + 0.5) * cell_size)
                        }
                    };
                    self.spawn_shockwave(
                        wx,
                        wy,
                        Color::from_rgba(0, 229, 255, 255),
                        cell_size * 0.9,
                    );
                    self.spawn_burst(wx, wy, Color::from_rgba(0, 229, 255, 255), 16);
                } else {
                    sound_trigger = Some(SoundTrigger::InvalidMove);
                }
            }
        }

        // 8. Render Barricade Walls (Permanent Cyan & In-Construction Hazard Amber)
        for r in 0..=rows {
            for c in 0..cols {
                let edge = Edge::Horizontal { r, c };
                if state.grid.get_edge(edge) == EdgeState::Wall {
                    let wx = gx + c as f32 * cell_size;
                    let wy = gy + r as f32 * cell_size;
                    let is_in_construction = state.placed_walls_this_turn.contains(&edge);

                    if is_in_construction {
                        let shine_phase = (t * 7.5).sin() * 0.5 + 0.5;
                        let main_alpha = (60.0 + 195.0 * shine_phase) as u8;
                        let aura_alpha = (10.0 + 200.0 * shine_phase) as u8;
                        let aura_w = (8.0 + 8.0 * shine_phase) * scale;

                        // Outer shining hazard aura
                        draw_line(
                            wx,
                            wy,
                            wx + cell_size,
                            wy,
                            aura_w,
                            Color::from_rgba(255, 180, 0, aura_alpha),
                        );
                        // Main hazard yellow line (shines on and off)
                        draw_line(
                            wx,
                            wy,
                            wx + cell_size,
                            wy,
                            6.0 * scale,
                            Color::from_rgba(
                                255,
                                (170.0 + 70.0 * shine_phase) as u8,
                                0,
                                main_alpha,
                            ),
                        );
                        // Neon yellow-white core
                        draw_line(
                            wx,
                            wy,
                            wx + cell_size,
                            wy,
                            2.0 * scale,
                            Color::from_rgba(
                                255,
                                255,
                                (160.0 + 95.0 * shine_phase) as u8,
                                main_alpha,
                            ),
                        );

                        let node_r = (4.0 + 2.0 * shine_phase) * scale;
                        draw_circle(wx, wy, node_r, Color::from_rgba(255, 191, 0, main_alpha));
                        draw_circle(
                            wx,
                            wy,
                            node_r * 0.5,
                            Color::from_rgba(255, 255, 220, main_alpha),
                        );
                        draw_circle(
                            wx + cell_size,
                            wy,
                            node_r,
                            Color::from_rgba(255, 191, 0, main_alpha),
                        );
                        draw_circle(
                            wx + cell_size,
                            wy,
                            node_r * 0.5,
                            Color::from_rgba(255, 255, 220, main_alpha),
                        );
                    } else {
                        draw_line(
                            wx,
                            wy,
                            wx + cell_size,
                            wy,
                            10.0 * scale,
                            Color::from_rgba(0, 229, 255, 90),
                        );
                        draw_line(
                            wx,
                            wy,
                            wx + cell_size,
                            wy,
                            5.0 * scale,
                            Color::from_rgba(0, 200, 230, 255),
                        );
                        draw_line(wx, wy, wx + cell_size, wy, 2.0 * scale, WHITE);

                        draw_circle(wx, wy, 5.0 * scale, Color::from_rgba(0, 229, 255, 255));
                        draw_circle(wx, wy, 2.0 * scale, WHITE);
                        draw_circle(
                            wx + cell_size,
                            wy,
                            5.0 * scale,
                            Color::from_rgba(0, 229, 255, 255),
                        );
                        draw_circle(wx + cell_size, wy, 2.0 * scale, WHITE);
                    }
                }
            }
        }

        for r in 0..rows {
            for c in 0..=cols {
                let edge = Edge::Vertical { r, c };
                if state.grid.get_edge(edge) == EdgeState::Wall {
                    let wx = gx + c as f32 * cell_size;
                    let wy = gy + r as f32 * cell_size;
                    let is_in_construction = state.placed_walls_this_turn.contains(&edge);

                    if is_in_construction {
                        let shine_phase = (t * 7.5).sin() * 0.5 + 0.5;
                        let main_alpha = (60.0 + 195.0 * shine_phase) as u8;
                        let aura_alpha = (10.0 + 200.0 * shine_phase) as u8;
                        let aura_w = (8.0 + 8.0 * shine_phase) * scale;

                        // Outer shining hazard aura
                        draw_line(
                            wx,
                            wy,
                            wx,
                            wy + cell_size,
                            aura_w,
                            Color::from_rgba(255, 180, 0, aura_alpha),
                        );
                        // Main hazard yellow line (shines on and off)
                        draw_line(
                            wx,
                            wy,
                            wx,
                            wy + cell_size,
                            6.0 * scale,
                            Color::from_rgba(
                                255,
                                (170.0 + 70.0 * shine_phase) as u8,
                                0,
                                main_alpha,
                            ),
                        );
                        // Neon yellow-white core
                        draw_line(
                            wx,
                            wy,
                            wx,
                            wy + cell_size,
                            2.0 * scale,
                            Color::from_rgba(
                                255,
                                255,
                                (160.0 + 95.0 * shine_phase) as u8,
                                main_alpha,
                            ),
                        );

                        let node_r = (4.0 + 2.0 * shine_phase) * scale;
                        draw_circle(wx, wy, node_r, Color::from_rgba(255, 191, 0, main_alpha));
                        draw_circle(
                            wx,
                            wy,
                            node_r * 0.5,
                            Color::from_rgba(255, 255, 220, main_alpha),
                        );
                        draw_circle(
                            wx,
                            wy + cell_size,
                            node_r,
                            Color::from_rgba(255, 191, 0, main_alpha),
                        );
                        draw_circle(
                            wx,
                            wy + cell_size,
                            node_r * 0.5,
                            Color::from_rgba(255, 255, 220, main_alpha),
                        );
                    } else {
                        draw_line(
                            wx,
                            wy,
                            wx,
                            wy + cell_size,
                            10.0 * scale,
                            Color::from_rgba(0, 229, 255, 90),
                        );
                        draw_line(
                            wx,
                            wy,
                            wx,
                            wy + cell_size,
                            5.0 * scale,
                            Color::from_rgba(0, 200, 230, 255),
                        );
                        draw_line(wx, wy, wx, wy + cell_size, 2.0 * scale, WHITE);

                        draw_circle(wx, wy, 5.0 * scale, Color::from_rgba(0, 229, 255, 255));
                        draw_circle(wx, wy, 2.0 * scale, WHITE);
                        draw_circle(
                            wx,
                            wy + cell_size,
                            5.0 * scale,
                            Color::from_rgba(0, 229, 255, 255),
                        );
                        draw_circle(wx, wy + cell_size, 2.0 * scale, WHITE);
                    }
                }
            }
        }

        // 9. Particle FX, Wet Droplet Jumps, Splashes & Shockwaves
        for event in &state.newly_cloned_this_step {
            let from_cx = gx + (event.from.1 as f32 + 0.5) * cell_size;
            let from_cy = gy + (event.from.0 as f32 + 0.5) * cell_size;
            let to_cx = gx + (event.to.1 as f32 + 0.5) * cell_size;
            let to_cy = gy + (event.to.0 as f32 + 0.5) * cell_size;
            self.spawn_clone_fx(CloneFxParams {
                from_r: event.from.0,
                from_c: event.from.1,
                to_r: event.to.0,
                to_c: event.to.1,
                from_x: from_cx,
                from_y: from_cy,
                to_x: to_cx,
                to_y: to_cy,
                cell_size,
            });
        }
        state.newly_cloned_this_step.clear();

        for &(r, c) in &state.newly_starved_this_step {
            let cx = gx + (c as f32 + 0.5) * cell_size;
            let cy = gy + (r as f32 + 0.5) * cell_size;
            self.spawn_shockwave(cx, cy, Color::from_rgba(255, 61, 0, 255), cell_size * 1.4);
            self.spawn_shockwave(cx, cy, Color::from_rgba(255, 145, 0, 255), cell_size * 1.0);
            self.spawn_burst(cx, cy, Color::from_rgba(255, 61, 0, 255), 24);
            self.spawn_burst(cx, cy, Color::from_rgba(255, 145, 0, 255), 24);
            self.spawn_burst(cx, cy, Color::from_rgba(255, 234, 0, 255), 16);
        }
        state.newly_starved_this_step.clear();

        // 9a. Draw Wet Biomass Droplet Jumps (Smooth, Viscous Parabolic Leaps)
        for jump in &self.wet_jumps {
            let progress = (1.0 - (jump.life / jump.max_life)).clamp(0.0, 1.0);

            if progress < 0.65 {
                let p_flight = (progress / 0.65).clamp(0.0, 1.0);
                // Hermite smoothstep easing for tangible organic weight
                let p = p_flight * p_flight * (3.0 - 2.0 * p_flight);

                let arc_h = jump.cell_size * 0.42;
                let x_base = jump.from_x + (jump.to_x - jump.from_x) * p;
                let y_base = jump.from_y + (jump.to_y - jump.from_y) * p;
                let y_arc = -arc_h * 4.0 * p * (1.0 - p);
                let drop_x = x_base;
                let drop_y = y_base + y_arc;

                // Motion vector & orientation for squash and stretch
                let dx = jump.to_x - jump.from_x;
                let dy = jump.to_y - jump.from_y - arc_h * 4.0 * (1.0 - 2.0 * p);
                let len = (dx * dx + dy * dy).sqrt().max(0.001);
                let dir_x = dx / len;
                let dir_y = dy / len;

                let v_speed = 4.0 * p * (1.0 - p);
                let stretch = 1.0 + 0.65 * v_speed;
                let squash = 1.0 / stretch.sqrt();

                // Fluid surface tension oscillation (wobble)
                let wobble = ((p * 26.0 + jump.seed).sin()) * 0.12 * (1.0 - p * 0.4);
                let base_r = jump.cell_size * 0.22 * (1.0 + wobble) * scale;

                // Viscous liquid neck / stretching filament during early flight (p < 0.45)
                if p < 0.45 {
                    let neck_fade = 1.0 - (p / 0.45);
                    let steps = 10;
                    let mut prev_pt = (jump.from_x, jump.from_y);
                    for i in 1..=steps {
                        let u = i as f32 / steps as f32;
                        let ux = jump.from_x + (drop_x - jump.from_x) * u;
                        let uy = jump.from_y + (drop_y - jump.from_y) * u
                            - (arc_h * 0.5) * (4.0 * u * (1.0 - u)) * neck_fade;
                        let taper = 1.0 - 0.65 * (4.0 * u * (1.0 - u));
                        let w =
                            (jump.cell_size * 0.20 * neck_fade * taper * scale).max(1.5 * scale);
                        draw_line(
                            prev_pt.0,
                            prev_pt.1,
                            ux,
                            uy,
                            w * 1.5,
                            Color::new(0.0, 0.9, 0.46, neck_fade * 0.35),
                        );
                        draw_line(
                            prev_pt.0,
                            prev_pt.1,
                            ux,
                            uy,
                            w,
                            Color::new(0.0, 0.9, 0.46, neck_fade * 0.85),
                        );
                        draw_line(
                            prev_pt.0,
                            prev_pt.1,
                            ux,
                            uy,
                            (w * 0.45).max(1.0),
                            Color::new(0.41, 0.94, 0.68, neck_fade * 0.95),
                        );
                        prev_pt = (ux, uy);
                    }
                }

                // Trailing micro-droplets (bead-on-a-string capillary effect)
                if p >= 0.18 {
                    let trails = [0.08, 0.16];
                    for (idx, &lag) in trails.iter().enumerate() {
                        let tp_flight = (p_flight - lag).max(0.0);
                        let tp = tp_flight * tp_flight * (3.0 - 2.0 * tp_flight);
                        let tx = jump.from_x + (jump.to_x - jump.from_x) * tp;
                        let ty = jump.from_y + (jump.to_y - jump.from_y) * tp
                            - arc_h * 4.0 * tp * (1.0 - tp);
                        let bead_r =
                            (jump.cell_size * (0.075 - idx as f32 * 0.022) * scale).max(2.0);
                        let bead_alpha = (1.0 - lag / 0.28).clamp(0.0, 1.0);

                        draw_circle(
                            tx,
                            ty,
                            bead_r * 1.5,
                            Color::new(0.0, 0.9, 0.46, bead_alpha * 0.35),
                        );
                        draw_circle(tx, ty, bead_r, Color::new(0.0, 0.9, 0.46, bead_alpha * 0.9));
                        draw_circle(
                            tx,
                            ty,
                            bead_r * 0.55,
                            Color::new(0.41, 0.94, 0.68, bead_alpha),
                        );
                        draw_circle(
                            tx - bead_r * 0.3,
                            ty - bead_r * 0.3,
                            (bead_r * 0.3).max(1.0),
                            Color::new(1.0, 1.0, 1.0, bead_alpha * 0.95),
                        );
                    }
                }

                // Main fluid teardrop blob (composite overlapping gelatinous discs)
                let head_pos = (
                    drop_x + dir_x * (base_r * 0.35 * stretch),
                    drop_y + dir_y * (base_r * 0.35 * stretch),
                );
                let body_pos = (drop_x, drop_y);
                let tail_pos = (
                    drop_x - dir_x * (base_r * 0.55 * stretch),
                    drop_y - dir_y * (base_r * 0.55 * stretch),
                );

                let blob_parts = [
                    (tail_pos, base_r * 0.65 * squash),
                    (body_pos, base_r * squash),
                    (head_pos, base_r * 0.95 * squash),
                ];

                // Outer soft bioluminescent slime glow
                for &(pos, r) in &blob_parts {
                    draw_circle(pos.0, pos.1, r * 1.55, Color::from_rgba(0, 230, 118, 55));
                }
                // Translucent gel mantle
                for &(pos, r) in &blob_parts {
                    draw_circle(pos.0, pos.1, r * 1.25, Color::from_rgba(0, 200, 100, 140));
                }
                // Viscous saturated emerald body
                for &(pos, r) in &blob_parts {
                    draw_circle(pos.0, pos.1, r, Color::from_rgba(0, 230, 118, 255));
                }
                // Bright cytoplasmic core
                for &(pos, r) in &blob_parts {
                    draw_circle(pos.0, pos.1, r * 0.6, Color::from_rgba(105, 240, 174, 255));
                }

                // Glossy specular reflection highlights (wet liquid sheen)
                let glint_x = head_pos.0 - base_r * 0.32;
                let glint_y = head_pos.1 - base_r * 0.32;
                draw_circle(
                    glint_x,
                    glint_y,
                    (base_r * 0.30).max(2.0),
                    Color::from_rgba(255, 255, 255, 245),
                );
                draw_circle(
                    glint_x + base_r * 0.22,
                    glint_y - base_r * 0.08,
                    (base_r * 0.15).max(1.2),
                    Color::from_rgba(255, 255, 255, 190),
                );
                // Lower rim bounce light
                draw_circle(
                    body_pos.0 + base_r * 0.30,
                    body_pos.1 + base_r * 0.30,
                    base_r * 0.22,
                    Color::from_rgba(174, 234, 0, 130),
                );
            } else {
                // Landing splat phase (pancake ripple & splash spread)
                let s = (progress - 0.65) / 0.35;
                let splat_r = jump.cell_size * (0.22 + 0.45 * s) * scale;
                let splat_alpha = ((1.0 - s) * 0.95).clamp(0.0, 1.0);

                draw_circle_lines(
                    jump.to_x,
                    jump.to_y,
                    splat_r,
                    (3.5 * (1.0 - s) * scale).max(1.0),
                    Color::new(0.0, 0.9, 0.46, splat_alpha),
                );
                if s < 0.6 {
                    let inner_r = splat_r * 0.6;
                    draw_circle_lines(
                        jump.to_x,
                        jump.to_y,
                        inner_r,
                        (2.5 * (1.0 - s / 0.6) * scale).max(1.0),
                        Color::new(0.41, 0.94, 0.68, splat_alpha),
                    );
                }
            }
        }

        // 9b. Draw Wet Splash Splatter Beads & Surface Slime Puddle
        for splash in &self.wet_splashes {
            let prog = (1.0 - (splash.life / splash.max_life)).clamp(0.0, 1.0);
            let alpha = (splash.life / splash.max_life).clamp(0.0, 1.0);

            // Expanding fluid puddle on tile surface
            let puddle_r = (splash.cell_size * (0.15 + 0.25 * prog) * scale).max(1.0);
            draw_circle(
                splash.cx,
                splash.cy,
                puddle_r * 1.3,
                Color::new(0.0, 0.9, 0.46, alpha * 0.25),
            );
            draw_circle(
                splash.cx,
                splash.cy,
                puddle_r,
                Color::new(0.41, 0.94, 0.68, alpha * 0.4),
            );

            for bead in &splash.beads {
                let bead_prog = (1.0 - (bead.life / bead.max_life)).clamp(0.0, 1.0);
                let alpha = (bead.life / bead.max_life).clamp(0.0, 1.0);
                let r = (bead.radius * (1.0 - bead_prog * 0.5) * scale).max(1.5);
                let col = Color::new(bead.color.r, bead.color.g, bead.color.b, alpha);

                draw_circle(
                    bead.x,
                    bead.y,
                    r * 1.4,
                    Color::new(0.0, 0.9, 0.46, alpha * 0.35),
                );
                draw_circle(bead.x, bead.y, r, col);
                draw_circle(
                    bead.x - r * 0.3,
                    bead.y - r * 0.3,
                    (r * 0.35).max(1.0),
                    Color::new(1.0, 1.0, 1.0, alpha * 0.9),
                );
            }
        }

        // 9c. Shockwaves
        for sw in &self.shockwaves {
            let color = Color::new(sw.color.r, sw.color.g, sw.color.b, sw.alpha.clamp(0.0, 1.0));
            draw_circle_lines(sw.cx, sw.cy, sw.radius, 4.0 * scale, color);
        }

        // 9d. Particle Bursts & Spores
        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let color = Color::new(p.color.r, p.color.g, p.color.b, alpha);
            draw_circle(p.x, p.y, p.radius, color);
        }

        // Switch back to screen camera
        set_default_camera();

        // Draw rendered target texture clipped smoothly at viewport bounds
        draw_texture_ex(
            &render_texture,
            viewport_x,
            viewport_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(viewport_w, viewport_h)),
                flip_y: true,
                ..Default::default()
            },
        );

        sound_trigger
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_edge_highlight(
        &self,
        edge: Edge,
        gx: f32,
        gy: f32,
        cell_w: f32,
        cell_h: f32,
        color: Color,
        scale: f32,
    ) {
        match edge {
            Edge::Horizontal { r, c } => {
                let wx = gx + c as f32 * cell_w;
                let wy = gy + r as f32 * cell_h;
                draw_line(wx, wy, wx + cell_w, wy, 8.0 * scale, color);
            }
            Edge::Vertical { r, c } => {
                let wx = gx + c as f32 * cell_w;
                let wy = gy + r as f32 * cell_h;
                draw_line(wx, wy, wx, wy + cell_h, 8.0 * scale, color);
            }
        }
    }

    fn draw_modal(
        &mut self,
        state: &mut GameState,
        screen_w: f32,
        screen_h: f32,
        scale: f32,
    ) -> Option<SoundTrigger> {
        let mut sound_trigger = None;

        let is_win = state.phase == GamePhase::Victory;
        let is_last_level = state.current_level_idx + 1 >= state.levels.len();
        let is_congrats = is_win && is_last_level;

        // Dark Translucent Backdrop
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::from_rgba(0, 0, 0, 190));

        // Manage and trigger confetti celebration on the final level victory
        if is_congrats {
            if self.confetti.is_empty() {
                self.spawn_confetti_burst(screen_w * 0.25, screen_h * 0.75, 45, scale);
                self.spawn_confetti_burst(screen_w * 0.75, screen_h * 0.75, 45, scale);
                self.spawn_confetti_burst(screen_w * 0.50, screen_h * 0.50, 30, scale);
            } else if self.confetti.len() < 160 {
                self.spawn_confetti_rain(screen_w, scale, 2);
            }
        }

        // Draw ambient celebration confetti
        self.draw_confetti();

        let card_w = if is_congrats {
            (460.0 * scale).min(screen_w * 0.94)
        } else {
            (420.0 * scale).min(screen_w * 0.92)
        };
        let card_h = if is_congrats {
            (290.0 * scale).min(screen_h * 0.88)
        } else {
            (260.0 * scale).min(screen_h * 0.85)
        };
        let card_x = (screen_w - card_w) / 2.0;
        let card_y = (screen_h - card_h) / 2.0;

        let border_color = if is_congrats {
            let pulse = (get_time() as f32 * 3.5).sin() * 0.5 + 0.5;
            Color::from_rgba(
                (255.0 * pulse + 0.0 * (1.0 - pulse)) as u8,
                (215.0 * pulse + 230.0 * (1.0 - pulse)) as u8,
                (0.0 * pulse + 118.0 * (1.0 - pulse)) as u8,
                255,
            )
        } else if is_win {
            Color::from_rgba(0, 230, 118, 255)
        } else {
            Color::from_rgba(255, 82, 82, 255)
        };

        // Dark Card Body (#0f172a)
        draw_rectangle(
            card_x,
            card_y,
            card_w,
            card_h,
            Color::from_rgba(15, 23, 42, 255),
        );
        draw_rectangle_lines(card_x, card_y, card_w, card_h, 3.5 * scale, border_color);

        let title = if is_congrats {
            "CONGRATULATIONS!"
        } else if is_win {
            "CONTAINMENT COMPLETE"
        } else {
            "☣ CONTAINMENT BREACHED"
        };
        let mut title_size = if is_congrats {
            26.0 * scale
        } else {
            24.0 * scale
        };
        let mut title_dim = self.measure_text_str(title, title_size);
        if title_dim.width > card_w - 20.0 * scale {
            title_size *= (card_w - 20.0 * scale) / title_dim.width;
            title_dim = self.measure_text_str(title, title_size);
        }
        let title_x = card_x + (card_w - title_dim.width) / 2.0;
        let title_color = if is_congrats {
            Color::from_rgba(255, 215, 0, 255)
        } else {
            border_color
        };
        self.draw_text_str(
            title,
            title_x,
            card_y + 42.0 * scale,
            title_size,
            title_color,
        );

        if is_congrats {
            let banner = "★ ALL SECTORS CONTAINED ★";
            let banner_dim = self.measure_text_str(banner, 16.0 * scale);
            self.draw_text_str(
                banner,
                card_x + (card_w - banner_dim.width) / 2.0,
                card_y + 70.0 * scale,
                16.0 * scale,
                Color::from_rgba(0, 230, 118, 255),
            );

            let stars_str = match state.star_rating {
                3 => "⭐ ⭐ ⭐",
                2 => "⭐ ⭐ ☆",
                _ => "⭐ ☆ ☆",
            };
            let star_dim = self.measure_text_str(stars_str, 32.0 * scale);
            self.draw_text_str(
                stars_str,
                card_x + (card_w - star_dim.width) / 2.0,
                card_y + 112.0 * scale,
                32.0 * scale,
                Color::from_rgba(255, 215, 0, 255), // Bright Gold
            );

            let msg = format!(
                "Facility secured! Final sector cleared in {} turns.",
                state.turn_number
            );
            let msg_dim = self.measure_text_str(&msg, 15.0 * scale);
            self.draw_text_str(
                &msg,
                card_x + (card_w - msg_dim.width) / 2.0,
                card_y + 155.0 * scale,
                15.0 * scale,
                WHITE,
            );

            let msg2 = "All bio-threats neutralized. Outstanding containment!";
            let msg2_dim = self.measure_text_str(msg2, 14.0 * scale);
            self.draw_text_str(
                msg2,
                card_x + (card_w - msg2_dim.width) / 2.0,
                card_y + 180.0 * scale,
                14.0 * scale,
                Color::from_rgba(148, 163, 184, 255),
            );
        } else if is_win {
            let stars_str = match state.star_rating {
                3 => "⭐ ⭐ ⭐",
                2 => "⭐ ⭐ ☆",
                _ => "⭐ ☆ ☆",
            };
            let star_dim = self.measure_text_str(stars_str, 32.0 * scale);
            self.draw_text_str(
                stars_str,
                card_x + (card_w - star_dim.width) / 2.0,
                card_y + 90.0 * scale,
                32.0 * scale,
                Color::from_rgba(255, 215, 0, 255), // Bright Gold
            );

            let msg = format!("Sector cleared in {} turns!", state.turn_number);
            let msg_dim = self.measure_text_str(&msg, 16.0 * scale);
            self.draw_text_str(
                &msg,
                card_x + (card_w - msg_dim.width) / 2.0,
                card_y + 135.0 * scale,
                16.0 * scale,
                WHITE,
            );
        } else {
            let msg = "Biomass capacity exceeded or no moves remain.";
            let msg_dim = self.measure_text_str(msg, 15.0 * scale);
            self.draw_text_str(
                msg,
                card_x + (card_w - msg_dim.width) / 2.0,
                card_y + 115.0 * scale,
                15.0 * scale,
                Color::from_rgba(226, 232, 240, 255),
            );
        }

        let btn_w = (140.0 * scale).min(card_w * 0.44);
        let btn_h = 36.0 * scale;
        let btn_y = card_y + card_h - 52.0 * scale;

        let mouse_pos = mouse_position();
        let clicked = is_mouse_button_released(MouseButton::Left);

        let retry_rect = Rect::new(card_x + 20.0 * scale, btn_y, btn_w, btn_h);
        let retry_label = if is_congrats {
            "↺ Replay Level"
        } else {
            "↺ Retry Level"
        };
        self.draw_button(retry_label, retry_rect, mouse_pos, 15.0 * scale);
        if clicked && retry_rect.contains(mouse_pos.into()) {
            state.reset_level();
            self.confetti.clear();
            sound_trigger = Some(SoundTrigger::ButtonClick);
        }

        let next_rect = Rect::new(card_x + card_w - btn_w - 20.0 * scale, btn_y, btn_w, btn_h);
        if is_congrats {
            self.draw_button("⏮ Start from First", next_rect, mouse_pos, 15.0 * scale);
            if clicked && next_rect.contains(mouse_pos.into()) {
                state.load_level(0);
                self.confetti.clear();
                sound_trigger = Some(SoundTrigger::ButtonClick);
            }
        } else if is_win {
            self.draw_button("▶ Next Level", next_rect, mouse_pos, 15.0 * scale);
            if clicked && next_rect.contains(mouse_pos.into()) {
                state.load_level(state.current_level_idx + 1);
                self.confetti.clear();
                sound_trigger = Some(SoundTrigger::ButtonClick);
            }
        } else {
            self.draw_button("⏭ Skip Level", next_rect, mouse_pos, 15.0 * scale);
            if clicked && next_rect.contains(mouse_pos.into()) {
                if state.current_level_idx + 1 < state.levels.len() {
                    state.load_level(state.current_level_idx + 1);
                } else {
                    state.reset_level();
                }
                self.confetti.clear();
                sound_trigger = Some(SoundTrigger::ButtonClick);
            }
        }

        sound_trigger
    }
}
