use crate::ray::Ray;
use crate::vec3::{Vec3, dot};

struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
    front_facing: bool,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_facing = dot(r.direction, outward_normal) < 0.0;
        self.normal = if self.front_facing { outward_normal } else { -1.0*outward_normal };
    }
}

trait Hittable {
   fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

struct Sphere {
    center: Vec3,
    radius: f64,
}
impl Sphere {
    fn new(center: Vec3, radius: f64) -> Self {
        Self {center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_square();
        let half_b = dot(oc, r.direction);
        let c = oc.length_square() - self.radius * self.radius;
        let discriminant = half_b * half_b - a*c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            for difference in &[-root, root] {
                let intersect = (-half_b + difference) / a;
                if (t_min..t_max).contains(&intersect) {
                    rec.t = intersect;
                    rec.p = r.at(rec.t);
                    rec.set_face_normal(r, (rec.p - self.center) / self.radius);
                    return true;
                }
            }
        }
        false
    }
}