use conquest_server::Vec2D;
use std::ops::{Add,Sub,Mul,Div,AddAssign,SubAssign,MulAssign,DivAssign};


#[derive(Debug)]
pub struct Health {
    pub amount: f32,
    pub max: f32
}

impl Health {
    pub fn new(amount: f32) -> Self {
        Self {
            amount,
            max: amount
        }
    }
}


impl Add<f32> for Health {
    type Output = Health;

    fn add(self, rhs: f32) -> Health {
        Self {
            amount: (self.amount + rhs).clamp(0.0, self.max),
            max: self.max
        }
    }
}

impl Sub<f32> for Health {
    type Output = Health;

    fn sub(self, rhs: f32) -> Health {
        Self {
            amount: (self.amount - rhs).clamp(0.0, self.max),
            max: self.max
        }
    }
}

impl Div<f32> for Health {
    type Output = Health;

    fn div(self, rhs: f32) -> Health {
        Self {
            amount: (self.amount / rhs).clamp(0.0, self.max),
            max: self.max
        }
    }
}

impl Mul<f32> for Health {
    type Output = Health;

    fn mul(self, rhs: f32) -> Health {
        Self {
            amount: (self.amount * rhs).clamp(0.0, self.max),
            max: self.max
        }
    }
}


impl AddAssign<f32> for Health {
    fn add_assign(&mut self, rhs: f32) {
        self.amount = (self.amount + rhs).clamp(0.0, self.max);
    }
}

impl SubAssign<f32> for Health {
    fn sub_assign(&mut self, rhs: f32) {
        self.amount = (self.amount - rhs).clamp(0.0, self.max);
    }
}

impl DivAssign<f32> for Health {
    fn div_assign(&mut self, rhs: f32) {
        self.amount = (self.amount / rhs).clamp(0.0, self.max);
    }
}

impl MulAssign<f32> for Health {
    fn mul_assign(&mut self, rhs: f32) {
        self.amount = (self.amount * rhs).clamp(0.0, self.max);
    }
}


#[derive(Debug)]
pub struct Body {
    pub position: Vec2D,
    pub velocity: Vec2D,
    pub size: f32,
    pub weight: f32,
    pub color: (f32,f32,f32)
}
