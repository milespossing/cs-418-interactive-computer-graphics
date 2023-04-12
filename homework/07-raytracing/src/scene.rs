use nalgebra::{Point3, Vector3};
use crate::models::{DEFAULT_COLOR, DEFAULT_MATERIAL, LightPrimitive, LightSourceObject, Material, ObjPrimative, SceneObject};
use crate::parser::{ProcFile, FileEntry};

#[derive(Debug)]
pub struct Scene {
    pub light_sources: Vec<LightSourceObject>,
    pub objects: Vec<SceneObject>,
}

impl Scene {
    pub fn from_file(file: &ProcFile) -> Result<Self, String> {
        let mut objects: Vec<SceneObject> = vec![];
        let mut light_sources: Vec<LightSourceObject> = vec![];
        let mut material: Material = DEFAULT_MATERIAL;
        let mut color: Vector3<f64> = DEFAULT_COLOR;

        for entry in &file.entries {
            match entry {
                // primitives
                FileEntry::Sphere { x, y, z, r } => {
                    let primitive = ObjPrimative::Sphere { xyz: Point3::<f64>::new(*x, *y, *z), r: *r };
                    objects.push(SceneObject::new(primitive, material.clone()));
                },
                // lighting
                FileEntry::Sun { x, y, z } => {
                    let light_source = LightPrimitive::Directional(Vector3::new(*x, *y, *z).normalize());
                    light_sources.push(LightSourceObject::new(light_source, color));
                },
                // settings
                FileEntry::Color { r, g, b } => {
                    color = Vector3::new(*r, *g, *b);
                    material.color = color;
                }
            };
        }
        Ok(Self { objects, light_sources })
    }
}