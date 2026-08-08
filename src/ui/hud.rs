use crate::game::grid::{CellType, Edge, EdgeState};
use crate::game::state::{GamePhase, GameState, SoundTrigger};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub radius: f32,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
}

#[derive(Debug, Clone)]
pub struct Shockwave {
    pub cx: f32,
    pub cy: f32,
    pub radius: f32,
    pub max_radius: f32,
    pub color: Color,
    pub alpha: f32,
}

pub struct Hud {
    pub font: Option<Font>,
    pub hovered_edge: Option<Edge>,
    pub particles: Vec<Particle>,
    pub shockwaves: Vec<Shockwave>,
}

impl Hud {
    pub fn new() -> Self {
        let font_bytes = include_bytes!("../../assets/fonts/Symbola.ttf");
        let font = load_ttf_font_from_bytes(font_bytes).ok();

        Self {
            font,
            hovered_edge: None,
            particles: Vec::new(),
            shockwaves: Vec::new(),
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

    pub fn update_fx(&mut self, dt: f32) {
        for p in self.particles.iter_mut() {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);

        for sw in self.shockwaves.iter_mut() {
            sw.radius += (sw.max_radius - sw.radius) * 7.0 * dt;
            sw.alpha -= dt * 1.8;
        }
        self.shockwaves.retain(|sw| sw.alpha > 0.0);
    }

    pub fn draw_and_handle_input(&mut self, state: &mut GameState) -> Option<SoundTrigger> {
        let dt = get_frame_time();
        self.update_fx(dt);

        let mut sound_trigger = None;

        let screen_w = screen_width();
        let screen_h = screen_height();

        // 1. High-Contrast Deep Slate Ambient Background (#0f172a)
        clear_background(Color::from_rgba(15, 23, 42, 255));

        // 2. Draw Header
        let header_sound = self.draw_header(screen_w);
        if header_sound.is_some() {
            sound_trigger = header_sound;
        }

        // 3. Draw Stats Bar
        let stats_y = 56.0;
        self.draw_stats_bar(state, screen_w, stats_y);

        // 4. Draw Level Description Banner
        let level_banner_y = stats_y + 56.0;
        self.draw_level_banner(state, screen_w, level_banner_y);

        // 5. Draw Control Bar (Level Selector, Action Buttons)
        let control_bar_y = level_banner_y + 44.0;
        let control_sound = self.draw_control_bar(state, screen_w, control_bar_y);
        if control_sound.is_some() {
            sound_trigger = control_sound;
        }

        // 6. Draw Grid & Interactive Workspace
        let grid_top = control_bar_y + 56.0;
        let grid_bottom_margin = 30.0;
        let available_h = (screen_h - grid_top - grid_bottom_margin).max(200.0);
        let available_w = (screen_w - 40.0).max(200.0);

        let grid_size = available_w.min(available_h).min(560.0);
        let grid_x = (screen_w - grid_size) / 2.0;
        let grid_y = grid_top + (available_h - grid_size) / 2.0;

        let grid_sound = self.draw_grid(state, grid_x, grid_y, grid_size);
        if grid_sound.is_some() {
            sound_trigger = grid_sound;
        }

        // 7. Draw Win / Loss Modal Overlay
        if state.phase == GamePhase::Victory || state.phase == GamePhase::Defeat {
            let modal_sound = self.draw_modal(state, screen_w, screen_h);
            if modal_sound.is_some() {
                sound_trigger = modal_sound;
            }
        }

        sound_trigger
    }

    fn draw_header(&self, screen_w: f32) -> Option<SoundTrigger> {
        let sound = None;
        // Dark Obsidian Header Bar
        draw_rectangle(0.0, 0.0, screen_w, 56.0, Color::from_rgba(15, 23, 42, 255));
        draw_line(
            0.0,
            56.0,
            screen_w,
            56.0,
            3.0,
            Color::from_rgba(0, 229, 255, 255),
        );

        let title = "☣ BIOMASS";
        let font_size = 32.0;
        let title_dim = self.measure_text_str(title, font_size);
        self.draw_text_str(
            title,
            20.0,
            38.0,
            font_size,
            Color::from_rgba(0, 230, 118, 255),
        );

        let subtitle = "TACTICAL BIOLOGICAL CONTAINMENT PROTOCOL";
        let subtitle_x = 20.0 + title_dim.width + 16.0;
        self.draw_text_str(
            subtitle,
            subtitle_x,
            35.0,
            14.0,
            Color::from_rgba(226, 232, 240, 255),
        );

        sound
    }

    fn draw_stats_bar(&self, state: &GameState, screen_w: f32, y: f32) {
        let h = 52.0;
        // Deep Slate Container (#1e293b)
        draw_rectangle(0.0, y, screen_w, h, Color::from_rgba(30, 41, 59, 255));
        draw_line(
            0.0,
            y + h,
            screen_w,
            y + h,
            2.0,
            Color::from_rgba(51, 65, 85, 255),
        );

        let item_w = (screen_w / 4.0).min(200.0);
        let start_x = (screen_w - item_w * 4.0) / 2.0;

        // Turn
        self.draw_stat_item(
            "⌛ TURN",
            &state.turn_number.to_string(),
            start_x,
            y,
            item_w,
            WHITE,
        );

        // Walls Remaining
        let walls_text = format!("{} / {}", state.walls_left, state.level.walls_per_turn);
        let walls_color = if state.walls_left > 0 {
            Color::from_rgba(0, 229, 255, 255)
        } else {
            Color::from_rgba(255, 171, 0, 255)
        };
        self.draw_stat_item(
            "🛡 WALLS LEFT",
            &walls_text,
            start_x + item_w,
            y,
            item_w,
            walls_color,
        );

        // Active Biomass
        let bio_count = state.grid.count_biomass();
        let bio_text = bio_count.to_string();
        let bio_color = Color::from_rgba(0, 230, 118, 255);
        self.draw_stat_item(
            "☣ ACTIVE BIOMASS",
            &bio_text,
            start_x + item_w * 2.0,
            y,
            item_w,
            bio_color,
        );

        // Max Capacity
        let max_text = state.level.max_threshold.to_string();
        self.draw_stat_item(
            "⚠️ MAX CAPACITY",
            &max_text,
            start_x + item_w * 3.0,
            y,
            item_w,
            Color::from_rgba(255, 82, 82, 255),
        );
    }

    fn draw_stat_item(&self, label: &str, value: &str, x: f32, y: f32, _w: f32, val_color: Color) {
        self.draw_text_str(
            label,
            x + 10.0,
            y + 20.0,
            14.0,
            Color::from_rgba(148, 163, 184, 255),
        );
        self.draw_text_str(value, x + 10.0, y + 43.0, 24.0, val_color);
    }

    fn draw_level_banner(&self, state: &GameState, screen_w: f32, y: f32) {
        let text = format!("☣ {} — {}", state.level.title, state.level.description);
        let font_size = 18.0;
        let dimensions = self.measure_text_str(&text, font_size);

        let draw_x = (screen_w - dimensions.width) / 2.0;

        // Dark Slate Banner Box
        let box_w = (dimensions.width + 30.0).max(300.0);
        let box_x = (screen_w - box_w) / 2.0;
        draw_rectangle(box_x, y, box_w, 34.0, Color::from_rgba(30, 41, 59, 255));
        draw_rectangle_lines(
            box_x,
            y,
            box_w,
            34.0,
            1.5,
            Color::from_rgba(0, 229, 255, 180),
        );

        self.draw_text_str(
            &text,
            draw_x.max(10.0),
            y + 23.0,
            font_size,
            Color::from_rgba(255, 255, 255, 255),
        );
    }

    fn draw_control_bar(
        &self,
        state: &mut GameState,
        screen_w: f32,
        y: f32,
    ) -> Option<SoundTrigger> {
        let mut sound_trigger = None;

        let center_x = screen_w / 2.0;

        // Level Selector (< Level X >)
        let btn_w = 36.0;
        let btn_h = 34.0;
        let selector_w = 250.0;

        let sel_left = center_x - 270.0;
        let prev_rect = Rect::new(sel_left, y, btn_w, btn_h);
        let next_rect = Rect::new(sel_left + selector_w - btn_w, y, btn_w, btn_h);

        let mouse_pos = mouse_position();
        let clicked = is_mouse_button_pressed(MouseButton::Left);

        // Prev Level Button
        self.draw_button("◀", prev_rect, mouse_pos, 18.0);
        if clicked && prev_rect.contains(mouse_pos.into()) && state.current_level_idx > 0 {
            state.load_level(state.current_level_idx - 1);
            sound_trigger = Some(SoundTrigger::WallPlace);
        }

        // Level Label
        let lvl_str = format!(
            "LEVEL {} / {}",
            state.current_level_idx + 1,
            state.levels.len()
        );
        self.draw_text_str(
            &lvl_str,
            sel_left + 52.0,
            y + 23.0,
            18.0,
            Color::from_rgba(0, 229, 255, 255),
        );

        // Next Level Button
        self.draw_button("▶", next_rect, mouse_pos, 18.0);
        if clicked
            && next_rect.contains(mouse_pos.into())
            && state.current_level_idx + 1 < state.levels.len()
        {
            state.load_level(state.current_level_idx + 1);
            sound_trigger = Some(SoundTrigger::WallPlace);
        }

        // Action Buttons: Undo, Reset, End Turn
        let act_start_x = center_x - 10.0;

        let undo_rect = Rect::new(act_start_x, y, 105.0, btn_h);
        self.draw_button("↩ Undo (Z)", undo_rect, mouse_pos, 15.0);
        if ((clicked && undo_rect.contains(mouse_pos.into())) || is_key_pressed(KeyCode::Z))
            && state.undo()
        {
            sound_trigger = Some(SoundTrigger::WallPlace);
        }

        let reset_rect = Rect::new(act_start_x + 113.0, y, 105.0, btn_h);
        self.draw_button("↺ Reset (R)", reset_rect, mouse_pos, 15.0);
        if (clicked && reset_rect.contains(mouse_pos.into())) || is_key_pressed(KeyCode::R) {
            state.reset_level();
            sound_trigger = Some(SoundTrigger::WallPlace);
        }

        let end_rect = Rect::new(act_start_x + 226.0, y, 135.0, btn_h);
        self.draw_button("▶ End Turn", end_rect, mouse_pos, 15.0);
        if ((clicked && end_rect.contains(mouse_pos.into())) || is_key_pressed(KeyCode::Space))
            && state.phase == GamePhase::PlayerTurn
        {
            state.end_turn();
            sound_trigger = Some(SoundTrigger::WallPlace);
        }

        sound_trigger
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

        let dim = self.measure_text_str(label, font_size);
        let text_x = rect.x + (rect.w - dim.width) / 2.0;
        let text_y = rect.y + (rect.h + dim.height) / 2.0 - 2.0;
        self.draw_text_str(label, text_x, text_y, font_size, WHITE);
    }

    fn draw_grid(
        &mut self,
        state: &mut GameState,
        gx: f32,
        gy: f32,
        size: f32,
    ) -> Option<SoundTrigger> {
        let mut sound_trigger = None;

        let rows = state.grid.rows;
        let cols = state.grid.cols;
        let cell_w = size / cols as f32;
        let cell_h = size / rows as f32;

        let t = get_time() as f32;

        // 1. High-Contrast Outer Board Container (#1e293b)
        draw_rectangle(
            gx - 10.0,
            gy - 10.0,
            size + 20.0,
            size + 20.0,
            Color::from_rgba(30, 41, 59, 255),
        );
        draw_rectangle_lines(
            gx - 10.0,
            gy - 10.0,
            size + 20.0,
            size + 20.0,
            3.0,
            Color::from_rgba(0, 229, 255, 255),
        );

        // 2. Render Checkered Grid Cells
        for r in 0..rows {
            for c in 0..cols {
                let cx = gx + c as f32 * cell_w;
                let cy = gy + r as f32 * cell_h;

                let cell_type = state.grid.get_cell(r, c);

                match cell_type {
                    CellType::Empty => {
                        // High-Contrast Clean Checkered Tiles
                        let tile_color = if (r + c) % 2 == 0 {
                            Color::from_rgba(248, 250, 252, 255) // Pristine Pure Light
                        } else {
                            Color::from_rgba(226, 232, 240, 255) // Crisp Pearl
                        };

                        draw_rectangle(cx + 1.0, cy + 1.0, cell_w - 2.0, cell_h - 2.0, tile_color);

                        // High-Contrast Grid Lines (#0284c7)
                        draw_rectangle_lines(
                            cx + 1.0,
                            cy + 1.0,
                            cell_w - 2.0,
                            cell_h - 2.0,
                            1.0,
                            Color::from_rgba(2, 132, 199, 120),
                        );
                    }
                    CellType::Biomass => {
                        // Mint Floor for Biomass Cell
                        draw_rectangle(
                            cx + 1.0,
                            cy + 1.0,
                            cell_w - 2.0,
                            cell_h - 2.0,
                            Color::from_rgba(209, 250, 229, 255),
                        );
                        draw_rectangle_lines(
                            cx + 1.0,
                            cy + 1.0,
                            cell_w - 2.0,
                            cell_h - 2.0,
                            1.5,
                            Color::from_rgba(0, 230, 118, 200),
                        );

                        // 3D Candy Bio Nucleus & 4 Orbiting Spore Blobs
                        let center_x = cx + cell_w / 2.0;
                        let center_y = cy + cell_h / 2.0;
                        let pulse = (t * 4.0 + (r + c) as f32).sin() * 2.5;
                        let base_r = cell_w.min(cell_h) * 0.29 + pulse;

                        // Soft Translucent Green Aura Fading Out
                        draw_circle(
                            center_x,
                            center_y,
                            base_r * 1.5,
                            Color::from_rgba(0, 230, 118, 50),
                        );
                        draw_circle(
                            center_x,
                            center_y,
                            base_r * 1.2,
                            Color::from_rgba(0, 230, 118, 100),
                        );

                        // Main Vibrant Candy Green Bio Nucleus
                        draw_circle(
                            center_x,
                            center_y,
                            base_r,
                            Color::from_rgba(0, 230, 118, 255),
                        );

                        // 4 Orbiting Spore Satellites
                        for i in 0..4 {
                            let angle = t * 2.8 + (i as f32 * std::f32::consts::TAU / 4.0);
                            let orbit_radius = base_r * 0.55;
                            let ox = center_x + angle.cos() * orbit_radius;
                            let oy = center_y + angle.sin() * orbit_radius;
                            draw_circle(
                                ox,
                                oy,
                                base_r * 0.28,
                                Color::from_rgba(105, 240, 174, 255),
                            );
                        }

                        // Glossy 3D Highlight Arc
                        draw_circle(
                            center_x - base_r * 0.25,
                            center_y - base_r * 0.25,
                            base_r * 0.28,
                            WHITE,
                        );
                    }
                    CellType::Obstacle => {
                        // High-Contrast Slate Obstacle Pillar
                        draw_rectangle(
                            cx + 1.0,
                            cy + 1.0,
                            cell_w - 2.0,
                            cell_h - 2.0,
                            Color::from_rgba(100, 116, 139, 255),
                        );
                        draw_rectangle_lines(
                            cx + 2.0,
                            cy + 2.0,
                            cell_w - 4.0,
                            cell_h - 4.0,
                            2.0,
                            Color::from_rgba(51, 65, 85, 255),
                        );
                        draw_line(
                            cx + 4.0,
                            cy + 4.0,
                            cx + cell_w - 4.0,
                            cy + cell_h - 4.0,
                            2.0,
                            Color::from_rgba(15, 23, 42, 255),
                        );
                    }
                }
            }
        }

        // 3. Mouse Hover & Edge Detection
        self.hovered_edge = None;
        let mouse_pos = mouse_position();
        let mx = mouse_pos.0;
        let my = mouse_pos.1;

        if state.phase == GamePhase::PlayerTurn
            && state.walls_left > 0
            && mx >= gx
            && mx <= gx + size
            && my >= gy
            && my <= gy + size
        {
            let rel_x = mx - gx;
            let rel_y = my - gy;

            let c = (rel_x / cell_w).floor() as usize;
            let r = (rel_y / cell_h).floor() as usize;

            if c < cols && r < rows {
                let dist_top = rel_y - (r as f32 * cell_h);
                let dist_bottom = ((r + 1) as f32 * cell_h) - rel_y;
                let dist_left = rel_x - (c as f32 * cell_w);
                let dist_right = ((c + 1) as f32 * cell_w) - rel_x;

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

                if state.grid.can_place_wall(edge) {
                    self.hovered_edge = Some(edge);
                }
            }
        }

        // 4. Draw Hovered Edge Highlight & Click Handler
        if let Some(edge) = self.hovered_edge {
            self.draw_edge_highlight(
                edge,
                gx,
                gy,
                cell_w,
                cell_h,
                Color::from_rgba(0, 229, 255, 255),
            );

            if is_mouse_button_pressed(MouseButton::Left) && state.try_place_wall(edge) {
                sound_trigger = Some(SoundTrigger::WallPlace);

                // Spawn wall placement sparkling burst & shockwave
                let (wx, wy) = match edge {
                    Edge::Horizontal { r, c } => {
                        (gx + (c as f32 + 0.5) * cell_w, gy + r as f32 * cell_h)
                    }
                    Edge::Vertical { r, c } => {
                        (gx + c as f32 * cell_w, gy + (r as f32 + 0.5) * cell_h)
                    }
                };
                self.spawn_shockwave(wx, wy, Color::from_rgba(0, 229, 255, 255), cell_w * 0.9);
                self.spawn_burst(wx, wy, Color::from_rgba(0, 229, 255, 255), 16);
            }
        }

        // 5. Draw 3D Cyan Barricade Walls with Metallic End Caps
        for r in 0..=rows {
            for c in 0..cols {
                if state.grid.get_edge(Edge::Horizontal { r, c }) == EdgeState::Wall {
                    let wx = gx + c as f32 * cell_w;
                    let wy = gy + r as f32 * cell_h;

                    // Outer cyan glow line
                    draw_line(
                        wx,
                        wy,
                        wx + cell_w,
                        wy,
                        10.0,
                        Color::from_rgba(0, 229, 255, 90),
                    );
                    // Vibrant 3D cyan wall core
                    draw_line(
                        wx,
                        wy,
                        wx + cell_w,
                        wy,
                        5.0,
                        Color::from_rgba(0, 200, 230, 255),
                    );
                    // Specular white center line
                    draw_line(wx, wy, wx + cell_w, wy, 2.0, WHITE);

                    // Metallic post caps
                    draw_circle(wx, wy, 5.0, Color::from_rgba(0, 229, 255, 255));
                    draw_circle(wx, wy, 2.0, WHITE);
                    draw_circle(wx + cell_w, wy, 5.0, Color::from_rgba(0, 229, 255, 255));
                    draw_circle(wx + cell_w, wy, 2.0, WHITE);
                }
            }
        }

        for r in 0..rows {
            for c in 0..=cols {
                if state.grid.get_edge(Edge::Vertical { r, c }) == EdgeState::Wall {
                    let wx = gx + c as f32 * cell_w;
                    let wy = gy + r as f32 * cell_h;

                    // Outer cyan glow line
                    draw_line(
                        wx,
                        wy,
                        wx,
                        wy + cell_h,
                        10.0,
                        Color::from_rgba(0, 229, 255, 90),
                    );
                    // Vibrant 3D cyan wall core
                    draw_line(
                        wx,
                        wy,
                        wx,
                        wy + cell_h,
                        5.0,
                        Color::from_rgba(0, 200, 230, 255),
                    );
                    // Specular white center line
                    draw_line(wx, wy, wx, wy + cell_h, 2.0, WHITE);

                    // Metallic post caps
                    draw_circle(wx, wy, 5.0, Color::from_rgba(0, 229, 255, 255));
                    draw_circle(wx, wy, 2.0, WHITE);
                    draw_circle(wx, wy + cell_h, 5.0, Color::from_rgba(0, 229, 255, 255));
                    draw_circle(wx, wy + cell_h, 2.0, WHITE);
                }
            }
        }

        // 6. Process Burning Blast Effect ONLY when trapped biomass is caught and starves
        for &(r, c) in &state.newly_starved_this_step {
            let cx = gx + (c as f32 + 0.5) * cell_w;
            let cy = gy + (r as f32 + 0.5) * cell_h;
            // Fiery burning flame shockwaves & ember ash bursts
            self.spawn_shockwave(cx, cy, Color::from_rgba(255, 61, 0, 255), cell_w * 1.4);
            self.spawn_shockwave(cx, cy, Color::from_rgba(255, 145, 0, 255), cell_w * 1.0);
            self.spawn_burst(cx, cy, Color::from_rgba(255, 61, 0, 255), 24);
            self.spawn_burst(cx, cy, Color::from_rgba(255, 145, 0, 255), 24);
            self.spawn_burst(cx, cy, Color::from_rgba(255, 234, 0, 255), 16);
        }
        state.newly_starved_this_step.clear();
        state.newly_infected_this_step.clear();

        // 7. Render Particle FX & Blast Shockwaves
        for sw in &self.shockwaves {
            let color = Color::new(sw.color.r, sw.color.g, sw.color.b, sw.alpha.clamp(0.0, 1.0));
            draw_circle_lines(sw.cx, sw.cy, sw.radius, 4.0, color);
        }

        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let color = Color::new(p.color.r, p.color.g, p.color.b, alpha);
            draw_circle(p.x, p.y, p.radius, color);
        }

        sound_trigger
    }

    fn draw_edge_highlight(
        &self,
        edge: Edge,
        gx: f32,
        gy: f32,
        cell_w: f32,
        cell_h: f32,
        color: Color,
    ) {
        match edge {
            Edge::Horizontal { r, c } => {
                let wx = gx + c as f32 * cell_w;
                let wy = gy + r as f32 * cell_h;
                draw_line(wx, wy, wx + cell_w, wy, 8.0, color);
            }
            Edge::Vertical { r, c } => {
                let wx = gx + c as f32 * cell_w;
                let wy = gy + r as f32 * cell_h;
                draw_line(wx, wy, wx, wy + cell_h, 8.0, color);
            }
        }
    }

    fn draw_modal(
        &self,
        state: &mut GameState,
        screen_w: f32,
        screen_h: f32,
    ) -> Option<SoundTrigger> {
        let mut sound_trigger = None;

        // Dark Translucent Backdrop
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::from_rgba(0, 0, 0, 190));

        let card_w = 460.0;
        let card_h = 280.0;
        let card_x = (screen_w - card_w) / 2.0;
        let card_y = (screen_h - card_h) / 2.0;

        let is_win = state.phase == GamePhase::Victory;

        let border_color = if is_win {
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
        draw_rectangle_lines(card_x, card_y, card_w, card_h, 4.0, border_color);

        let title = if is_win {
            "CONTAINMENT COMPLETE"
        } else {
            "☣ CONTAINMENT BREACHED"
        };
        let title_dim = self.measure_text_str(title, 28.0);
        let title_x = card_x + (card_w - title_dim.width) / 2.0;
        self.draw_text_str(title, title_x, card_y + 48.0, 28.0, border_color);

        if is_win {
            let stars_str = match state.star_rating {
                3 => "⭐ ⭐ ⭐",
                2 => "⭐ ⭐ ☆",
                _ => "⭐ ☆ ☆",
            };
            let star_dim = self.measure_text_str(stars_str, 38.0);
            self.draw_text_str(
                stars_str,
                card_x + (card_w - star_dim.width) / 2.0,
                card_y + 100.0,
                38.0,
                Color::from_rgba(255, 215, 0, 255), // Bright Gold
            );

            let msg = format!("Sector cleared in {} turns!", state.turn_number);
            let msg_dim = self.measure_text_str(&msg, 18.0);
            self.draw_text_str(
                &msg,
                card_x + (card_w - msg_dim.width) / 2.0,
                card_y + 150.0,
                18.0,
                WHITE,
            );
        } else {
            let msg = "Biomass capacity exceeded or no moves remain.";
            let msg_dim = self.measure_text_str(msg, 17.0);
            self.draw_text_str(
                msg,
                card_x + (card_w - msg_dim.width) / 2.0,
                card_y + 130.0,
                17.0,
                Color::from_rgba(226, 232, 240, 255),
            );
        }

        let btn_w = 145.0;
        let btn_h = 38.0;
        let btn_y = card_y + card_h - 62.0;

        let mouse_pos = mouse_position();
        let clicked = is_mouse_button_pressed(MouseButton::Left);

        let retry_rect = Rect::new(card_x + 50.0, btn_y, btn_w, btn_h);
        self.draw_button("↺ Retry Level", retry_rect, mouse_pos, 16.0);
        if clicked && retry_rect.contains(mouse_pos.into()) {
            state.reset_level();
            sound_trigger = Some(SoundTrigger::WallPlace);
        }

        let next_rect = Rect::new(card_x + card_w - 195.0, btn_y, btn_w, btn_h);
        if is_win {
            self.draw_button("▶ Next Level", next_rect, mouse_pos, 16.0);
            if clicked && next_rect.contains(mouse_pos.into()) {
                if state.current_level_idx + 1 < state.levels.len() {
                    state.load_level(state.current_level_idx + 1);
                } else {
                    state.reset_level();
                }
                sound_trigger = Some(SoundTrigger::WallPlace);
            }
        }

        sound_trigger
    }
}
