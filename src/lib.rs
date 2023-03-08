use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32
}



impl Add<f32> for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: f32) -> Vec2D {
        Vec2D {
            x: self.x + rhs,
            y: self.y + rhs
        }
    }
}

impl Sub<f32> for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: f32) -> Vec2D {
        Vec2D {
            x: self.x - rhs,
            y: self.y - rhs
        }
    }
}

impl Mul<f32> for Vec2D {
    type Output = Vec2D;

    fn mul(self, scalar: f32) -> Vec2D {
        Vec2D { x: self.x * scalar, y: self.y * scalar }
    }
}

impl Div<f32> for Vec2D {
    type Output = Vec2D;

    fn div(self, scalar: f32) -> Vec2D {
        Vec2D { x: self.x / scalar, y: self.y / scalar }
    }
}


impl AddAssign<f32> for Vec2D {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl SubAssign<f32> for Vec2D {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl MulAssign<f32> for Vec2D {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign<f32> for Vec2D {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}


impl Add<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn add(self, other: Vec2D) -> Vec2D {
        Vec2D { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn sub(self, other: Vec2D) -> Vec2D {
        Vec2D { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn mul(self, other: Vec2D) -> Vec2D {
        Vec2D { x: self.x * other.x, y: self.y * other.y }
    }
}

impl Div<Vec2D> for Vec2D {
    type Output = Vec2D;

    fn div(self, other: Vec2D) -> Vec2D {
        Vec2D { x: self.x / other.x, y: self.y / other.y }
    }
}


impl AddAssign<Vec2D> for Vec2D {
    fn add_assign(&mut self, rhs: Vec2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign<Vec2D> for Vec2D {
    fn sub_assign(&mut self, rhs: Vec2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl MulAssign<Vec2D> for Vec2D {
    fn mul_assign(&mut self, rhs: Vec2D) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl DivAssign<Vec2D> for Vec2D {
    fn div_assign(&mut self, rhs: Vec2D) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}



impl Vec2D {
    pub fn new(x: f32, y: f32) -> Vec2D {
        Vec2D { x, y }
    }

    pub fn zero() -> Vec2D {
        Vec2D::new(0.0, 0.0)
    }

    pub fn dot(&self, other: Vec2D) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: Vec2D) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vec2D {
        let mag = self.mag();
        Vec2D::new(self.x / mag, self.y / mag)
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
}
