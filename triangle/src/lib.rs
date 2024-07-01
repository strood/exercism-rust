pub struct Triangle<T> {
    sides: [T; 3],
}

pub trait Positive {
    fn is_positive(&self) -> bool;
}

impl Positive for i32 {
    fn is_positive(&self) -> bool {
        *self > 0
    }
}

impl Positive for f32 {
    fn is_positive(&self) -> bool {
        self.is_sign_positive() && *self != 0.0
    }
}

impl<T: Copy + Positive + PartialEq + PartialOrd + std::ops::Add<Output = T>> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if Self::check_sides(&sides) {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    fn check_sides(sides: &[T; 3]) -> bool {
        sides.iter().all(|&side| side.is_positive())
            && sides[0] + sides[1] > sides[2]
            && sides[1] + sides[2] > sides[0]
            && sides[2] + sides[0] > sides[1]
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[2] == self.sides[0]
    }
}
