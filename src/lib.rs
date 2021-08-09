//! Simple 2d solid collision library.
//!
//! Based on https://maddythorson.medium.com/celeste-and-towerfall-physics-d24bd2ae0fc5

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

pub fn collision(en1: &Entity, en2: &Entity, en1_offset: Vec2) -> bool {
    let x = en1.pos.x + en1_offset.x;
    let y = en1.pos.y + en1_offset.y;
    x + en1.bb.x > en2.pos.x - en2.bb.x
        && x - en1.bb.x < en2.pos.x + en2.bb.x
        && y + en1.bb.y > en2.pos.y - en2.bb.y
        && y - en1.bb.y < en2.pos.y + en2.bb.y
}

pub struct Entity {
    pub pos: Vec2,
    pub bb: Vec2,
}

pub fn move_x<F>(entity: &mut Entity, amount: f32, accum: &mut f32, mut collide_at: F) -> bool
where
    F: FnMut(&Entity, Vec2) -> bool,
{
    *accum += amount;
    let mut move_amount = accum.round() as i32;
    if move_amount != 0 {
        *accum -= move_amount as f32;
        let sign = move_amount.signum();
        while move_amount != 0 {
            if !collide_at(entity, vec2(sign, 0)) {
                //There is no Solid immediately beside us
                entity.pos.x += sign;
                move_amount -= sign;
            } else {
                //Hit a solid!
                return true;
            }
        }
    }
    false
}

pub fn move_y<F>(entity: &mut Entity, amount: f32, accum: &mut f32, mut collide_at: F) -> bool
where
    F: FnMut(&Entity, Vec2) -> bool,
{
    *accum += amount;
    let mut move_amount = accum.round() as i32;
    if move_amount != 0 {
        *accum -= move_amount as f32;
        let sign = move_amount.signum();
        while move_amount != 0 {
            if !collide_at(entity, vec2(0, sign)) {
                //There is no Solid immediately beside us
                entity.pos.y += sign;
                move_amount -= sign;
            } else {
                //Hit a solid!
                return true;
            }
        }
    }
    false
}
