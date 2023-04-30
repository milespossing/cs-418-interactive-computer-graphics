use nalgebra::{Point3, Vector3};
use std::ops::Div;
use uuid::Uuid;

#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub min: Point3<f64>,
    pub max: Point3<f64>,
}

impl AABB {
    pub fn new(min: Point3<f64>, max: Point3<f64>) -> Self {
        Self { min, max }
    }

    pub fn overlaps(&self, other: &AABB) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }

    // This function created by chatGPT and modified heavily to save a bit of time and annoying coding
    // turns out that it didn't same me much time. Maybe chatGPT 5 will do better, but I'm not sure.
    pub fn subdivide(&self) -> Vec<AABB> {
        let half_vec = (self.max - self.min).div(2.0);
        let half_outside = self.min + half_vec;
        let mut aabbs: Vec<AABB> = vec![];

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let scaler = Vector3::<f64>::new(i as f64, j as f64, k as f64);
                    let min_point = self.min + half_vec.component_mul(&scaler);
                    let max_point = half_outside + half_vec.component_mul(&scaler);
                    aabbs.push(AABB::new(min_point, max_point));
                }
            }
        }

        aabbs
    }
}

#[cfg(test)]
mod aabb_tests {
    use super::*;

    #[test]
    fn overlaps_with_partially_outside_box() {
        let a = AABB::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0));
        let b = AABB::new(Point3::new(-2.0, -2.0, -2.0), Point3::new(0.0, 0.0, 0.0));
        assert_eq!(true, a.overlaps(&b));
        assert_eq!(true, b.overlaps(&a));
    }

    #[test]
    fn overlaps_with_entirely_inside_box() {
        let a = AABB::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0));
        let b = AABB::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.5, 0.5, 0.5));
        assert_eq!(true, a.overlaps(&b));
        assert_eq!(true, b.overlaps(&a));
    }

    #[test]
    fn does_not_overlap_with_entirely_outside_box() {
        let a = AABB::new(Point3::new(-1.0, -1.0, -1.0), Point3::new(1.0, 1.0, 1.0));
        let b = AABB::new(Point3::new(2.0, 2.0, 2.0), Point3::new(3.0, 3.0, 3.0));
        assert_eq!(false, a.overlaps(&b));
        assert_eq!(false, b.overlaps(&a));
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
