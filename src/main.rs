use raylib::consts::KeyboardKey;
use raylib::prelude::*;

mod welcome_animation;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .resizable()
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);

    let mut x: i32 = 0;
    let mut q: f32 = 0.0;

    rl.gui_enable();

    rl = welcome_animation::play(rl, &thread);

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_D) {
            x += q as i32;
        }

        if rl.is_key_down(KeyboardKey::KEY_A) {
            x -= q as i32;
        }

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        d.gui_slider(
            Rectangle::new(200.0, 200.0, 200.0, 20.0),
            Some(rstr!("text")),
            Some(rstr!("text2")),
            &mut q,
            0.0,
            200.0,
        );

        d.draw_text("Hello, world!", x, 12, 20, Color::BLACK);

        d.draw_fps(200, 12);
    }
}
