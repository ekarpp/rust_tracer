#![allow(dead_code)]
use crate::{DVec3, DVec2};
use crate::rand_utils;
use crate::consts::PATH_TRACE_RR;
use crate::tracer::hit::Hit;
use crate::tracer::material::Material;
use crate::tracer::ray::Ray;
use crate::tracer::scene::Scene;

/// Implements the path tracing algorithm with
/// Russian Roulette (With probability `p` terminate each path.
/// Multiply contributions by reciprocal of `1-p`) and
/// next event estimation (Importance sample light at each impact).
mod path_trace;
/// Naive integrator that importance samples light once.
mod direct_light;
/// bidirectional path tracing
mod bd_path_trace;

/// Enum to choose which integrator to use
pub enum Integrator {
    PathTrace,
    DirectLight,
    BDPathTrace,
}

impl Integrator {
    pub fn integrate(&self, s: &Scene, r: &Ray) -> DVec3 {
        match self {
            Self::PathTrace => path_trace::integrate(s, r, 0, true),
            Self::DirectLight => direct_light::integrate(s, r),
            Self::BDPathTrace => bd_path_trace::integrate(s, r),
        }
    }
}

/// Shoots a shadow ray towards random light from `h`. pass scatter pdf?
fn shadow_ray(scene: &Scene, h: &Hit, rand_sq: DVec2) -> DVec3 {
    let material = h.object.material();
    match material {
        Material::Diffuse(_) => {
            let light = scene.uniform_random_light();

            /* ray to sampled point on light. return tuple with pdf? */
            let r = light.sample_towards(h.p, rand_sq);

            match scene.hit_light(&r, light) {
                None => DVec3::ZERO,
                Some(hl) => {
                    material.brdf(h.p)
                        * h.norm.dot(r.dir.normalize()).abs()
                        / light.sample_towards_pdf(h.p, r.dir, &hl)
                }
            }
        }
        _ => DVec3::ZERO,
    }
}
