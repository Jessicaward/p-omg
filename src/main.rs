mod paddle;

use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    //Left paddle
    let player_paddle = paddle::Paddle::new(Rect::new(screen_width() / 5.0, screen_height() / 5.0, paddle::PADDLE_WIDTH, paddle::PADDLE_HEIGHT));

    //Right paddle
    let ai_paddle = paddle::Paddle::new(Rect::new((screen_width() / 5.0) * 4.0, screen_height() / 5.0 , paddle::PADDLE_WIDTH, paddle::PADDLE_HEIGHT));

    loop {
        clear_background(BLACK);
        player_paddle.draw();
        ai_paddle.draw();
        next_frame().await
    }
}