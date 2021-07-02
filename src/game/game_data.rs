use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        return Vector{x, y};
    }

    pub fn vector_mod(&self) -> f64 {
        return (x.pow(2) + y.pow(2)).pow(0.5);
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        return Vector::new(self.x + rhs.x, self.y + rhs.y);
    }
}
