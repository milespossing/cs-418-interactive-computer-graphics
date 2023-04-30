use crate::models::{ObjPrimative, SceneObject};
use crate::scene::{Scene, MAX_OBJECTS, BVHNode};
use nalgebra::{Point3, Vector3};
use std::ops::{Add, Div, Sub};
use uuid::Uuid;

const ENABLE_BVH: bool = true;

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
        Self { scene }
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

    fn find_closest_intersection(&self, ray: &Ray, objects: &Vec<SceneObject>, ignore_object_id: Option<Uuid>) -> Option<RayHit> {
        objects
            .iter()
            .map(|o| self.find_intersection(ray, &o))
            .filter(|o| match (o, ignore_object_id) {
                (Some(d), Some(uuid)) if uuid == d.object_id => false,
                (Some(d), _) => d.distance > 0.00001,
                (None, _) => false,
            })
            .map(|o| o.unwrap())
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
    }

    fn find_intersection_bvh(&self, node: &BVHNode, ray: &Ray, filter_object: Option<Uuid>) -> Option<RayHit> {
        if node.is_leaf() {
            self.find_closest_intersection(ray, &node.bounding_volume.children, filter_object)
            // node.bounding_volume.children
            //     .iter()
            //     .map(|o| self.find_intersection(ray, &o))
            //     .filter(|o| match o {
            //         Some(d) => d.distance > 0.00001,
            //         None => false,
            //     })
            //     .map(|o| o.unwrap())
            //     .min_by(|a, b| a.distance.total_cmp(&b.distance))
        } else {
            let mut intersections: Vec<(&BVHNode, f64)> = match &node.children {
                None => Vec::new(),
                Some(children) => {
                    children.iter()
                        .filter_map(|n| { match n.bounding_volume.aabb.intersect(ray) {
                            None => None,
                            Some(d) => Some((n, d)),
                        }}).collect()//.(|(_, a), (_, b)| a.total_cmp(b));
                }
            };
            intersections.sort_by(|(_, a), (_, b)| a.total_cmp(&b));
            for intersection in intersections {
                if let Some(d) = self.find_intersection_bvh( &intersection.0, ray, filter_object) {
                    return Some(d);
                }
            }
            None
        }
    }

    pub fn trace_ray(&self, ray: &Ray, ignore_object_id: Option<Uuid>) -> Option<RayHit> {
        if ENABLE_BVH && self.scene.objects.len() < MAX_OBJECTS {
            self.find_closest_intersection(ray, &self.scene.objects, ignore_object_id)
            // self.scene
            //     .objects
            //     .iter()
            //     .map(|o| self.find_intersection(ray, &o))
            //     .filter(|o| match (o, ignore_object_id) {
            //         (Some(d), Some(uuid)) if uuid == d.object_id => false,
            //         (Some(d), _) => d.distance > 0.00001,
            //         (None, _) => false,
            //     })
            //     .map(|o| o.unwrap())
            //     .min_by(|a, b| a.distance.total_cmp(&b.distance))
        } else {
            // TODO: need to add the planes back in here
            let root_inter = self.scene.bvh.bounding_volume.aabb.intersect(ray);
            if root_inter.is_some() {
                return self.find_intersection_bvh(&self.scene.bvh, ray, ignore_object_id);
            }
            None
        }
    }
    //
    // pub fn filter_trace_ray(&self, ray: &Ray, ignore_object_id: Uuid) -> Option<RayHit> {
    //     self.scene
    //         .objects
    //         .iter()
    //         .filter(|o| o.id != ignore_object_id)
    //         .map(|o| self.find_intersection(ray, &o))
    //         .filter(|o| o.is_some())
    //         .map(|o| o.unwrap())
    //         .min_by(|a, b| a.distance.total_cmp(&b.distance))
    // }
}
