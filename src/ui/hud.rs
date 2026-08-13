use crate::game::grid::{CellType, Edge, EdgeState};
use crate::game::state::{GamePhase, GameState, SoundTrigger};
use macroquad::prelude::*;

use super::fx::{Particle, Shockwave};

pub struct Hud {
    pub font: Option<Font>,
    pub hovered_edge: Option<Edge>,
    pub suppressed_hover_edge: Option<Edge>,
    pub particles: Vec<Particle>,
    pub shockwaves: Vec<Shockwave>,
    pub pan_offset: (f32, f32),
    pub drag_start: Option<(f32, f32)>,
    pub is_dragging: bool,
    pub last_level_idx: usize,
    pub render_target: Option<RenderTarget>,
}

impl Hud {
    pub fn new() -> Self {
        let font_bytes = include_bytes!("../../assets/fonts/Symbola.ttf");
        let font = load_ttf_font_from_bytes(font_bytes).ok();

        Self {
            font,
            hovered_edge: None,
            suppressed_hover_edge: None,
            particles: Vec::new(),
            shockwaves: Vec::new(),
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
            38.0 * scale
        } else {
            32.0 * scale
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
            "⚠️ MAX",
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
        let mut font_size = 16.0 * scale;
        let mut dimensions = self.measure_text_str(&text, font_size);

        // Dynamically shrink font size if text is too wide for screen
        let max_w = screen_w - 30.0 * scale;
        if dimensions.width > max_w {
            let ratio = (max_w / dimensions.width).clamp(0.65, 1.0);
            font_size *= ratio;
            dimensions = self.measure_text_str(&text, font_size);
        }

        let box_h = 32.0 * scale;
        let box_w = (dimensions.width + 24.0 * scale).min(screen_w - 20.0 * scale);
        let box_x = (screen_w - box_w) / 2.0;
        draw_rectangle(box_x, y, box_w, box_h, Color::from_rgba(30, 41, 59, 255));
        draw_rectangle_lines(
            box_x,
            y,
            box_w,
            box_h,
            1.5 * scale,
            Color::from_rgba(0, 229, 255, 180),
        );

        let draw_x = (screen_w - dimensions.width) / 2.0;
        self.draw_text_str(
            &text,
            draw_x.max(box_x + 6.0),
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
                        draw_rectangle(
                            cx + 1.0,
                            cy + 1.0,
                            cell_size - 2.0,
                            cell_size - 2.0,
                            Color::from_rgba(209, 250, 229, 255),
                        );
                        draw_rectangle_lines(
                            cx + 1.0,
                            cy + 1.0,
                            cell_size - 2.0,
                            cell_size - 2.0,
                            1.5,
                            Color::from_rgba(0, 230, 118, 200),
                        );

                        let center_x = cx + cell_size / 2.0;
                        let center_y = cy + cell_size / 2.0;
                        let pulse = (t * 4.0 + (r + c) as f32).sin() * (2.5 * scale);
                        let base_r = cell_size * 0.28 + pulse;

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
                        draw_circle(
                            center_x,
                            center_y,
                            base_r,
                            Color::from_rgba(0, 230, 118, 255),
                        );

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

                        draw_circle(
                            center_x - base_r * 0.25,
                            center_y - base_r * 0.25,
                            base_r * 0.28,
                            WHITE,
                        );
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

        // 9. Particle FX & Blast Shockwaves
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
        state.newly_infected_this_step.clear();

        for sw in &self.shockwaves {
            let color = Color::new(sw.color.r, sw.color.g, sw.color.b, sw.alpha.clamp(0.0, 1.0));
            draw_circle_lines(sw.cx, sw.cy, sw.radius, 4.0 * scale, color);
        }

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
        &self,
        state: &mut GameState,
        screen_w: f32,
        screen_h: f32,
        scale: f32,
    ) -> Option<SoundTrigger> {
        let mut sound_trigger = None;

        // Dark Translucent Backdrop
        draw_rectangle(0.0, 0.0, screen_w, screen_h, Color::from_rgba(0, 0, 0, 190));

        let card_w = (420.0 * scale).min(screen_w * 0.92);
        let card_h = (260.0 * scale).min(screen_h * 0.85);
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
        draw_rectangle_lines(card_x, card_y, card_w, card_h, 3.0 * scale, border_color);

        let title = if is_win {
            "CONTAINMENT COMPLETE"
        } else {
            "☣ CONTAINMENT BREACHED"
        };
        let mut title_size = 24.0 * scale;
        let mut title_dim = self.measure_text_str(title, title_size);
        if title_dim.width > card_w - 20.0 * scale {
            title_size *= (card_w - 20.0 * scale) / title_dim.width;
            title_dim = self.measure_text_str(title, title_size);
        }
        let title_x = card_x + (card_w - title_dim.width) / 2.0;
        self.draw_text_str(
            title,
            title_x,
            card_y + 44.0 * scale,
            title_size,
            border_color,
        );

        if is_win {
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

        let btn_w = (130.0 * scale).min(card_w * 0.42);
        let btn_h = 36.0 * scale;
        let btn_y = card_y + card_h - 52.0 * scale;

        let mouse_pos = mouse_position();
        let clicked = is_mouse_button_released(MouseButton::Left);

        let retry_rect = Rect::new(card_x + 20.0 * scale, btn_y, btn_w, btn_h);
        self.draw_button("↺ Retry Level", retry_rect, mouse_pos, 15.0 * scale);
        if clicked && retry_rect.contains(mouse_pos.into()) {
            state.reset_level();
            sound_trigger = Some(SoundTrigger::ButtonClick);
        }

        let next_rect = Rect::new(card_x + card_w - btn_w - 20.0 * scale, btn_y, btn_w, btn_h);
        if is_win {
            self.draw_button("▶ Next Level", next_rect, mouse_pos, 15.0 * scale);
            if clicked && next_rect.contains(mouse_pos.into()) {
                if state.current_level_idx + 1 < state.levels.len() {
                    state.load_level(state.current_level_idx + 1);
                } else {
                    state.reset_level();
                }
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
                sound_trigger = Some(SoundTrigger::ButtonClick);
            }
        }

        sound_trigger
    }
}
