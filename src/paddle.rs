use macroquad::color::WHITE;
use macroquad::input::is_key_down;
use macroquad::math::Rect;
use macroquad::miniquad::KeyCode;
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

    pub fn movement(&mut self,up:KeyCode,down:KeyCode) {
        if is_key_down(up) {
            self.rect.y -= 1.*PADDLE_SPEED;
        }else if is_key_down(down) {
            self.rect.y += 1.*PADDLE_SPEED;
        }
    }
}