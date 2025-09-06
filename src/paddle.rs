use macroquad::color::WHITE;
use macroquad::math::Rect;
use macroquad::shapes::draw_rectangle;

pub const PADDLE_WIDTH: f32 = 20f32;
pub const PADDLE_HEIGHT: f32 = 80f32;
pub const PADDLE_SPEED:f32 = 10f32;

#[derive(Clone, Copy)]
pub struct Paddle {
    rect: Rect
}

impl Paddle {
    pub fn new(rect: Rect) -> Self {
        Self {
            rect
        }
    }

    pub fn draw(&self) {
        let r = self.rect;
        draw_rectangle(r.x, r.y, r.w, r.h, WHITE)
    }
}