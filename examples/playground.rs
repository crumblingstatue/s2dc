use macroquad::prelude::*;
use s2dc::Rect;

enum Control {
    Orig,
    Proj,
}

impl Control {
    fn name(&self) -> &str {
        match *self {
            Self::Orig => "original",
            Self::Proj => "projected",
        }
    }
}

#[macroquad::main("s2dc playground")]
async fn main() {
    let mut orig_rect = Rect::new(100., 100., 32., 32.).unwrap();
    let mut proj_rect = orig_rect;
    let mut ctrl = Control::Orig;
    let mut solids = Vec::new();
    let mut start_point = None;
    loop {
        let speed = if is_key_down(KeyCode::LeftShift) {
            1.0
        } else {
            8.0
        };
        let controlled = match ctrl {
            Control::Orig => &mut orig_rect,
            Control::Proj => &mut proj_rect,
        };
        if is_key_pressed(KeyCode::Enter) {
            ctrl = match ctrl {
                Control::Orig => Control::Proj,
                Control::Proj => Control::Orig,
            };
        }
        if is_key_down(KeyCode::Left) {
            *controlled.x_mut() -= speed;
        } else if is_key_down(KeyCode::Right) {
            *controlled.x_mut() += speed;
        }
        if is_key_down(KeyCode::Up) {
            *controlled.y_mut() -= speed;
        } else if is_key_down(KeyCode::Down) {
            *controlled.y_mut() += speed;
        }
        if is_mouse_button_pressed(MouseButton::Left) {
            let mp = mouse_position();
            start_point = Some(mp);
        }
        if let Some(point) = start_point {
            let mp = mouse_position();
            if let Some(rect) = Rect::new(point.0, point.1, mp.0 - point.0, mp.1 - point.1) {
                draw_rectangle_lines(*rect.x(), *rect.y(), *rect.w(), *rect.h(), 1.0, RED);
                if is_mouse_button_released(MouseButton::Left) {
                    solids.push(rect);
                    start_point = None;
                }
            }
        }
        let ctrl_rect = *controlled;
        draw_text(
            &format!("Controlling {} (enter)", ctrl.name()),
            0.,
            32.,
            32.,
            WHITE,
        );
        draw_rectangle(
            *orig_rect.x(),
            *orig_rect.y(),
            *orig_rect.w(),
            *orig_rect.h(),
            WHITE,
        );
        draw_rectangle(
            *proj_rect.x(),
            *proj_rect.y(),
            *proj_rect.w(),
            *proj_rect.h(),
            YELLOW,
        );
        let new = s2dc::solve(&orig_rect, &proj_rect, &solids);
        for solid in &solids {
            draw_rectangle(*solid.x(), *solid.y(), *solid.w(), *solid.h(), BROWN);
            if let Some(intersect) = proj_rect.intersection(solid) {
                draw_rectangle(
                    *intersect.x(),
                    *intersect.y(),
                    *intersect.w(),
                    *intersect.h(),
                    RED,
                );
            }
        }
        let new_x = *new.x();
        let new_y = *new.y();
        draw_line(new_x + 16., new_y - 16., new_x + 16., new_y, 2.0, GREEN);
        draw_line(new_x + 8., new_y - 8., new_x + 16., new_y, 2.0, GREEN);
        draw_line(new_x + 24., new_y - 8., new_x + 16., new_y, 2.0, GREEN);
        draw_rectangle(new_x, new_y, *new.w(), *new.h(), GREEN);
        draw_text("C", *ctrl_rect.x() + 8.0, *ctrl_rect.y() + 24.0, 32.0, BLUE);
        next_frame().await
    }
}
