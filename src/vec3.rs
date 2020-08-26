use std::fmt;
use std::fmt::Formatter;

#[derive(PartialOrd, PartialEq, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Self {x, y, z}
    }

    pub fn length_square(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_square().sqrt()
    }
    pub fn unit(&self) -> Vec3 {
        self / self.length()
    }
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z)});
impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z)});
impl_op_ex!(* |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z)});
impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x / b.x, a.y / b.y, a.z / b.z)});

impl_op_ex_commutative!(+ |a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x + b, a.y + b, a.z + b)});
impl_op_ex_commutative!(- |a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x - b, a.y - b, a.z - b)});
impl_op_ex_commutative!(* |a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x * b, a.y * b, a.z * b)});
impl_op_ex_commutative!(/ |a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x / b, a.y / b, a.z / b)});

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", (255.999 * self.x) as i32, (255.999 * self.y) as i32, (255.999 * self.z) as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;

    #[test]
    fn addition() {
        assert_eq!(&Vec3::new(1.0, 2.0, 3.0) + &Vec3::new(4.0, 5.0, 6.0), Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn addition_floats() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) / 2.0, Vec3::new(0.5, 1.0, 1.5));
    }
}