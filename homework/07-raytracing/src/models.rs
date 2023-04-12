use nalgebra::{Point3, Vector3};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum ObjPrimative {
    Sphere { xyz: Point3<f64>, r: f64 },
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Vector3<f64>,
    pub albedo: f64,
}

pub const DEFAULT_COLOR: Vector3<f64> = Vector3::new(1.0, 1.0, 1.0);
pub const DEFAULT_ALBEDO: f64 = 1.1;
pub const DEFAULT_MATERIAL: Material = Material {
    color: DEFAULT_COLOR,
    albedo: DEFAULT_ALBEDO,
};

#[derive(Debug, Clone, Copy)]
pub struct SceneObject {
    pub id: Uuid,
    pub primitive: ObjPrimative,
    pub material: Material,
}

impl SceneObject {
    pub fn new(primative: ObjPrimative, material: Material) -> Self {
        Self {
            id: Uuid::new_v4(),
            primitive: primative,
            material,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LightPrimitive {
    Directional(Vector3<f64>),
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
