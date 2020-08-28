use crate::ray::Ray;
use crate::vec3::{Vec3, dot};

pub struct HitData {
    p: Vec3,
    pub normal: Vec3,
    t: f64,
    front_facing: bool,
}

impl HitData {
    fn new(t: f64, r: &Ray, p: Vec3, outward_normal: Vec3) -> HitData {
        let front_facing = dot(r.direction, outward_normal) < 0.0;
        let normal = if front_facing { outward_normal } else { -1.0*outward_normal };
        Self {p, normal, t, front_facing}
    }
}

pub trait Hittable {
   fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitData>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitData> {
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
                    let p = r.at(intersect);
                    return Some(HitData::new(intersect, r, p, (p - self.center) / self.radius));
                }
            }
        }
        None
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self {objects: Vec::new()}
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitData> {
        let mut last_hit = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_data) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_data.t;
                last_hit = Some(hit_data);
            }
        }
        last_hit
    }
}