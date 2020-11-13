mod camera;
mod material;
mod math;
mod plane;
mod random;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use material::Material;
use math::clamp01;
use plane::Plane;
use random::RngXorShift;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

use std::fs::File;

const SAMPLES: usize = 256;
const SAMPLE_SCALE: f32 = 1.0 / (SAMPLES as f32);

const BOUNCES: usize = 8;

fn main() {
    let width = 1920;
    let height = 1080;
    let mut image_data: Vec<u8> = vec![0 as u8; width * height * 3];

    let camera = Camera::look_at(
        Vec3::new(0.0, 0.0, 0.5),
        Vec3::new(0.0, -0.25, -1.0),
        std::f32::consts::FRAC_PI_2,
        16.0 / 9.0,
    );

    let now = std::time::Instant::now();

    const TILE_SIZE: usize = 64;

    let tile_count_x = (width + TILE_SIZE - 1) / TILE_SIZE;
    let tile_count_y = (height + TILE_SIZE - 1) / TILE_SIZE;

    let materials = vec![
        Material {
            reflection: Vec3::new(1.0, 1.0, 1.0),
            emission: Vec3::new(0.0, 0.0, 0.0),
            scattering: 1.0,
        },
        Material {
            reflection: Vec3::new(1.0, 1.0, 1.0),
            emission: Vec3::new(0.0, 0.0, 0.0),
            scattering: 0.0,
        },
        Material {
            reflection: Vec3::new(0.3, 0.3, 1.0),
            emission: Vec3::new(0.0, 0.0, 0.0),
            scattering: 1.0,
        },
        Material {
            reflection: Vec3::new(1.0, 0.3, 0.3),
            emission: Vec3::new(0.0, 0.0, 0.0),
            scattering: 1.0,
        },
        Material {
            reflection: Vec3::new(1.0, 1.0, 1.0),
            emission: Vec3::new(0.7, 0.7, 0.7),
            scattering: 1.0,
        },
    ];

    let spheres = vec![
        (
            Sphere {
                center: Vec3::new(0.25, -0.3, -0.15),
                radius: 0.2,
            },
            0,
        ),
        (
            Sphere {
                center: Vec3::new(-0.25, -0.3, -0.25),
                radius: 0.2,
            },
            1,
        ),
        (
            Sphere {
                center: Vec3::new(0.0, 0.6, -0.25),
                radius: 0.25,
            },
            4,
        ),
    ];

    let planes = vec![
        (
            Plane {
                normal: Vec3::new(0.0, 1.0, 0.0),
                distance: -0.5,
            },
            0,
        ),
        (
            Plane {
                normal: Vec3::new(0.0, -1.0, 0.0),
                distance: -0.5,
            },
            0,
        ),
        (
            Plane {
                normal: Vec3::new(0.0, 0.0, -1.0),
                distance: -0.5,
            },
            0,
        ),
        (
            Plane {
                normal: Vec3::new(0.0, 0.0, 1.0),
                distance: -0.5,
            },
            0,
        ),
        (
            Plane {
                normal: Vec3::new(-1.0, 0.0, 0.0),
                distance: -0.5,
            },
            2,
        ),
        (
            Plane {
                normal: Vec3::new(1.0, 0.0, 0.0),
                distance: -0.5,
            },
            3,
        ),
    ];

    let state = (&materials, &spheres, &planes);

    struct Tile {
        y_min: usize,
        y_max: usize,
        x_min: usize,
        x_max: usize,
        data: [u8; TILE_SIZE * TILE_SIZE * 3],
    }

    let mut tasks: Vec<Tile> = Vec::with_capacity(tile_count_x * tile_count_y);

    for y in 0..tile_count_y {
        let y_min = y * TILE_SIZE;
        let y_max = std::cmp::min(y_min + TILE_SIZE, height);
        for x in 0..tile_count_x {
            let x_min = x * TILE_SIZE;
            let x_max = std::cmp::min(x_min + TILE_SIZE, width);

            tasks.push(Tile {
                y_min,
                y_max,
                x_min,
                x_max,
                data: [0; TILE_SIZE * TILE_SIZE * 3],
            });
        }
    }

    let mut pool = scoped_threadpool::Pool::new(4);

    pool.scoped(|scope| {
        for t in &mut tasks {
            scope.execute(move || {
                let mut rng = RngXorShift::new(58727590);

                for y in (t.y_min..t.y_max).rev() {
                    for x in t.x_min..t.x_max {
                        let mut color = Vec3::zero();
                        for _ in 0..SAMPLES {
                            let u = (x as f32 + rng.bi()) / (width as f32 - 1.0);
                            let v = (y as f32 + rng.bi()) / (height as f32 - 1.0);
                            let ray = camera.ray_from_uv(u, v);

                            color = color.add(ray_color(ray, state, &mut rng).scale(SAMPLE_SCALE));
                        }

                        let i = ((TILE_SIZE - (y - t.y_min) - 1) * TILE_SIZE + (x - t.x_min)) * 3;
                        t.data[i] = (clamp01(color.x.sqrt()) * 255.0) as u8;
                        t.data[i + 1] = (clamp01(color.y.sqrt()) * 255.0) as u8;
                        t.data[i + 2] = (clamp01(color.z.sqrt()) * 255.0) as u8;
                    }
                }
            });
        }
    });

    for t in tasks {
        for y in (t.y_min..t.y_max).rev() {
            for x in t.x_min..t.x_max {
                let i = ((height - y - 1) * width + x) * 3;
                let task_i = ((TILE_SIZE - (y - t.y_min) - 1) * TILE_SIZE + (x - t.x_min)) * 3;
                image_data[i + 0] = t.data[task_i + 0];
                image_data[i + 1] = t.data[task_i + 1];
                image_data[i + 2] = t.data[task_i + 2];
            }
        }
    }

    println!("Took: {}ms", now.elapsed().as_millis());
    write_image_to_file(width, height, &image_data);
}

fn sky_color(ray: Ray) -> Vec3 {
    let t = (ray.direction.normalize().y + 1.0) * 0.5;
    let white = Vec3::one();
    let blue = Vec3::new(0.5, 0.7, 1.0);
    white.lerp(blue, t)
}

fn ray_color(
    ray: Ray,
    state: (&Vec<Material>, &Vec<(Sphere, u8)>, &Vec<(Plane, u8)>),
    rng: &mut RngXorShift,
) -> Vec3 {
    let mut ray = ray;
    let mut atten = Vec3::one();
    let mut color = Vec3::zero();
    for _ in 0..BOUNCES {
        let mut normal_index: Option<(Vec3, u8)> = None;

        const MIN_DISTANCE: f32 = 0.0001;

        let mut min = std::f32::MAX;
        for (s, i) in state.1 {
            let t = s.intersect(ray);
            if t > MIN_DISTANCE && t < min {
                min = t;

                let point = ray.at(t);
                normal_index = Some((s.normal(point), *i));
            }
        }

        for (p, i) in state.2 {
            let t = p.intersect(ray);
            if t > MIN_DISTANCE && t < min {
                min = t;
                normal_index = Some((p.normal, *i));
            }
        }

        if let Some((normal, index)) = normal_index {
            let point = ray.at(min);
            let material = state.0[index as usize];
            let direction = {
                let diffuse = normal.add(rng.unit());
                let mirror = ray
                    .direction
                    .sub(normal.scale(normal.dot(ray.direction) * 2.0));
                mirror.lerp(diffuse, material.scattering)
            };
            ray = Ray {
                origin: point,
                direction,
            };
            color = color.add(atten.hadamard(material.emission));
            atten = atten.scale(0.5).hadamard(material.reflection);
        } else {
            return atten.hadamard(sky_color(ray));
        }
    }
    color
}

fn write_image_to_file(width: usize, height: usize, image_data: &Vec<u8>) {
    let file = File::create("image.png").expect("Failed to create file image.png");

    let ref mut w = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(image_data.as_slice()).unwrap();
}
