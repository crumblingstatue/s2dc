use macroquad::prelude::*;
use s2dc::Rect;

#[macroquad::main("s2dc playground")]
async fn main() {
    let mut player_rect = Rect::new(100., 100., 32., 32.);
    let solids = [Rect::new(200., 200., 64., 64.)];
    loop {
        let speed = 7.3;
        if is_key_down(KeyCode::Left) {
            player_rect.x -= speed;
        } else if is_key_down(KeyCode::Right) {
            player_rect.x += speed;
        }
        if is_key_down(KeyCode::Up) {
            player_rect.y -= speed;
        } else if is_key_down(KeyCode::Down) {
            player_rect.y += speed;
        }
        draw_rectangle(
            player_rect.x,
            player_rect.y,
            player_rect.w,
            player_rect.h,
            WHITE,
        );
        for solid in &solids {
            draw_rectangle(solid.x, solid.y, solid.w, solid.h, BROWN);
            if let Some(intersect) = player_rect.intersection(solid) {
                draw_rectangle(intersect.x, intersect.y, intersect.w, intersect.h, YELLOW);
            }
        }
        next_frame().await
    }
}
