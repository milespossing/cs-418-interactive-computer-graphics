use nalgebra::Point3;
use crate::models::{DEFAULT_MATERIAL, Material, ObjPrimative, SceneObject};
use crate::parser::{ProcFile, FileEntry};

#[derive(Debug)]
pub struct Scene {
    pub objects: Vec<SceneObject>,
}

impl Scene {
    pub fn from_file(file: &ProcFile) -> Result<Self, String> {
        let mut objects: Vec<SceneObject> = vec![];
        let mut material: Material = DEFAULT_MATERIAL;

        for entry in &file.entries {
            let primitive: ObjPrimative = match entry {
                FileEntry::Sphere { x, y, z, r } => ObjPrimative::Sphere { xyz: Point3::<f64>::from([*x, *y, *z]), r: *r },
            };
            objects.push(SceneObject::new(primitive, material.clone()));
        }
        Ok(Self { objects })
    }
}