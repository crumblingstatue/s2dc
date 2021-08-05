type Unit = i32;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: Unit,
    pub y: Unit,
}

pub fn vec2(x: Unit, y: Unit) -> Vec2 {
    Vec2 { x, y }
}

fn center(v1: Unit, v2: Unit) -> Unit {
    v1 + ((v2 - v1) / 2)
}

fn halfextent(v1: Unit, v2: Unit) -> Unit {
    (v1.max(v2) - v1.min(v2)).abs() / 2
}

pub fn pos_and_bb_centered(x1: Unit, y1: Unit, x2: Unit, y2: Unit) -> (Vec2, Vec2) {
    let cx = center(x1, x2);
    let cy = center(y1, y2);
    let hhe = halfextent(x1, x2);
    let vhe = halfextent(y1, y2);
    (vec2(cx, cy), vec2(hhe, vhe))
}

pub fn pos_bb_xywh(pos: Vec2, bb: Vec2) -> (Unit, Unit, Unit, Unit) {
    (pos.x - bb.x, pos.y - bb.y, bb.x * 2, bb.y * 2)
}

pub fn collision(p1: Vec2, bb1: Vec2, p2: Vec2, bb2: Vec2) -> bool {
    p1.x + bb1.x > p2.x - bb2.x
        && p1.x - bb1.x < p2.x + bb2.x
        && p1.y + bb1.y > p2.y - bb2.y
        && p1.y - bb1.y < p2.y + bb2.y
}
