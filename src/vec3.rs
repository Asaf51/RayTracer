use std::ops;
use rand::Rng;

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

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
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
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn new_random_unit() -> Self {
        let mut rng = rand::thread_rng();
        let a: f64 = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let r: f64 = (1.0 - z * z).sqrt();

        Self {
            x: r * a.cos(),
            y: r * a.sin(),
            z
        }
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max)
        }
    }
}

#[inline]
pub fn unit_vector(vector: &Vector3) -> Vector3 {
    vector / vector.length()
}

#[inline]
pub fn dot_product(u: &Vector3, v: &Vector3) -> f64 {
    return (u.x * v.x) + (u.y * v.y) + (u.z * v.z);
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[inline]
pub fn random_in_unit_sphere() -> Vector3 {
    let mut rng = rand::thread_rng();
    let x : f64 = rng.gen_range(-1.0, 1.0);
    let y : f64 = rng.gen_range(-1.0, 1.0);
    let current_size = x * x + y * y;
    let z : f64 = if current_size > 1.0 {
        rng.gen_range(-1.0, 1.0)
    } else {
        let flip : bool = rng.gen();
        let abs_z = rng.gen_range((1.0 - current_size).sqrt(), 1.0);
        if flip { -abs_z } else { abs_z }
    };
    Vector3::new(x, y, z)
}

pub fn cross(u: &Vector3, v: &Vector3) -> Vector3 {
    Vector3::new(
        u.y * v.z - u.z * v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x
    )
}