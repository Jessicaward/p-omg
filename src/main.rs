mod paddle;
mod ball;

use macroquad::prelude::*;
use crate::ball::BALL_RADIUS;

const WINDOW_WIDTH:i32 = 1280;
const WINDOW_HEIGHT:i32 = 720;

#[macroquad::main(conf)]
async fn main() {
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

    loop {
        clear_background(BLACK);

        //player paddle
        player_paddle.movement(KeyCode::W, KeyCode::S);
        player_paddle.draw();

        //ai paddle
        ai_paddle.draw();

        //ball
        ball.draw();
        ball.move_ball();

        next_frame().await
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
        icon: None,
        sample_count: 0,
        platform: Default::default(),
    }
}