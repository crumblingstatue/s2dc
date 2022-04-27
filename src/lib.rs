//! Simple 2d solid collision library.
//!
//! Based on <https://maddythorson.medium.com/celeste-and-towerfall-physics-d24bd2ae0fc5>

#![warn(missing_docs)]

type Unit = i32;

/// A 2 dimensional mathematical vector (x, y)
#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    /// x component (horizontal)
    pub x: Unit,
    /// y component (vertical)
    pub y: Unit,
}

/// Convenience function to create a Vec2 from (x, y)
pub fn vec2(x: Unit, y: Unit) -> Vec2 {
    Vec2 { x, y }
}

fn center(v1: Unit, v2: Unit) -> Unit {
    v1 + ((v2 - v1) / 2)
}

fn halfextent(v1: Unit, v2: Unit) -> Unit {
    (v1.max(v2) - v1.min(v2)).abs() / 2
}

/// 2D collision entity.
///
/// It has a 2d position, and a 2d bounding box that is centered on the position.
#[derive(Clone, Copy, Debug)]
pub struct Entity {
    /// Position. This is the center point of the entity.
    pub pos: Vec2,
    /// Bounding box, centered on `pos`,
    pub bb: Vec2,
}

impl Entity {
    /// Creates an entity from the 4 corners of a rectangle
    pub fn from_rect_corners(x1: Unit, y1: Unit, x2: Unit, y2: Unit) -> Self {
        let cx = center(x1, x2);
        let cy = center(y1, y2);
        let hhe = halfextent(x1, x2);
        let vhe = halfextent(y1, y2);
        Self {
            pos: vec2(cx, cy),
            bb: vec2(hhe, vhe),
        }
    }
    /// Creates an entity from its component position and bounding box
    pub fn from_pos_and_bb(pos: Vec2, bb: Vec2) -> Self {
        Self { pos, bb }
    }
    /// Returns the (x, y, width, height) of the rectangle of this entity
    pub fn xywh(&self) -> (Unit, Unit, Unit, Unit) {
        (
            self.pos.x - self.bb.x,
            self.pos.y - self.bb.y,
            self.bb.x * 2,
            self.bb.y * 2,
        )
    }
    /// Returns whether this entity would collide with another if its position was offset by `offset`
    pub fn would_collide(&self, other: &Entity, offset: Vec2) -> bool {
        let x = self.pos.x + offset.x;
        let y = self.pos.y + offset.y;
        x + self.bb.x > other.pos.x - other.bb.x
            && x - self.bb.x < other.pos.x + other.bb.x
            && y + self.bb.y > other.pos.y - other.bb.y
            && y - self.bb.y < other.pos.y + other.bb.y
    }
}

/// An [`Entity`] that can move.
pub struct MobileEntity {
    /// The [`Entity`] component of this `MobileEntity`
    pub en: Entity,
    accum_x: f32,
    accum_y: f32,
}

impl MobileEntity {
    /// Creates a mobile entity from a position and bounding box
    pub fn from_pos_and_bb(pos: Vec2, bb: Vec2) -> Self {
        Self {
            en: Entity::from_pos_and_bb(pos, bb),
            accum_x: 0.0,
            accum_y: 0.0,
        }
    }
    /// Move a set amount horizontally, taking collisions into account.
    ///
    /// The `would_collide` function is a function that takes this entity and an offset
    /// as parameters, and returns whether the entity would collide if offset.
    ///
    /// The usual way to do this is to keep a list of `Entity` that are the solids, and
    /// go through each and call `entity.would_collide(solid, offset)` for each.
    ///
    /// Returns `true` if the move was unobstructed, `false` if there was a collision.
    pub fn move_x<F>(&mut self, amount: f32, mut would_collide: F) -> bool
    where
        F: FnMut(&Entity, Vec2) -> bool,
    {
        self.accum_x += amount;
        let mut move_amount = self.accum_x.round() as i32;
        if move_amount != 0 {
            self.accum_x -= move_amount as f32;
            let sign = move_amount.signum();
            while move_amount != 0 {
                if would_collide(&self.en, vec2(sign, 0)) {
                    return false;
                } else {
                    self.en.pos.x += sign;
                    move_amount -= sign;
                }
            }
        }
        true
    }
    /// Move a set amount vertically, taking collisions into account.
    ///
    /// The `would_collide` function is a function that takes this entity and an offset
    /// as parameters, and returns whether the entity would collide if offset.
    ///
    /// The usual way to do this is to keep a list of `Entity` that are the solids, and
    /// go through each and call `entity.would_collide(solid, offset)` for each.
    ///
    /// Returns `true` if the move was unobstructed, `false` if there was a collision.
    pub fn move_y<F>(&mut self, amount: f32, mut would_collide: F) -> bool
    where
        F: FnMut(&Entity, Vec2) -> bool,
    {
        self.accum_y += amount;
        let mut move_amount = self.accum_y.round() as i32;
        if move_amount != 0 {
            self.accum_y -= move_amount as f32;
            let sign = move_amount.signum();
            while move_amount != 0 {
                if would_collide(&self.en, vec2(0, sign)) {
                    return false;
                } else {
                    self.en.pos.y += sign;
                    move_amount -= sign;
                }
            }
        }
        true
    }
}
