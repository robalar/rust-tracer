use cgmath::{InnerSpace, Vector3};
use rand::{distributions::uniform::SampleUniform, prelude::Distribution};

pub fn random_vector3<T: SampleUniform>(min: T, max: T) -> Vector3<T> {
    let between = rand::distributions::Uniform::new(min, max);
    let mut rng = rand::thread_rng();

    Vector3 {
        x: between.sample(&mut rng),
        y: between.sample(&mut rng),
        z: between.sample(&mut rng),
    }
}

pub fn random_in_unit_sphere() -> Vector3<f64> {
    loop {
        let p: Vector3<f64> = random_vector3(-1.0, 1.0);
        if p.magnitude2() < 1.0 {
            return p;
        }
    }
}

pub fn random_on_unit_sphere() -> Vector3<f64> {
    random_in_unit_sphere().normalize()
}

pub fn random_in_unit_disk() -> Vector3<f64> {
    let between = rand::distributions::Uniform::new(-1.0, 1.0);
    let mut rng = rand::thread_rng();

    loop {
        let p = Vector3::new(between.sample(&mut rng), between.sample(&mut rng), 0.0);
        if p.magnitude2() < 1.0 {
            return p;
        }
    }
}
