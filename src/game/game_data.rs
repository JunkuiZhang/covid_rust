use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct EntityStatus {
    pub is_alive: bool,
    pub is_aware: Awareness,
    pub is_infected: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Awareness {
    Aware(f64),
    NotAware(f64),
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        return Vector { x, y };
    }

    fn vector_mod(&self) -> f64 {
        return (self.x.powi(2) + self.y.powi(2)).powf(0.5);
    }

    pub fn normalize(&mut self) {
        let vec_mod = self.vector_mod();
        self.x /= vec_mod;
        self.y /= vec_mod;
    }

    pub fn get_nums(&self) -> (i16, i16) {
        let num_x = self.x.round() as i16;
        let num_y = self.y.round() as i16;
        return (num_x, num_y);
    }

    pub fn distance_with(&self, vec: &Vector) -> f64 {
        return ((self.x - vec.x).powi(2) + (self.y - vec.y).powi(2)).powf(0.5);
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

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        return Vector::new(self.x - rhs.x, self.y - rhs.y);
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vector::new(self.x * rhs, self.y * rhs);
    }
}
