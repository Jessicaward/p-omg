use macroquad::color::WHITE;
use macroquad::math::{vec2, Circle, Vec2};
use macroquad::shapes::draw_circle;
use macroquad::window::screen_height;
use rand::{rng, Rng};

pub const BALL_RADIUS:f32 = 15f32;
const BALL_SPEED:f32 = 8f32;

#[derive(Copy,Clone)]
pub struct Ball {
    circle:Circle,
    dir: Vec2 // direction of the ball
}

impl Ball {
    pub fn new(circle: Circle) -> Self {
        let mut rng = rng();
        let dir_x = rng.random_range(-1f32..=1f32);
        let dir_y = rng.random_range(-1f32..=1f32);
        Self {
            circle,
            dir:vec2(dir_x, dir_y)
        }
    }

    pub fn draw(&self) {
        let c = self.circle;
        draw_circle(c.x, c.y, c.r, WHITE);
    }

    pub fn move_ball(&mut self) {
        self.circle.x += self.dir.x * BALL_SPEED;
        self.circle.y += self.dir.y * BALL_SPEED;
        if self.circle.y > screen_height() - BALL_RADIUS || self.circle.y < 0.0 {
            self.dir.y = -self.dir.y;
        }
    }
}