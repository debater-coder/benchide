use benchide::App;
use macroquad::prelude::*;

#[macroquad::main("benchide")]
async fn main() {
    let mut state = App::default();

    loop {
        let messages = state.view();

        for message in messages {
            state.update(message);
        }

        next_frame().await
    }
}