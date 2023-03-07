use crate::DVec3;
use crate::rand_utils;
use crate::tracer::scene::Scene;
use crate::tracer::camera::Camera;
use rayon::iter::{ParallelIterator, IntoParallelIterator};

pub fn _render(
    img_height: usize,
    px_height: f64,
    img_width: usize,
    px_width: f64,
    num_samples: usize,
    cam: &Camera,
    scene: &Scene,
) -> Vec<DVec3> {
    (0..img_height).into_par_iter().flat_map(|y| {
        (0..img_width).map(|x| {
            let u = x as f64 * px_width;
            let v = (img_height - 1 - y) as f64 * px_height;

            (0..num_samples).map(|_| {
                let randx = rand_utils::rand_f64();
                let randy = rand_utils::rand_f64();
                cam.ray_at(u + randx*px_width, v + randy*px_height)
            }).fold(DVec3::ZERO, |acc, r| acc + r.color(scene, 0))
                / num_samples as f64
        }).collect::<Vec<DVec3>>()
    }).collect()
}
