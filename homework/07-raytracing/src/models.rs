use nalgebra::{Point3, Vector3};
use std::ops::{Div};
use uuid::Uuid;
use crate::raytracer::Ray;

#[derive(Copy, Clone, Debug)]
pub struct AABB(pub [Point3<f64>; 2]);

// TODO: Add tests here

impl AABB {
    pub fn new(min: Point3<f64>, max: Point3<f64>) -> Self {
        Self([min, max])
    }

    pub fn overlaps(&self, other: &AABB) -> bool {
        self.0[0].x < other.0[1].x
            && self.0[1].x > other.0[0].x
            && self.0[0].y < other.0[1].y
            && self.0[1].y > other.0[0].y
            && self.0[0].z < other.0[1].z
            && self.0[1].z > other.0[0].z
    }

    // This function created by chatGPT and modified heavily to save a bit of time and annoying coding
    // turns out that it didn't same me much time. Maybe chatGPT 5 will do better, but I'm not sure.
    pub fn subdivide(&self) -> [AABB; 8] {
        let half_vec = (self.0[1] - self.0[0]).div(2.0);
        let half_outside = self.0[0] + half_vec;
        let mut aabbs = [AABB([Point3::new(0.0, 0.0, 0.0); 2]); 8];

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let scaler = Vector3::<f64>::new(i as f64, j as f64, k as f64);
                    let min_point = self.0[0] + half_vec.component_mul(&scaler);
                    let max_point = half_outside + half_vec.component_mul(&scaler);
                    aabbs[usize::try_from(i * 4 + j * 2 + k).unwrap()] =
                        AABB([min_point, max_point]);
                }
            }
        }

        aabbs
    }

    pub fn intersect(&self, ray: &Ray) -> Option<f64> {
        let mut tmin = f64::NEG_INFINITY;
        let mut tmax = f64::INFINITY;
        let invdir = ray.direction.map(|d| 1.0 / d);

        for i in 0..3 {
            let t0 = (self.0[0][i] - ray.origin[i]) * invdir[i];
            let t1 = (self.0[1][i] - ray.origin[i]) * invdir[i];
            tmin = tmin.max(t0.min(t1));
            tmax = tmax.min(t0.max(t1));
        }

        if tmax < tmin {
            None
        } else {
            Some(tmin)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ObjPrimative {
    Sphere {
        xyz: Point3<f64>,
        r: f64,
    },
    Plane {
        n: Vector3<f64>,
        p: Point3<f64>,
    },
    Triangle {
        vertices: [Point3<f64>; 3],
        n: Vector3<f64>,
        e1: Vector3<f64>,
        e2: Vector3<f64>,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Vector3<f64>,
    pub albedo: f64,
    pub shininess: f64,
}

pub const DEFAULT_COLOR: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0);
pub const DEFAULT_ALBEDO: f64 = 1.04;
pub const DEFAULT_SHININESS: f64 = 0.0;
pub const DEFAULT_MATERIAL: Material = Material {
    color: DEFAULT_COLOR,
    albedo: DEFAULT_ALBEDO,
    shininess: DEFAULT_SHININESS,
};

#[derive(Debug, Clone, Copy)]
pub struct SceneObject {
    pub id: Uuid,
    pub primitive: ObjPrimative,
    pub material: Material,
    pub aabb: Option<AABB>,
}

impl SceneObject {
    pub fn new(primative: ObjPrimative, material: Material) -> Self {
        let aabb: Option<AABB> = match primative {
            ObjPrimative::Sphere { xyz, r } => {
                Some(AABB::new(xyz.map(|i| i - r), xyz.map(|i| i + r)))
            }
            ObjPrimative::Triangle {
                vertices,
                n: _,
                e1: _,
                e2: _,
            } => {
                let min_x = vertices.iter().fold(f64::INFINITY, |a, &b| a.min(b.x));
                let min_y = vertices.iter().fold(f64::INFINITY, |a, &b| a.min(b.y));
                let min_z = vertices.iter().fold(f64::INFINITY, |a, &b| a.min(b.z));
                let max_x = vertices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b.x));
                let max_y = vertices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b.y));
                let max_z = vertices.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b.z));
                Some(AABB::new(
                    Point3::new(min_x, min_y, min_z),
                    Point3::new(max_x, max_y, max_z),
                ))
            }
            _ => None,
        };
        Self {
            id: Uuid::new_v4(),
            primitive: primative,
            material,
            aabb,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LightPrimitive {
    Directional(Vector3<f64>),
    Point(Point3<f64>),
}

#[derive(Debug, Clone, Copy)]
pub struct LightSourceObject {
    pub source: LightPrimitive,
    pub color: Vector3<f64>,
}

impl LightSourceObject {
    pub fn new(source: LightPrimitive, color: Vector3<f64>) -> Self {
        Self { source, color }
    }
}
