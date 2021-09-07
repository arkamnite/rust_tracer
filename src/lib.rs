use std::ops::*;

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn length_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    pub fn add(&self, f: f64) -> Vec3 {
        Vec3 {
            x: self.x + f,
            y: self.y + f,
            z: self.z + f,
        }
    }

    pub fn sub(&self, f: f64) -> Vec3 {
        self.add(f * -1.0)
    }

    pub fn div(&self, f: f64) -> Vec3 {
        self.mul(1.0/f)
    }

    pub fn mul(&self, f: f64) -> Vec3 {
        Vec3 {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }

    pub fn cross(&self, o: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }

    pub fn dot(&self, o: &Vec3) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn unit_vector(&self) -> Vec3 {
        self.div(self.length())
    }
}

// Add a vector
impl Add for Vec3 {
    type Output = Self;

    fn add(self, f:Self) -> Self {
        Self {
            x: self.x + f.x,
            y: self.y + f.y,
            z: self.z + f.z,
        }
    }
}

// Subtract a float to all values
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, f:Self) -> Self {
        Self {
            x: self.x - f.x,
            y: self.y - f.y,
            z: self.z - f.z,
        }
    }
}

// Multiply by a float

// Divide by a float



