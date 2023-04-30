use crate::lighting_models::LightingModel;
use crate::parser::{FileEntry, ProcFile};
use crate::raytracer::{Ray, RayHit, RayTracer};
use crate::scene::{CameraSettings, Scene};
use crate::utils::{vec3_add_alpha, BLACK};
use nalgebra::{Vector3, Vector4};
use std::ops::Add;

#[derive(Debug)]
pub struct RendererOutput {
    pub pixel_buffer: Vec<Vec<Option<Vector3<f64>>>>,
}

impl RendererOutput {
    pub fn new(width: usize, height: usize) -> RendererOutput {
        let default_pixel = None;
        let mut pixel_buffer: Vec<Vec<Option<Vector3<f64>>>> = Vec::new();
        for _i in 0..height {
            let mut row: Vec<Option<Vector3<f64>>> = Vec::new();
            for _j in 0..width {
                row.push(default_pixel);
            }
            pixel_buffer.push(row);
        }
        RendererOutput { pixel_buffer }
    }
    pub fn into_alpha(self) -> Vec<Vec<Vector4<f64>>> {
        self.pixel_buffer
            .iter()
            .map(|row| row.iter().map(|p| vec3_add_alpha(p)).collect())
            .collect()
    }
}

pub struct Renderer<'a> {
    scene: &'a Scene,
    options: RendererOptions,
    ray_tracer: RayTracer<'a>,
    lighting_model: LightingModel<'a>,
}

struct RendererOptions {
    width: usize,
    height: usize,
    max_depth: usize,
    aa: usize,
}

impl RendererOptions {
    fn from_file(file: &ProcFile) -> Result<Self, String> {
        let bounces = file.entries.iter().find_map(|entry| match entry {
            FileEntry::Bounces { b } => Some(b),
            _ => None,
        });
        let max_depth = match bounces {
            Some(bounces) => *bounces,
            None => 4,
        };

        let aa = file.get_aa();

        Ok(RendererOptions {
            width: file.header.width as usize,
            height: file.header.height as usize,
            max_depth,
            aa,
        })
    }
}

type Position = (usize, usize);

fn initialize_rays(options: &RendererOptions, camera: &CameraSettings) -> Vec<(Ray, Position)> {
    let w: f64 = (options.width * options.aa) as f64;
    let h: f64 = (options.height * options.aa) as f64;
    let mut rays: Vec<(Ray, Position)> = Vec::new();
    for y in 0..options.height * options.aa {
        let s_y = (h - 2.0 * y as f64) / w.max(h);
        for x in 0..options.width * options.aa {
            let s_x = (2.0 * x as f64 - w) / w.max(h);
            rays.push((
                Ray::new(
                    camera.position,
                    camera
                        .forward
                        .add(camera.right.scale(s_x))
                        .add(camera.up.scale(s_y))
                        .normalize(),
                ),
                (x, y),
            ));
        }
    }
    return rays;
}

impl<'a> Renderer<'a> {
    pub fn from_file(file: &ProcFile, scene: &'a Scene) -> Result<Self, String> {
        let options = RendererOptions::from_file(file)?;
        let ray_tracer: RayTracer<'_> = RayTracer::new(&scene);
        let lighting_model = LightingModel::from_file(&file, scene);
        Ok(Self {
            scene,
            options,
            ray_tracer,
            lighting_model,
        })
    }

    // return the lit value at this position
    fn light(&self, hit: &RayHit) -> Vector3<f64> {
        let material = self.scene.get_object(hit.object_id).unwrap().material;
        let light = self.lighting_model.light(&hit);
        material.color.component_mul(&light.scale(material.albedo))
    }

    fn get_recast_ray(&self, hit: &RayHit, depth: usize) -> Vector3<f64> {
        let i = hit.direction;
        let n = hit.surface_normal;
        let d = i - (2.0 * n.dot(&i) * n);
        let new_ray = Ray::new(hit.position, d);
        match self.cast_ray(&new_ray, depth + 1) {
            Some(new_hit) => {
                if new_hit.x == 0.0 && new_hit.y == 0.0 && new_hit.z == 0.0 {
                    self.cast_ray(&new_ray, depth + 1);
                }
                new_hit
            }
            None => BLACK,
        }
    }

    fn cast_ray(&self, ray: &Ray, depth: usize) -> Option<Vector3<f64>> {
        if depth > self.options.max_depth {
            return None;
        }
        match self.ray_tracer.trace_ray(ray, None) {
            Some(hit) => {
                let material = self.scene.get_object(hit.object_id).unwrap().material;
                match material.shininess {
                    s if s == 0.0 => Some(self.light(&hit)),
                    s if s == 1.0 => Some(self.get_recast_ray(&hit, depth)),
                    s => {
                        let lit = self.light(&hit).scale(1.0 - s);
                        let bounced = self.get_recast_ray(&hit, depth).scale(s);
                        Some(lit + bounced)
                    }
                }
            }
            None => None,
        }
    }

    pub fn render_scene(&self) -> Result<RendererOutput, String> {
        let rays = initialize_rays(&self.options, &self.scene.camera_settings);
        let mut output = RendererOutput::new(
            self.options.width * self.options.aa,
            self.options.height * self.options.aa,
        );

        for (ray, (x, y)) in rays.iter() {
            match self.cast_ray(ray, 0) {
                Some(color) => {
                    output.pixel_buffer[*y][*x] = Some(color);
                    // vec3_to_rgb(&color.map(|c| match self.options.exposure {
                    //     None => c,
                    //     Some(e) => 1.0 - (-c * e).exp(),
                    // }))
                    // .to_rgba();
                }
                None => {}
            }
        }
        Ok(output)
    }
}
