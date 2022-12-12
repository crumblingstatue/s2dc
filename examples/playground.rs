use macroquad::prelude::*;
use s2dc as c;

#[macroquad::main("s2dc playground")]
async fn main() {
    let mut solids = Vec::new();
    let mut start_point = None;
    let mut player = c::MobileEntity::from_pos_and_bb(c::vec2(100, 100), c::vec2(16, 16));
    let mut vspeed = 0.0;
    let gravity = 1.4;
    let mut platformer_mode = false;
    let mut solid_collision = true;
    loop {
        if is_key_pressed(KeyCode::Tab) {
            platformer_mode = !platformer_mode;
        }
        if is_key_pressed(KeyCode::C) {
            solid_collision ^= true;
        }
        let speed = if is_key_down(KeyCode::LeftShift) {
            1.
        } else {
            17.62
        };
        let collider = |en: &c::Entity, offs| {
            for solid in &solids {
                if en.would_collide(solid, offs) {
                    return true;
                }
            }
            false
        };
        if is_key_down(KeyCode::Left) {
            player.move_x(-speed, collider);
        } else if is_key_down(KeyCode::Right) {
            player.move_x(speed, collider);
        }
        if is_key_down(KeyCode::Up) {
            player.move_y(-speed, collider);
        } else if is_key_down(KeyCode::Down) {
            player.move_y(speed, collider);
        }
        if platformer_mode {
            vspeed += gravity;
            if is_key_pressed(KeyCode::Space) {
                vspeed = -16.0;
            }
            if !player.move_y(vspeed, collider) {
                // Reset vertical momentum so it doesn't accumulate while we're on the ground
                vspeed = 0.0;
            }
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let mp = mouse_position();
            start_point = Some(mp);
        }
        if let Some(point) = start_point {
            let mp = mouse_position();
            let en = c::Entity::from_rect_corners(
                point.0 as i32,
                point.1 as i32,
                mp.0 as i32,
                mp.1 as i32,
            );
            let (x, y, w, h) = en.xywh();
            draw_rectangle_lines(x as f32, y as f32, w as f32, h as f32, 2.0, RED);
            if is_mouse_button_released(MouseButton::Left) {
                solids.push(en);
                start_point = None;
            }
            draw_rectangle(point.0, point.1, 1., 1., MAGENTA);
            draw_line(
                en.pos.x as f32,
                en.pos.y as f32,
                en.pos.x as f32 + en.bb.x as f32,
                en.pos.y as f32 + en.bb.y as f32,
                1.0,
                GREEN,
            );
            draw_rectangle(en.pos.x as f32, en.pos.y as f32, 1., 1., YELLOW);
        }
        let mut player_color = WHITE;
        for solid in &solids {
            let (x, y, w, h) = solid.xywh();
            let mut c2 = BROWN;
            if player.en.would_collide(solid, c::vec2(0, 0)) {
                player_color = RED;
                c2 = YELLOW;
            }
            draw_rectangle(x as f32, y as f32, w as f32, h as f32, c2);
        }
        let (x, y, w, h) = player.en.xywh();
        draw_rectangle(x as f32, y as f32, w as f32, h as f32, player_color);
        draw_rectangle(
            player.en.pos.x as f32 - 1.,
            player.en.pos.y as f32 - 1.,
            2.,
            2.,
            MAGENTA,
        );
        draw_text(
            &format!(
                "Platformer mode: {} (tab) | solid collision: {} (c)",
                if platformer_mode {
                    "space to jump"
                } else {
                    "off"
                },
                if solid_collision {
                    "on"
                } else {
                    "off (not implemented)"
                }
            ),
            0.0,
            24.0,
            24.0,
            WHITE,
        );
        next_frame().await
    }
}
