use crate::models::{
    LightPrimitive, LightSourceObject, Material, ObjPrimative, SceneObject, AABB, DEFAULT_COLOR,
    DEFAULT_MATERIAL,
};
use crate::parser::{FileEntry, ProcFile};
use nalgebra::{Point3, Vector3};
use std::ops::{Div, Sub};
use uuid::Uuid;

pub const MAX_OBJECTS: usize = 20;

#[derive(Debug, Clone)]
pub struct BoundingVolume {
    pub aabb: AABB,
    pub children: Vec<SceneObject>,
}

#[derive(Debug, Clone)]
pub struct BVHNode {
    pub bounding_volume: BoundingVolume,
    pub children: Option<Vec<BVHNode>>,
}

impl BoundingVolume {
    fn new(aabb: AABB, children: Vec<SceneObject>) -> BoundingVolume {
        BoundingVolume { aabb, children }
    }
}

// creation
impl BVHNode {
    fn build(aabb: AABB, objects: Vec<SceneObject>) -> Self {
        println!("{}", objects.len());
        let bounding_volume = BoundingVolume::new(aabb, objects.clone());
        if objects.len() > MAX_OBJECTS {
            let children: Vec<BVHNode> = aabb
                .subdivide()
                .iter()
                .filter_map(|&v| {
                    let interior_objects: Vec<SceneObject> = objects
                        .clone()
                        .iter()
                        .filter(|&o| match o.aabb {
                            Some(o_aabb) => o_aabb.overlaps(&v),
                            None => true,
                        })
                        .map(|&o| o.clone())
                        .collect();
                    if interior_objects.len() > 0 {
                        Some(BVHNode::build(v, interior_objects))
                    } else {
                        None
                    }
                })
                .collect();
            BVHNode {
                bounding_volume,
                children: Some(children),
            }
        } else {
            BVHNode {
                bounding_volume,
                children: None,
            }
        }
    }

    pub fn from_objects(objects: Vec<SceneObject>) -> Self {
        let minx = objects
            .iter()
            .filter_map(|o| match o.aabb {
                Some(aabb) => Some(aabb.0[0].x),
                None => None,
            })
            .fold(f64::INFINITY, |m, x| m.min(x));
        let miny = objects
            .iter()
            .filter_map(|o| match o.aabb {
                Some(aabb) => Some(aabb.0[0].y),
                None => None,
            })
            .fold(f64::INFINITY, |m, y| m.min(y));
        let minz = objects
            .iter()
            .filter_map(|o| match o.aabb {
                Some(aabb) => Some(aabb.0[0].z),
                None => None,
            })
            .fold(f64::INFINITY, |m, z| m.min(z));
        let maxx = objects
            .iter()
            .filter_map(|o| match o.aabb {
                Some(aabb) => Some(aabb.0[1].x),
                None => None,
            })
            .fold(f64::NEG_INFINITY, |m, x| m.max(x));
        let maxy = objects
            .iter()
            .filter_map(|o| match o.aabb {
                Some(aabb) => Some(aabb.0[1].y),
                None => None,
            })
            .fold(f64::NEG_INFINITY, |m, x| m.max(x));
        let maxz = objects
            .iter()
            .filter_map(|o| match o.aabb {
                Some(aabb) => Some(aabb.0[1].z),
                None => None,
            })
            .fold(f64::NEG_INFINITY, |m, x| m.max(x));
        let bounding_box = AABB::new(Point3::new(minx, miny, minz), Point3::new(maxx, maxy, maxz));
        let bounding_volume = BoundingVolume::new(bounding_box, objects.clone());
        if objects.len() < MAX_OBJECTS {
            BVHNode {
                bounding_volume,
                children: None,
            }
        } else {
            let children: Vec<Self> = bounding_box
                .subdivide()
                .iter()
                .map(|v| {
                    let interior_objects: Vec<SceneObject> = objects
                        .clone()
                        .iter()
                        .filter(|&o| match o.aabb {
                            Some(o_aabb) => v.overlaps(&o_aabb),
                            None => true,
                        })
                        .map(|o| *o)
                        .collect();
                    Self::build(*v, interior_objects)
                })
                .collect();
            BVHNode {
                bounding_volume,
                children: Some(children),
            }
        }
    }
}

impl BVHNode {
    pub fn is_leaf(&self) -> bool {
        self.children.is_none()
    }
}

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
    pub bvh: BVHNode,
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
                // settings
                FileEntry::Eye { x, y, z } => {
                    let eye = Point3::new(*x, *y, *z);
                    camera_settings.position = eye;
                }
                // FileEntry::Forward { x, y, z } => {
                //     panic!("Forward objects are not supported");
                //     let p = camera_settings.up.cross(&camera_settings.forward);
                //     let forward = Vector3::new(*x, *y, *z);
                //     // let
                //     let right = forward.cross(&camera_settings.up);
                //     camera_settings.forward = forward;
                //     camera_settings.right = right;
                // }
                FileEntry::Up { x, y, z } => {
                    let up = Vector3::new(*x, *y, *z);
                    let right = camera_settings.forward.cross(&up);
                    camera_settings.up = up;
                    camera_settings.right = right;
                }
                _ => {}
            };
        }
        let bvh = BVHNode::from_objects(objects.clone());
        Ok(Self {
            objects,
            light_sources,
            camera_settings,
            bvh,
        })
    }
}
