use raylib::prelude::*;

pub fn play(mut rl: RaylibHandle, thread: &RaylibThread) -> RaylibHandle {
    // -- hold & logo fade out
    let mut elapsed_frames = 0;
    let fade_len = 50;
    let hold_len = 75;

    let txt = "By @demo-exe";
    let txt_width = rl.measure_text(txt, 20);
    let logo = Image::load_image("assets/icon.png").unwrap();
    let logo_texture = rl.load_texture_from_image(&thread, &logo).unwrap();

    while !rl.window_should_close() && elapsed_frames < (fade_len + hold_len) {
        let x = (rl.get_screen_width() - txt_width) / 2;
        let y = rl.get_screen_height() / 2;

        let img_x = (rl.get_screen_width() - logo_texture.width()) / 2;
        let img_y = rl.get_screen_height() / 2 - logo_texture.height() - 10;

        let alpha = if elapsed_frames > hold_len {
            1.0 - ((elapsed_frames - hold_len) as f32 / fade_len as f32)
        } else {
            1.0
        };

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text(txt, x, y, 20, Color::BLACK.alpha(alpha));
        d.draw_texture(&logo_texture, img_x, img_y, Color::WHITE.alpha(alpha));

        elapsed_frames += 1;
    }

    // -- name fade-in
    elapsed_frames = 0;
    let fade_len = 50;
    let hold_len = 100;
    let txt = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let txt_width = rl.measure_text(&txt, 20);

    while !rl.window_should_close() && elapsed_frames < (fade_len + hold_len) {
        let x = (rl.get_screen_width() - txt_width) / 2;
        let y = rl.get_screen_height() / 2;

        let alpha = if elapsed_frames < hold_len {
            elapsed_frames as f32 / fade_len as f32
        } else {
            1.0
        };

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text(&txt, x, y, 20, Color::BLACK.alpha(alpha));

        elapsed_frames += 1;
    }

    // -- name move animation
    elapsed_frames = 0;
    let anim_len = 40;

    while !rl.window_should_close() && elapsed_frames < anim_len {
        let from_x = (rl.get_screen_width() - txt_width) / 2;
        let from_y = rl.get_screen_height() / 2;
        let tgt_x = rl.get_screen_width() - txt_width - 5;
        let tgt_y = 5;

        let p: f32 = elapsed_frames as f32 / anim_len as f32;

        let exp = p.powf(3.);

        let x = lerp(from_x as f32, tgt_x as f32, exp) as i32;
        let y = lerp(from_y as f32, tgt_y as f32, exp) as i32;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text(&txt, x, y, 20, Color::BLACK);

        elapsed_frames += 1;
    }

    // -- hold name in corner
    elapsed_frames = 0;
    let hold_len = 30;

    while !rl.window_should_close() && elapsed_frames < hold_len {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        draw_corner_name(&mut d);

        elapsed_frames += 1;
    }

    return rl;
}

pub fn draw_corner_name(d: &mut RaylibDrawHandle) {
    // TODO: memoize
    let txt = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let txt_width = d.measure_text(&txt, 20);

    let tgt_x = d.get_screen_width() - txt_width - 5;
    let tgt_y = 5;
    d.draw_text(&txt, tgt_x, tgt_y, 20, Color::BLACK);
}
