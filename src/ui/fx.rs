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
