use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T> Rect<T> {
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Self { x, y, w, h }
    }
}

fn minmax<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

impl<T: Add<T, Output = T> + Sub<T, Output = T> + Copy + PartialOrd> Rect<T> {
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let self_right = self.x + self.w;
        let other_right = other.x + other.w;
        let self_bottom = self.y + self.h;
        let other_bottom = other.y + other.h;
        let (_, bigger_x) = minmax(self.x, other.x);
        let (_, bigger_y) = minmax(self.y, other.y);
        let (smaller_right, _) = minmax(self_right, other_right);
        let (smaller_bottom, _) = minmax(self_bottom, other_bottom);
        let x = bigger_x;
        let y = bigger_y;
        let w = smaller_right - bigger_x;
        let h = smaller_bottom - bigger_y;
        if x < smaller_right && y < smaller_bottom {
            Some(Rect { x, y, w, h })
        } else {
            None
        }
    }
}

#[test]
fn test_intersection() {
    use assert_matches::assert_matches;
    let rect1 = Rect::new(12, 31, 20, 21);
    let rect2 = Rect::new(19, 42, 37, 31);
    let rect3 = Rect::new(41, 30, 34, 55);
    assert_matches!(
        rect1.intersection(&rect2),
        Some(Rect {
            x: 19,
            y: 42,
            w: 13,
            h: 10
        })
    );
    assert_matches!(
        rect2.intersection(&rect1),
        Some(Rect {
            x: 19,
            y: 42,
            w: 13,
            h: 10
        })
    );
    assert_matches!(
        rect2.intersection(&rect3),
        Some(Rect {
            x: 41,
            y: 42,
            w: 15,
            h: 31
        })
    );
    assert_matches!(
        rect3.intersection(&rect2),
        Some(Rect {
            x: 41,
            y: 42,
            w: 15,
            h: 31
        })
    );
    assert_matches!(rect1.intersection(&rect3), None);
    assert_matches!(rect3.intersection(&rect1), None);
}
