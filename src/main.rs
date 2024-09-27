use benchide::App;
use macroquad::prelude::*;
#[macroquad::main("benchide")]
async fn main() {
    let mut app = App::default();

    let font = load_ttf_font("./JetBrainsMono-Regular.ttf").await.unwrap();

    app.font = Some(font);

    loop {
        let messages = app.view();

        for message in messages {
            app.update(message);
        }

        next_frame().await
    }
}