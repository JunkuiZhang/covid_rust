use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        return Vector { x, y };
    }

    fn vector_mod(&self) -> f64 {
        return (self.x.powf(2.0) + self.y.powf(2.0)).powf(0.5);
    }

    pub fn normalize(&mut self) {
        let vec_mod = self.vector_mod();
        self.x /= vec_mod;
        self.y /= vec_mod;
    }

    pub fn set_direction_length(&mut self, target_num: f64) {
        let vec_mod = self.vector_mod();
        self.x = self.x / vec_mod * target_num;
        self.y = self.y / vec_mod * target_num;
    }

    pub fn get_nums(&self) -> (i16, i16) {
        let num_x = self.x.round() as i16;
        let num_y = self.y.round() as i16;
        return (num_x, num_y);
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        return Vector::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vector::new(self.x * rhs, self.y * rhs);
    }
}
