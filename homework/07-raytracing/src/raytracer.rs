use crate::models::{ObjPrimative, SceneObject, AABB};
use crate::scene::{BVHNode, Scene, MAX_OBJECTS};
use nalgebra::{Point3, Vector3};
use std::ops::{Add, Div, Sub};
use uuid::Uuid;

const FORCE_BVH: bool = true;
const MIN_RAY_LENGTH: f64 = 0.0001;

pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }

    fn intersect_aabb(&self, aabb: &AABB) -> Option<f64> {
        let mut tmin = -f64::INFINITY;
        let mut tmax = f64::INFINITY;

        for i in 0..3 {
            let inv_d = 1.0 / self.direction[i];
            let mut t1 = (aabb.min[i] - self.origin[i]) * inv_d;
            let mut t2 = (aabb.max[i] - self.origin[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t1, &mut t2);
            }

            tmin = tmin.max(t1);
            tmax = tmax.min(t2);

            if tmax <= tmin {
                return None;
            }
        }

        Some(tmin)
    }
}

// Test suite for Ray
#[cfg(test)]
mod ray_tests {
    use super::*;
    // test to see if a ray intersects an AABB
    #[test]
    pub fn ray_intersects_from_outside() {
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let aabb = AABB::new(Point3::new(1.0, -1.0, -1.0), Point3::new(2.0, 1.0, 1.0));
        let inter = r.intersect_aabb(&aabb);
        assert_eq!(inter, Some(1.0));
    }

    #[test]
    pub fn ray_intersects_from_inside() {
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let aabb = AABB::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0));
        let inter = r.intersect_aabb(&aabb);
        assert_eq!(inter, Some(-1.0));
    }

    #[test]
    pub fn returns_none_if_ray_does_not_intersect_any_object() {
        let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let aabb = AABB::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(-1.0, -1.0, -1.0));
        assert_eq!(r.intersect_aabb(&aabb), None);
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
    force_bvh: bool,
}

fn plane_intersection(
    ray: &Ray,
    object: &SceneObject,
    n: Vector3<f64>,
    p: Point3<f64>,
) -> Option<RayHit> {
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

impl<'a> RayTracer<'a> {
    pub fn new(scene: &'a Scene) -> Self {
        Self {
            force_bvh: FORCE_BVH,
            scene,
        }
    }

    fn find_intersection(&self, ray: &Ray, object: &SceneObject) -> Option<RayHit> {
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
                    surface_normal: match inside {
                        false => surface_normal,
                        true => -surface_normal,
                    },
                })
            }
            ObjPrimative::Plane { n, p } => plane_intersection(ray, object, n, p),
            ObjPrimative::Triangle {
                vertices,
                n,
                e1,
                e2,
            } => {
                let intersection = plane_intersection(ray, object, n, vertices[0]);
                match intersection {
                    None => None,
                    Some(mut hit) => {
                        hit.surface_normal = match n {
                            a if self.scene.camera_settings.forward.dot(&a) > 0.0 => -a,
                            a => a,
                        };
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

    fn find_closest_intersection(
        &self,
        ray: &Ray,
        objects: &Vec<SceneObject>,
        ignore_object_id: Option<Uuid>,
    ) -> Option<RayHit> {
        let hits: Vec<RayHit> = objects
            .iter()
            .filter_map(|o| match ignore_object_id {
                None => self.find_intersection(ray, o),
                Some(id) if id != o.id => self.find_intersection(ray, o),
                _ => None,
            })
            .collect();
        let result = hits
            .iter()
            .filter(|&i| i.distance > MIN_RAY_LENGTH)
            .map(|i| i.clone())
            .min_by(|a, b| a.distance.total_cmp(&b.distance));
        return result;
    }

    fn find_intersection_bvh(
        &self,
        node: &BVHNode,
        ray: &Ray,
        filter_object: Option<Uuid>,
    ) -> Option<RayHit> {
        match &node.children {
            None => {
                self.find_closest_intersection(ray, &node.bounding_volume.children, filter_object)
            }
            Some(children) => {
                children
                    .iter()
                    .filter(|&n| match ray.intersect_aabb(&n.bounding_volume.aabb) {
                        None => false,
                        _ => true,
                    })
                    .filter_map(|n| self.find_intersection_bvh(n, ray, filter_object))
                    .min_by(|a, b| a.distance.total_cmp(&b.distance))
            }
        }
    }

    pub fn trace_ray(&self, ray: &Ray, ignore_object_id: Option<Uuid>) -> Option<RayHit> {
        if !self.force_bvh && self.scene.objects.len() < MAX_OBJECTS {
            self.find_closest_intersection(ray, &self.scene.objects, ignore_object_id)
        } else {
            let root_inter = ray.intersect_aabb(&self.scene.bvh.bounding_volume.aabb);
            if root_inter.is_some() {
                self.find_intersection_bvh(&self.scene.bvh, ray, ignore_object_id)
            } else {
                let planes = self.scene.objects.iter().filter_map(|&o| match o.primitive {
                    ObjPrimative::Plane {.. } => Some(o.clone()),
                    _ => None,
                }).collect();
                self.find_closest_intersection(ray, &planes, ignore_object_id)
            }
        }
    }
}

#[cfg(test)]
mod bvh_tests {
    use super::*;
    use crate::parser::{parse_file, ProcFile};
    use crate::renderer::Renderer;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_bvh() {
        let file: ProcFile =
            parse_file(PathBuf::from_str("tests/data/bvh_test_shadow.txt").unwrap()).unwrap();

        let scene = Scene::from_file(&file).unwrap();
        let renderer = Renderer::from_file(&file, &scene).unwrap();
        let output = renderer.render_scene().unwrap();
    }
}
