use crate::models::{
    LightPrimitive, LightSourceObject, Material, ObjPrimative, SceneObject, DEFAULT_COLOR,
    DEFAULT_MATERIAL,
};
use crate::parser::{FileEntry, ProcFile};
use nalgebra::{Point3, Vector3};
use std::ops::{Div, Sub};
use uuid::Uuid;

#[derive(Debug)]
pub struct CameraSettings {
    pub position: Point3<f64>,
    pub forward: Vector3<f64>,
    pub right: Vector3<f64>,
    pub up: Vector3<f64>,
}

const DEFAULT_CAMERA_SETTINGS: CameraSettings = CameraSettings {
    position: Point3::new(0.0, 0.0, 0.0),
    forward: Vector3::new(0.0, 0.0, -1.0),
    right: Vector3::new(1.0, 0.0, 0.0),
    up: Vector3::new(0.0, 1.0, 0.0),
};

#[derive(Debug)]
pub struct Scene {
    pub camera_settings: CameraSettings,
    pub light_sources: Vec<LightSourceObject>,
    pub objects: Vec<SceneObject>,
}

fn get_vertex(i: i32, v: &Vec<Point3<f64>>) -> Point3<f64> {
    let neg: bool = i < 0;
    let ind: usize = match neg {
        true => v.len() - usize::try_from(i * -1).unwrap(),
        false => usize::try_from(i - 1).unwrap(),
    };
    v[ind]
}

impl Scene {
    pub fn get_object(&self, id: Uuid) -> Option<&SceneObject> {
        match self.objects.iter().find(|&o| o.id == id) {
            Some(o) => Some(&o),
            None => None,
        }
    }
}

impl Scene {
    pub fn from_file(file: &ProcFile) -> Result<Self, String> {
        let mut camera_settings = DEFAULT_CAMERA_SETTINGS;
        let mut objects: Vec<SceneObject> = vec![];
        let mut light_sources: Vec<LightSourceObject> = vec![];
        let mut material: Material = DEFAULT_MATERIAL;
        let mut color: Vector3<f64> = DEFAULT_COLOR;
        let mut vertices: Vec<Point3<f64>> = vec![];


        for entry in &file.entries {
            match entry {
                // primitives
                FileEntry::Sphere { x, y, z, r } => {
                    let primitive = ObjPrimative::Sphere {
                        xyz: Point3::<f64>::new(*x, *y, *z),
                        r: *r,
                    };
                    objects.push(SceneObject::new(primitive, material.clone()));
                }
                FileEntry::Plane { a, b, c, d } => {
                    let n = Vector3::new(*a, *b, *c).normalize();
                    let p: Point3<f64> = match (a, b, c) {
                        (&a, _, _) if a != 0.0 => Point3::new(-d.div(a), 0.0, 0.0),
                        (_, &b, _) if b != 0.0 => Point3::new(0.0, -d.div(b), 0.0),
                        (_, _, &c) if c != 0.0 => Point3::new(0.0, 0.0, -d.div(c)),
                        _ => panic!("Cannot create a plane without a normal"),
                    };
                    let primitive = ObjPrimative::Plane { n, p };
                    objects.push(SceneObject::new(primitive, material.clone()));
                }
                FileEntry::Xyz { x, y, z } => {
                    vertices.push(Point3::new(*x, *y, *z));
                }
                FileEntry::Triangle { a, b, c } => {
                    let p1 = get_vertex(*a, &vertices);
                    let p2 = get_vertex(*b, &vertices);
                    let p3 = get_vertex(*c, &vertices);
                    let vertices: [Point3<f64>; 3] = [p1, p2, p3];
                    let n = vertices[1]
                        .sub(vertices[0])
                        .cross(&vertices[2].sub(vertices[1]))
                        .normalize();
                    let a1 = vertices[2].sub(vertices[0]).cross(&n);
                    let a2 = vertices[1].sub(vertices[0]).cross(&n);
                    let e1 = a1.scale(1.0 / a1.dot(&vertices[1].sub(vertices[0])));
                    let e2 = a2.scale(1.0 / a2.dot(&vertices[2].sub(vertices[0])));
                    let primitive = ObjPrimative::Triangle {
                        vertices,
                        n,
                        e1,
                        e2,
                    };
                    objects.push(SceneObject::new(primitive, material.clone()));
                }
                // lighting
                FileEntry::Sun { x, y, z } => {
                    let light_source =
                        LightPrimitive::Directional(Vector3::new(*x, *y, *z).normalize());
                    light_sources.push(LightSourceObject::new(light_source, color));
                }
                FileEntry::Bulb { x, y, z } => {
                    let light_source = LightPrimitive::Point(Point3::new(*x, *y, *z));
                    light_sources.push(LightSourceObject::new(light_source, color));
                }
                // Materials
                FileEntry::Color { r, g, b } => {
                    color = Vector3::new(*r, *g, *b);
                    material.color = color;
                }
                FileEntry::Shiny { s } => {
                    material.shininess = *s;
                }
                FileEntry::Bounces { b } => {

                }
                // settings
                FileEntry::Eye { x, y, z } => {
                    let eye = Point3::new(*x, *y, *z);
                    camera_settings.position = eye;
                }
                FileEntry::Forward { x, y, z } => {
                    panic!("Forward objects are not supported");
                    let p = camera_settings.up.cross(&camera_settings.forward);
                    let forward = Vector3::new(*x, *y, *z);
                    // let
                    let right = forward.cross(&camera_settings.up);
                    camera_settings.forward = forward;
                    camera_settings.right = right;
                }
                FileEntry::Up { x, y, z } => {
                    let up = Vector3::new(*x, *y, *z);
                    let right = camera_settings.forward.cross(&up);
                    camera_settings.up = up;
                    camera_settings.right = right;
                }
                FileEntry::Expose { v: _ } => {}
            };
        }
        Ok(Self {
            objects,
            light_sources,
            camera_settings,
        })
    }
}
