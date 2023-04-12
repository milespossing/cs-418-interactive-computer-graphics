use crate::models::{ObjPrimative, SceneObject};
use crate::scene::Scene;
use nalgebra::{Point3, Vector3};
use std::ops::{Add, Div, Sub};
use uuid::Uuid;

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }
}

#[derive(Debug, Clone)]
pub struct RayHit {
    pub position: Point3<f64>,
    pub direction: Vector3<f64>,
    pub distance: f64,
    pub object_id: Uuid,
    pub surface_normal: Vector3<f64>,
}

// need to perform raytracing given a scene
pub struct RayTracer<'a> {
    scene: &'a Scene,
}

fn plane_intersection(ray: &Ray, object: &SceneObject, n: Vector3<f64>, p: Point3<f64>) -> Option<RayHit> {
    let t = p.sub(ray.origin).dot(&n) / ray.direction.dot(&n);
    return if t < 0f64 {
        None
    } else {
        let position = ray.origin.add(ray.direction.scale(t));
        Some(RayHit {
            position,
            direction: ray.direction,
            distance: t,
            object_id: object.id,
            surface_normal: n,
        })
    };
}

fn find_intersection(ray: &Ray, object: &SceneObject) -> Option<RayHit> {
    match object.primitive {
        ObjPrimative::Sphere { xyz, r } => {
            // check if the ray starts inside the sphere
            let vec_to_sphere = xyz.sub(ray.origin);
            let inside = vec_to_sphere.magnitude_squared() < r.powi(2);
            let t_c = vec_to_sphere
                .dot(&ray.direction)
                .div(ray.direction.magnitude());
            if !inside && t_c < 0f64 {
                return None;
            }
            let d2 = ray
                .origin
                .add(ray.direction.scale(t_c))
                .sub(xyz)
                .magnitude_squared();
            if !inside && r.powi(2) < d2 {
                return None;
            }
            let r2 = r.powi(2);
            let num = (r2 - d2).sqrt();
            let den = ray.direction.magnitude();
            let t_offset = num / den;
            let distance = match inside {
                true => t_c + t_offset,
                false => t_c - t_offset,
            };
            let intersection: Point3<f64> = ray.origin.add(ray.direction.scale(distance));
            let surface_normal = intersection.sub(xyz).scale(1.0 / r);
            Some(RayHit {
                position: intersection,
                direction: ray.direction,
                distance,
                object_id: object.id,
                surface_normal,
            })
        }
        ObjPrimative::Plane { n, p } => {
            plane_intersection(ray, object, n, p)
        },
        ObjPrimative::Triangle { vertices, n, e1, e2 } => {
            let intersection = plane_intersection(ray, object, n, vertices[0]);
            match intersection {
                None => None,
                Some(hit) => {
                    let b1 = e1.dot(&hit.position.sub(vertices[0]));
                    let b2 = e2.dot(&hit.position.sub(vertices[0]));
                    let b0 = 1.0 - b1 - b2;
                    if b0 > 0.0 && b1 > 0.0 && b2 > 0.0 {
                        Some(hit)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl<'a> RayTracer<'a> {
    pub fn new(scene: &'a Scene) -> Self {
        Self { scene }
    }

    pub fn trace_ray(&self, ray: &Ray) -> Option<RayHit> {
        self.scene
            .objects
            .iter()
            .map(|o| find_intersection(ray, &o))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
    }

    pub fn filter_trace_ray(&self, ray: &Ray, ignore_object_id: Uuid) -> Option<RayHit> {
        self.scene
            .objects
            .iter()
            .filter(|o| o.id != ignore_object_id)
            .map(|o| find_intersection(ray, &o))
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
    }
}
