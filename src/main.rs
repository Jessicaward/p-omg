mod paddle;
mod ball;
mod fs;

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

    let system_file = get_random_file();

    loop {
        clear_background(BLACK);

        //text
        draw_text(&format!("now playing with {}", system_file.to_string_lossy().as_ref()), 20.0, 20.0, 30.0, WHITE);

        //player paddle
        player_paddle.movement(KeyCode::W, KeyCode::S);
        player_paddle.draw();

        //ai paddle
        ai_paddle.draw();
        ai_paddle.ai_movement(&ball);

        //ball
        ball.draw();
        ball.move_ball();

        //collision
        ball.collision_with_paddle(&player_paddle.rect);
        ball.collision_with_paddle(&ai_paddle.rect);

        //check for win / lose
        if ball.circle.x < 0.0 {
            //ai wins round

            //TODO burn file
            ball = ball::Ball::new(Circle::new(screen_width()/2.,screen_height()/2.,BALL_RADIUS));
        }else if ball.circle.x > screen_width() {
            //player wins round

            //TODO change file
            ball = ball::Ball::new(Circle::new(screen_width()/2.,screen_height()/2.,BALL_RADIUS));
        }

        next_frame().await
    }
}

fn get_random_file() -> std::path::PathBuf {
    fs::random_file("/bin").unwrap()
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