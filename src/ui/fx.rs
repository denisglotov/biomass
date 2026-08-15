use macroquad::prelude::Color;

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

#[derive(Debug, Clone)]
pub struct SplashBead {
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
pub struct WetSplash {
    pub cx: f32,
    pub cy: f32,
    pub cell_size: f32,
    pub life: f32,
    pub max_life: f32,
    pub beads: Vec<SplashBead>,
}

#[derive(Debug, Clone, Copy)]
pub struct CloneFxParams {
    pub from_r: usize,
    pub from_c: usize,
    pub to_r: usize,
    pub to_c: usize,
    pub from_x: f32,
    pub from_y: f32,
    pub to_x: f32,
    pub to_y: f32,
    pub cell_size: f32,
}

#[derive(Debug, Clone)]
pub struct WetDropletJump {
    pub from_r: usize,
    pub from_c: usize,
    pub to_r: usize,
    pub to_c: usize,
    pub from_x: f32,
    pub from_y: f32,
    pub to_x: f32,
    pub to_y: f32,
    pub life: f32,
    pub max_life: f32,
    pub cell_size: f32,
    pub seed: f32,
    pub has_splashed: bool,
}

#[derive(Debug, Clone)]
pub struct Confetti {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
    pub sway_phase: f32,
}
