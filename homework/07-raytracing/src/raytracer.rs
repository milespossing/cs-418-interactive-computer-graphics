use std::ops::{Add, Div, Sub};
use nalgebra::{Point3, Vector3};
use crate::models::{ObjPrimative, SceneObject};

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction, }
    }
}

pub struct RayHit {
    pub distance: f64,
}

// need to perform raytracing given a scene
pub struct RayTracer<'a> {
    objects: &'a Vec<SceneObject>,
}

fn find_intersection(ray: &Ray, object: &ObjPrimative) -> Option<RayHit> {
    match object {
        ObjPrimative::Sphere { xyz, r } => {
            // check if the ray starts inside the sphere
            let vec_to_sphere = xyz.sub(ray.origin);
            let inside = vec_to_sphere.magnitude_squared() < r.powi(2);
            let t_c = vec_to_sphere.dot(&ray.direction).div(ray.direction.magnitude());
            if !inside && t_c < 0f64 { return None; }
            let d2 = ray.origin.add(ray.direction.scale(t_c)).sub(xyz).magnitude_squared();
            if !inside && r.powi(2) < d2 { return None; }
            let t_offset = (r.powi(2) - d2.sqrt()).sqrt() / ray.direction.magnitude();
            match inside {
                true => Some(RayHit { distance: t_c + t_offset }),
                false => Some(RayHit { distance: t_c - t_offset }),
            }
        }
    }
}

impl<'a> RayTracer<'a> {
    pub fn new(objects: &'a Vec<SceneObject>) -> Self { Self { objects } }

    pub fn trace_ray(&self, ray: &Ray) -> Option<RayHit> {
        self
            .objects
            .iter()
            .map(|o| find_intersection(ray, &o.primitive))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
    }
}