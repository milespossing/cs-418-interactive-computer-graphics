use nalgebra::{Point3, Vector3};

#[derive(Debug, Clone, Copy)]
pub enum ObjPrimative {
    Sphere { xyz: Point3<f64>, r: f64 },
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Vector3<f64>,
    pub alpha: f64,
}

pub const DEFAULT_COLOR: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0);
pub const DEFAULT_ALPHA: f64 = 1.0;
pub const DEFAULT_MATERIAL: Material = Material {
    color: DEFAULT_COLOR,
    alpha: DEFAULT_ALPHA,
};

#[derive(Debug, Clone, Copy)]
pub struct SceneObject {
    pub primitive: ObjPrimative,
    pub material: Material,
}

impl SceneObject {
    pub fn new(primative: ObjPrimative, material: Material) -> Self {
        Self {
            primitive: primative,
            material,
        }
    }
}
