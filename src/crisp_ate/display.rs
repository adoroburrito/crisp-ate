use raylib::prelude::*;

const ROWS: i32 = 32;
const COLUMNS: i32 = 64;

const SCALING: i32 = 10;

pub fn create_display() -> (RaylibHandle, RaylibThread) {
    raylib::init()
        .size((COLUMNS * SCALING).into(), (ROWS * SCALING).into())
        .title("CrispAte")
        .build()
}

pub fn draw_frame(screen_state: [bool; 64 * 32], mut d: RaylibDrawHandle) {
    d.clear_background(Color::BLACK);

    let mut row = 0;
    let mut col = 0;

    for pixel in screen_state {
        let target_col = col * SCALING;
        let target_row = row * SCALING;
        let color = match pixel {
            true => Color::WHITE,
            false => Color::DARKGRAY,
        };

        d.draw_rectangle(target_col, target_row, SCALING, SCALING, color);

        col += 1;

        if col == COLUMNS {
            col = 0;
            row += 1;
        }
    }
}
