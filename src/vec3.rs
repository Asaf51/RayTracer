use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

// Those things are the same, but with a different name
// Color is RGB
pub type Color = Vector3;
pub type Point3 = Vector3;

impl Default for Vector3 {
    fn default() -> Vector3 {
        Vector3 {x: 0_f64, y: 0_f64, z: 0_f64}
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl ops::Div<f64> for &Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }
}

impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {x, y, z}
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powf(2_f64) + self.y.powf(2_f64) + self.z.powf(2_f64)
    }
}

pub fn unit_vector(vector: &Vector3) -> Vector3 {
    vector / vector.length()
}

pub fn dot_product(u: &Vector3, v: &Vector3) -> f64 {
    return (u.x * v.x) + (u.y * v.y) + (u.z * v.z);
}