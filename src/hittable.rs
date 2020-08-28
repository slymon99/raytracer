use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub enum Hittable<'a> {
    Sphere(Sphere<'a>),
}

fn hit<'a>(r: &Ray, t_min: f64, t_max: f64, object: &'a Hittable) -> Option<HitData<'a>> {
    match object {
        Hittable::Sphere(s) => {s.hit(r, t_min, t_max)}
    }
}

pub struct HitData<'a> {
    pub normal: Vec3,
    pub p: Vec3,
    pub mat: &'a Material,
    t: f64,
}

impl<'a> HitData<'a> {
    fn new(t: f64, r: Ray, p: Vec3, outward_normal: Vec3, mat: &'a Material) -> HitData {
        let front_facing = dot(r.direction, outward_normal) < 0.0;
        let normal = if front_facing {
            outward_normal
        } else {
            -1.0 * outward_normal
        };
        Self { normal, p, t, mat }
    }
}

pub struct Sphere<'a> {
    center: Vec3,
    radius: f64,
    mat: &'a Material,
}

impl<'a> Sphere<'a>{
    pub fn new(center: Vec3, radius: f64, mat: &'a Material) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitData> {
        let oc = r.origin - self.center;
        let a = r.direction.length_square();
        let half_b = dot(oc, r.direction);
        let c = oc.length_square() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            for difference in &[-root, root] {
                let intersect = (-half_b + difference) / a;
                if (t_min..t_max).contains(&intersect) {
                    let p = r.at(intersect);
                    return Some(HitData::new(
                        intersect,
                        r.clone(),
                        p,
                        (p - self.center) / self.radius,
                        self.mat,
                    ));
                }
            }
        }
        None
    }
}

pub struct HittableList<'a> {
    objects: Vec<Hittable<'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, object: Hittable<'a>) {
        self.objects.push(object);
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitData> {
        let mut last_hit = None;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_data) = hit(r, t_min, closest_so_far, object) {
                closest_so_far = hit_data.t;
                last_hit = Some(hit_data);
            }
        }
        last_hit
    }
}
