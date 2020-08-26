use crate::vec3::Vec3;

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Self {origin, direction}
    }
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use crate::ray::Ray;
    use crate::vec3::Vec3;

    #[test]
    fn at() {
        let ray: &Ray = &Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 0.0));
        assert_eq!(ray.at(5.0), Vec3::new(6.0, 6.0, 1.0));
    }
}
