mod paddle;
mod ball;
mod fs;

use macroquad::prelude::*;
use macroquad_particles::{self as particles, AtlasConfig, BlendMode, EmitterConfig};
use crate::ball::BALL_RADIUS;

const WINDOW_WIDTH:i32 = 1280;
const WINDOW_HEIGHT:i32 = 720;

#[macroquad::main(conf)]
async fn main() {
    //we need a timer when the round ends for the explosion animation
    let mut round_over_timer: Option<f64> = None;

    //this enables explosion later
    let texture = load_texture("assets/smoke_fire.png").await.unwrap();
    let mut one_shot_emitter = particles::Emitter::new(EmitterConfig {
        texture: Some(texture.clone()),
        ..explosion()
    });

    //Left paddle
    let mut player_paddle = paddle::Paddle::new(Rect::new(
        WINDOW_WIDTH as f32 / 8.0,  //x
        WINDOW_HEIGHT as f32 / 8.0, //y
        paddle::PADDLE_WIDTH,       //width
        paddle::PADDLE_HEIGHT       //height
    ));

    //Right paddle
    let mut ai_paddle = paddle::Paddle::new(Rect::new(
        (WINDOW_WIDTH as f32 / 8.0) * 7.0, //x
        WINDOW_HEIGHT as f32 / 8.0 ,       //y
        paddle::PADDLE_WIDTH,              //width
        paddle::PADDLE_HEIGHT              //height
    ));

    let mut ball = ball::Ball::new(Circle::new(screen_width() / 2., screen_height() /2., BALL_RADIUS));
    let mut system_file = get_random_file();

    loop {
        clear_background(BLACK);

        //text
        draw_text(&format!("now playing with {}", system_file.to_string_lossy().as_ref()), 20.0, 20.0, 30.0, WHITE);

        //draw paddles and ball
        player_paddle.draw();
        ai_paddle.draw();
        ball.draw();
        one_shot_emitter.draw(vec2(ball.circle.x, ball.circle.y));

        if round_over_timer.is_none() {
            //Game is going on as normal, allow collision and movement

            //movement
            ai_paddle.ai_movement(&ball);
            player_paddle.movement(KeyCode::W, KeyCode::S);
            ball.move_ball();

            //collision
            ball.collision_with_paddle(&player_paddle.rect);
            ball.collision_with_paddle(&ai_paddle.rect);

            //check for wins
            if ball.circle.x < 0.0 {
                //ai wins round

                //pause the game for 1 sec for explosion animation
                round_over_timer = Some(get_time() + 1.0);

                //reset ball
                ball = ball::Ball::new(Circle::new(screen_width()/2.,screen_height()/2.,BALL_RADIUS));
            } else if ball.circle.x > screen_width() {
                //player wins round, reset file and ball
                system_file = get_random_file();
                ball = ball::Ball::new(Circle::new(screen_width()/2.,screen_height()/2.,BALL_RADIUS));
            }
        } else {
            //check if pause is over
            if let Some(end_time) = round_over_timer {
                if get_time() >= end_time {
                    // reset ball and pick new file
                    system_file = get_random_file();
                    ball = ball::Ball::new(Circle::new(
                        screen_width() / 2.,
                        screen_height() / 2.,
                        BALL_RADIUS,
                    ));
                    round_over_timer = None; // unpause
                }
                one_shot_emitter.config.emitting = true
            } else {
                system_file = get_random_file();
            }
        }

        next_frame().await
    }
}

fn get_random_file() -> std::path::PathBuf {
    fs::random_file("/home/jessica/pomg").unwrap()
}

fn explosion() -> EmitterConfig {
    EmitterConfig {
        one_shot: true,
        emitting: false,
        lifetime: 0.3,
        lifetime_randomness: 0.7,
        explosiveness: 0.95,
        amount: 30,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 200.0,
        size: 30.0,
        gravity: vec2(0.0, -1000.0),
        atlas: Some(AtlasConfig::new(4, 4, 8..)),
        blend_mode: BlendMode::Additive,
        ..Default::default()
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("p-omg: save your system files!"),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        fullscreen: false,
        high_dpi: true,
        window_resizable: false,
        ..Default::default()
    }
}