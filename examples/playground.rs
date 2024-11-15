use egui_macroquad::egui;
use egui_macroquad::macroquad;
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
    let mut solid_highlight_index = None;
    loop {
        clear_background(BLACK);
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
        if player.en.pos.y > screen_height() as i32 {
            player.en.pos.y = 0;
        }
        vspeed = vspeed.clamp(-96., 96.);
        let mut egui_wants_ptr = false;
        egui_macroquad::cfg(|ctx| {
            egui_wants_ptr = ctx.wants_pointer_input();
        });
        if is_mouse_button_pressed(MouseButton::Left) && !egui_wants_ptr {
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
        for (i, solid) in solids.iter().enumerate() {
            let (x, y, w, h) = solid.xywh();
            let mut c2 = BROWN;
            if player.en.collides(solid) {
                player_color = RED;
                c2 = YELLOW;
            }
            if solid_highlight_index == Some(i) {
                c2 = PURPLE;
            }
            draw_rectangle(x as f32, y as f32, w as f32, h as f32, c2);
        }
        egui_macroquad::ui(|ctx| {
            egui::Window::new("Playground").show(ctx, |ui| {
                ui.heading("Player");
                ui.add(egui::DragValue::new(&mut player.en.pos.x).prefix("x "));
                ui.add(egui::DragValue::new(&mut player.en.pos.y).prefix("y "));
                ui.heading("Level");
                let mut idx = 0;
                let mut any_hov = false;
                egui::Grid::new("solids_grid").show(ui, |ui| {
                    ui.label("pos.x");
                    ui.label("pos.y");
                    ui.label("bb.x");
                    ui.label("bb.y");
                    ui.label("Delete");
                    ui.end_row();
                    solids.retain_mut(|solid| {
                        let mut retain = true;
                        let mut hov = false;
                        hov |= ui.add(egui::DragValue::new(&mut solid.pos.x)).hovered();
                        hov |= ui.add(egui::DragValue::new(&mut solid.pos.y)).hovered();
                        hov |= ui.add(egui::DragValue::new(&mut solid.bb.x)).hovered();
                        hov |= ui.add(egui::DragValue::new(&mut solid.bb.y)).hovered();
                        let re = ui.button("x");
                        hov |= re.hovered();
                        if re.clicked() {
                            retain = false;
                        }
                        if hov {
                            solid_highlight_index = Some(idx);
                            any_hov = true;
                        }
                        idx += 1;
                        ui.end_row();
                        retain
                    });
                });
                if !any_hov {
                    solid_highlight_index = None;
                }
                if ui.button("Clear").clicked() {
                    solids.clear();
                }
            });
        });
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
        egui_macroquad::draw();
        next_frame().await
    }
}
