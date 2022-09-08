use cgmath::{InnerSpace, Vector3};
use rand::Rng;

use crate::{
    colour::Colour,
    ray::Ray,
    vec::{random_in_unit_sphere, random_on_unit_sphere},
};

#[derive(Clone, Copy)]
pub enum Material {
    Lambetarian { albedo: Colour<f64> },
    Metal { albedo: Colour<f64>, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
}

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Colour<f64>,
}

fn almost_zero(vec: Vector3<f64>) -> bool {
    vec.x.abs() < f64::EPSILON && vec.y.abs() < f64::EPSILON && vec.z.abs() < f64::EPSILON
}

fn reflect(incident: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
    incident - 2.0 * incident.dot(normal) * normal
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn refract(incident: Vector3<f64>, normal: Vector3<f64>, etia_over_etat: f64) -> Vector3<f64> {
    let cos_theta = -incident.dot(normal).min(1.0);
    let r_out_perp = etia_over_etat * (incident + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perp.magnitude2()).abs().sqrt() * normal;
    r_out_parallel + r_out_perp
}

impl Material {
    pub fn scatter(self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatteredRay> {
        match self {
            Material::Lambetarian { albedo } => {
                let mut scatter_direction = hit_record.normal + random_on_unit_sphere();

                if almost_zero(scatter_direction) {
                    scatter_direction = hit_record.normal;
                }

                Some(ScatteredRay {
                    ray: Ray::new(hit_record.p, scatter_direction),
                    attenuation: albedo,
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = reflect(ray.direction.normalize(), hit_record.normal);
                Some(ScatteredRay {
                    ray: Ray::new(hit_record.p, reflected + fuzz * random_in_unit_sphere()),
                    attenuation: albedo,
                })
            }
            Material::Dielectric {
                index_of_refraction,
            } => {
                let refraction_ratio = if hit_record.front_face {
                    1.0 / index_of_refraction
                } else {
                    index_of_refraction
                };

                let unit_direction = ray.direction.normalize();
                let cos_theta = -unit_direction.dot(hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract
                    || reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen()
                {
                    reflect(unit_direction, hit_record.normal)
                } else {
                    refract(unit_direction, hit_record.normal, refraction_ratio)
                };

                Some(ScatteredRay {
                    ray: Ray::new(hit_record.p, direction),
                    attenuation: Colour::new(1.0, 1.0, 1.0),
                })
            }
        }
    }
}

pub struct HitRecord {
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    fn new(
        t: f64,
        p: Vector3<f64>,
        outward_normal: Vector3<f64>,
        ray: &Ray,
        material: Material,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;

        HitRecord {
            p,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct World {
    pub shapes: Vec<Sphere>,
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut record: Option<HitRecord> = None;

        for shape in self.shapes.iter() {
            if let Some(hit) = shape.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                record = Some(hit);
            };
        }

        record
    }
}

pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.magnitude2();
        let half_b = oc.dot(ray.direction);
        let c = oc.magnitude2() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let square_root_d = discriminant.sqrt();
            let mut root = (-half_b - square_root_d) / a;
            if root < t_min || root > t_max {
                root = (-half_b + square_root_d) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }

            let p = ray.at(root);
            let outward_normal = (p - self.center) / self.radius;

            Some(HitRecord::new(root, p, outward_normal, ray, self.material))
        }
    }
}
