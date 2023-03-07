use glam::f64::{DVec3, DMat3};
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkGroup};
use std::time::Duration;

use rust_tracer::perlin::Perlin;
use rust_tracer::renderer;
use rust_tracer::tracer::scene::Scene;
use rust_tracer::tracer::camera::Camera;
use rust_tracer::tracer::texture::Texture;
use rust_tracer::tracer::material::Material;
use rust_tracer::tracer::object::sphere::Sphere;
use rust_tracer::tracer::object::{Cuboid, Plane, Rectangle};


const WIDTH: usize = 256;
const PX_WID: f64 = 1.0 / (WIDTH - 1) as f64;
const HEIGHT: usize = 140;
const PX_HEI: f64 = 1.0 / (HEIGHT - 1) as f64;
const NUM_SAMPLES: usize = 1;
const VFOV: f64 = 90.0;
const ASPECT_RATIO: f64 = WIDTH as f64 / HEIGHT as f64;

fn criterion_renderer(c: &mut Criterion) {
    let scene = _scene();
    let cam = Camera::new(
        ASPECT_RATIO,
        VFOV,
        DVec3::new(0.0, 0.0, 0.0), // origin
        DVec3::new(0.0, 0.0, -1.0), // towards
        DVec3::new(0.0, 1.0, 0.0), // up
    );
    let mut g = c.benchmark_group("grp");
    g.sample_size(11);
    g.bench_function("bench 1", |b| b.iter(|| {
        bench_render(black_box(&cam), black_box(&scene))
    }));
    g.finish();
}

criterion_group!(benches, criterion_renderer);
criterion_main!(benches);

fn bench_render(cam: &Camera, scene: &Scene) {
    renderer::_render(
        HEIGHT,
        PX_HEI,
        WIDTH,
        PX_WID,
        NUM_SAMPLES,
        cam,
        scene
    );
}



fn _scene() -> Scene {
    let l = DVec3::new(-0.3, 0.2, -0.1);
    Scene::new(
        l,
        DVec3::splat(0.15),
        vec![
            // floor
            Plane::new(
                DVec3::new(0.0, -0.5, 0.0),
                DVec3::new(0.0, 1.0, 0.0),
                Material::Phong(Texture::Solid(DVec3::ONE))
                    /*Texture::Checkerboard(
                    Box::new(Texture::Checkerboard(
                        Box::new(Texture::Solid(DVec3::ZERO)),
                        Box::new(Texture::Solid(DVec3::ONE)),
                        4.0,
                    )),
                    /* share same perlin between all textures?
                     * could make cool checkers that way */
                    Box::new(Texture::Marble(
                        Perlin::new(DVec3::splat(192.0) / 255.9)
                    )),
                    1.0,
                )),*/
            ),/*
            // right
            Plane::new(
                DVec3::new(3.0, 0.0, -3.0),
                DVec3::new(-1.0, 0.0, 1.0),
                Material::Phong(Texture::Solid(DVec3::new(0.0, 0.0, 1.0))),
            ),
            Rectangle::new(
                DMat3::from_cols(
                    DVec3::new(1.2, 0.2, -0.8),
                    DVec3::new(0.8, 0.6, -0.4),
                    DVec3::new(0.4, 0.6, -0.8),
                ),
                Material::Mirror,
            ),
            Cuboid::new(
                DMat3::from_cols(
                    DVec3::new(-0.6, -0.5, -0.7),
                    DVec3::new(-0.5, -0.5, -0.7),
                    DVec3::new(-0.5, -0.5, -0.8),
                ),
                DMat3::from_cols(
                    DVec3::new(-0.6, -0.4, -0.7),
                    DVec3::new(-0.5, -0.4, -0.7),
                    DVec3::new(-0.5, -0.4, -0.8),
                ),
                Material::Phong(Texture::Checkerboard(
                    Box::new(Texture::Solid(DVec3::new(1.0, 0.0, 1.0))),
                    Box::new(Texture::Solid(
                        DVec3::new(50.0, 205.0, 50.0) / 255.9
                    )),
                    9.0,
                )),
            ),
            // left
            Plane::new(
                DVec3::new(-3.0, 0.0, -3.0),
                DVec3::new(1.0, 0.0, 1.0),
                Material::Phong(Texture::Solid(DVec3::new(1.0, 0.0, 0.0))),
            ),
            // behind
            Plane::new(
                DVec3::new(0.0, 0.0, 1.0),
                DVec3::new(0.0, 0.0, -1.0),
                Material::Phong(Texture::Solid(DVec3::new(1.0, 0.0, 1.0))),
            ),
            Sphere::new(
                DVec3::new(0.0, 0.0, -1.0),
                0.5,
                Material::Phong(Texture::Solid(
                    DVec3::new(136.0, 8.0, 8.0) / 255.9
                )),
            ),
            Sphere::new(
                DVec3::new(-0.9, 0.0, -1.0),
                0.1,
                Material::Mirror,
            ),
            Sphere::new(
                DVec3::new(-0.4, -0.12, -0.5),
                0.1,
                Material::Glass,
            ),
            Sphere::new(
                DVec3::new(0.4, -0.2, -0.5),
                0.1,
                Material::Phong(Texture::Marble(Perlin::new(
                    DVec3::new(255.0, 182.0, 193.0) / 255.9
                ))),
            ),*/
        ]
    )
}
