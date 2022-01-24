use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

pub const BLOCK_SIZE: f64 = 20.;

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let screen_x = to_coord(x);
    let screen_y = to_coord(y);

    rectangle(
        color,
        [screen_x, screen_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g,
    );
}

pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let screen_x = to_coord(x);
    let screen_y = to_coord(y);
    let screen_w = BLOCK_SIZE * (width as f64);
    let screen_h = BLOCK_SIZE * (height as f64);

    rectangle(
        color,
        [screen_x, screen_y, screen_w, screen_h],
        con.transform,
        g,
    );
}
