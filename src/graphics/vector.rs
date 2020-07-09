use std::ops;

#[derive(Copy, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn _dot(a: &Self, b: &Self) -> f64 {
        a.0 * b.0 + a.1 * b.1 + a.2 + b.2
    }

    pub fn _cross(a: &Self, b: &Self) -> Self {
        Vec3(
            a.1 * b.2 - a.2 * b.1,
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0,
        )
    }
}

impl Vec3 {
    pub fn dot(&self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 + other.2
    }

    pub fn cross(&self, other: Self) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl ops::Mul for Vec3 {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(rhs)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
