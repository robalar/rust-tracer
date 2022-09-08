use std::io::Write;
use std::time::Instant;
use std::{fs::File, io::Error};

use cgmath::{InnerSpace, Vector3};
use colour::Colour;
use rand::Rng;
use ray::Ray;
use shapes::Hittable;

use crate::camera::Camera;
use crate::shapes::{Material, Sphere, World};

mod camera;
mod colour;
mod ray;
mod shapes;
mod vec;

fn ray_colour<T: Hittable>(ray: &Ray, hittable: &T, depth: u32) -> Colour<f64> {
    if depth == 0 {
        Colour::new(0.0, 0.0, 0.0)
    } else if let Some(hit_record) = hittable.hit(ray, 0.001, f64::INFINITY) {
        if let Some(scattered_ray) = hit_record.material.scatter(ray, &hit_record) {
            scattered_ray.attenuation.mul_element_wise(ray_colour(
                &scattered_ray.ray,
                hittable,
                depth - 1,
            ))
        } else {
            Colour::new(1.0, 1.0, 1.0)
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Colour::new(
            (1.0 - t) + t * 0.5,
            (1.0 - t) + t * 0.7,
            (1.0 - t) + t * 1.0,
        )
    }
}

fn random_world() -> World {
    let ground_material = Material::Lambetarian {
        albedo: Colour::new(0.5, 0.5, 0.5),
    };

    let mut shapes = vec![Sphere {
        center: Vector3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }];

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let center = Vector3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vector3::<f64>::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                let choose_mat: f64 = rng.gen();
                let material = if choose_mat < 0.8 {
                    Material::Lambetarian {
                        albedo: Colour::random(),
                    }
                } else if choose_mat < 0.95 {
                    Material::Metal {
                        albedo: Colour::random(),
                        fuzz: rng.gen_range(0.0..0.5),
                    }
                } else {
                    Material::Dielectric {
                        index_of_refraction: 1.5,
                    }
                };
                shapes.push(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }

    shapes.push(Sphere {
        center: Vector3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric {
            index_of_refraction: 1.5,
        },
    });

    shapes.push(Sphere {
        center: Vector3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Lambetarian {
            albedo: Colour::new(0.4, 0.2, 0.1),
        },
    });

    shapes.push(Sphere {
        center: Vector3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal {
            albedo: Colour::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        },
    });

    World { shapes }
}

fn main() -> Result<(), Error> {
    let look_from = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        look_from,
        look_at,
        Vector3::new(0.0, 1.0, 0.0),
        20.0,
        3.0 / 2.0,
        0.1,
        10.0,
        1200,
    );

    let world = random_world();

    let samples_per_pixel: u32 = 500;
    let mut rng = rand::thread_rng();

    let now = Instant::now();
    let mut lines: Vec<String> = vec![format!(
        "P3\n{} {}\n255",
        camera.image_width, camera.image_height
    )];
    for j in (0..camera.image_height).rev() {
        println!("Rendering scanline {j}");
        for i in 0..camera.image_width {
            let colour: Colour<f64> =
                (0..samples_per_pixel).fold(Colour::new(0.0, 0.0, 0.0), |acc, _| {
                    let u = (i as f64 + rng.gen::<f64>()) / (camera.image_width - 1) as f64;
                    let v = (j as f64 + rng.gen::<f64>()) / (camera.image_height - 1) as f64;

                    let ray = camera.get_ray(u, v);
                    acc + ray_colour(&ray, &world, 50)
                }) * (1.0 / samples_per_pixel as f64);

            let mapped = Colour::<u32>::from(Colour::new(
                colour.r.sqrt(),
                colour.g.sqrt(),
                colour.b.sqrt(),
            ));
            lines.push(format!("{} {} {}", mapped.r, mapped.g, mapped.b));
        }
    }
    println!("Rendering took {:.2?}", now.elapsed());

    let mut output = File::create("output.ppm")?;
    writeln!(output, "{}", lines.join("\n"))?;

    Ok(())
}
