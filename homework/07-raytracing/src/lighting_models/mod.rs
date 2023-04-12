use crate::lighting_models::lambert::LambertLighting;
use crate::models::LightPrimitive;
use crate::parser::ProcFile;
use crate::raytracer::{Ray, RayHit, RayTracer};
use crate::scene::Scene;
use nalgebra::Vector3;
use std::ops::Add;

mod lambert;

pub struct LightingModel<'a> {
    lambert: LambertLighting,
    scene: &'a Scene,
    ray_tracer: RayTracer<'a>,
}

impl<'a> LightingModel<'a> {
    pub fn from_file(_file: &ProcFile, scene: &'a Scene) -> Self {
        Self {
            lambert: LambertLighting {},
            scene,
            ray_tracer: RayTracer::new(scene),
        }
    }

    // Gets the light color incident to a surface from the lights in a scene
    pub fn light(&self, hit: &RayHit) -> Vector3<f64> {
        let mut result = Vector3::<f64>::zeros();
        for light in &self.scene.light_sources {
            let light_result: Option<Vector3<f64>> = match light.source {
                LightPrimitive::Directional(d) => {
                    let shadow_ray = Ray::new(hit.position, d);
                    match self.ray_tracer.filter_trace_ray(&shadow_ray, hit.object_id) {
                        Some(_) => {
                            None
                        },
                        None => {
                            let dist = self.lambert.get_distribution(&d, &hit.surface_normal);
                            Some(light.color.scale(dist))
                        }
                    }
                }
            };
            match light_result {
                Some(v) => {
                    result = result.add(&v);
                }
                None => continue,
            }
        }
        result
    }
}
