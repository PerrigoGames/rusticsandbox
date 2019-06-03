use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

const BLOCK_SIZE: f64 = 25.0;

pub fn to_coord(gm_coord: i32) -> f64 {
    (gm_coord as f64) * BLOCK_SIZE
}

pub fn draw_block(clr: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let g_x = to_coord(x);
    let g_y = to_coord(y);

    rectangle(clr, [g_x, g_y, BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
}

pub fn draw_rect(clr: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let g_x = to_coord(x);
    let g_y = to_coord(y);

    rectangle(
        clr,
        [
            g_x,
            g_y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}
