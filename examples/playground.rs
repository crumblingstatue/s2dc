use c::{collision, pos_and_bb_centered, pos_bb_xywh, Entity};
use macroquad::prelude::*;
use s2dc as c;

#[macroquad::main("s2dc playground")]
async fn main() {
    let mut solids = Vec::new();
    let mut start_point = None;
    let mut player = c::Entity {
        pos: c::vec2(100, 100),
        bb: c::vec2(16, 16),
    };
    loop {
        let speed = if is_key_down(KeyCode::LeftShift) {
            1.
        } else {
            17.62
        };
        let mut accum = 0.0;
        if is_key_down(KeyCode::Left) {
            c::move_x(&mut player, -speed, &mut accum, &solids);
        } else if is_key_down(KeyCode::Right) {
            c::move_x(&mut player, speed, &mut accum, &solids);
        }
        if is_key_down(KeyCode::Up) {
            c::move_y(&mut player, -speed, &mut accum, &solids);
        } else if is_key_down(KeyCode::Down) {
            c::move_y(&mut player, speed, &mut accum, &solids);
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let mp = mouse_position();
            start_point = Some(mp);
        }
        if let Some(point) = start_point {
            let mp = mouse_position();
            let (pos, bb) =
                pos_and_bb_centered(point.0 as i32, point.1 as i32, mp.0 as i32, mp.1 as i32);
            let (x, y, w, h) = pos_bb_xywh(pos, bb);
            draw_rectangle_lines(x as f32, y as f32, w as f32, h as f32, 1.0, RED);
            if is_mouse_button_released(MouseButton::Left) {
                solids.push(Entity { pos, bb });
                start_point = None;
            }
            draw_rectangle(point.0, point.1, 1., 1., MAGENTA);
            draw_line(
                pos.x as f32,
                pos.y as f32,
                pos.x as f32 + bb.x as f32,
                pos.y as f32 + bb.y as f32,
                1.0,
                GREEN,
            );
            draw_rectangle(pos.x as f32, pos.y as f32, 1., 1., YELLOW);
        }
        let mut player_color = WHITE;
        for solid in &solids {
            let (x, y, w, h) = pos_bb_xywh(solid.pos, solid.bb);
            let mut c2 = BROWN;
            if collision(&player, solid, c::vec2(0, 0)) {
                player_color = RED;
                c2 = YELLOW;
            }
            draw_rectangle(x as f32, y as f32, w as f32, h as f32, c2);
        }
        let (x, y, w, h) = pos_bb_xywh(player.pos, player.bb);
        draw_rectangle(x as f32, y as f32, w as f32, h as f32, player_color);
        draw_rectangle(
            player.pos.x as f32 - 1.,
            player.pos.y as f32 - 1.,
            2.,
            2.,
            MAGENTA,
        );
        next_frame().await
    }
}
