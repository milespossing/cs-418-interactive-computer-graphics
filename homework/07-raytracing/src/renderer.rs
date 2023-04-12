use std::ops::Add;
use image::{Pixel, Rgba};
use nalgebra::{Vector3};
use uuid::Uuid;
use crate::lighting_models::LightingModel;
use crate::models::Material;
use crate::parser::ProcFile;
use crate::raytracer::{Ray, RayHit, RayTracer};
use crate::scene::{CameraSettings, Scene};
use crate::utils::vec3_to_rgb;


#[derive(Debug)]
pub struct RendererOutput {
    pub pixel_buffer: Vec<Vec<Rgba<u8>>>,
}

impl RendererOutput {
    pub fn new(width: usize, height: usize) -> RendererOutput {
        let default_pixel = Rgba([0, 0, 0, 0]);
        let mut pixel_buffer: Vec<Vec<Rgba<u8>>> = Vec::new();
        for _i in 0..height {
            let mut row: Vec<Rgba<u8>> = Vec::new();
            for _j in 0..width {
                row.push(default_pixel);
            }
            pixel_buffer.push(row);
        }
        RendererOutput {
            pixel_buffer,
        }
    }
}

pub struct Renderer<'a> {
    scene: &'a Scene,
    options: RendererOptions,
    ray_tracer: RayTracer<'a>,
    lighting_model: LightingModel<'a>
}

struct RendererOptions {
    width: usize,
    height: usize,
    max_depth: usize,
}

impl RendererOptions {
    fn from_file(file: &ProcFile) -> Result<Self, String> {
        Ok(RendererOptions {
            width: file.header.width as usize,
            height: file.header.height as usize,
            max_depth: 1,
        })
    }
}

type Position = (usize, usize);

fn initialize_rays(options: &RendererOptions, camera: &CameraSettings) -> Vec<(Ray, Position)> {
    let w: f64 = options.width as f64;
    let h: f64 = options.height as f64;
    let mut rays: Vec<(Ray, Position)> = Vec::new();
    for y in 0..options.height {
        let s_y = (h - 2.0 * y as f64) / w.max(h);
        for x in 0..options.width {
            let s_x = (2.0 * x as f64 - w) / w.max(h);
            rays.push((Ray::new(
                camera.position,
                camera.forward.add(camera.right.scale(s_x)).add(camera.up.scale(s_y)).normalize(),
            ), (x, y)));
        }
    }
    return rays;
}

impl<'a> Renderer<'a> {
    pub fn from_file(file: &ProcFile, scene: &'a Scene) -> Result<Self, String> {
        let options = RendererOptions::from_file(file)?;
        let ray_tracer: RayTracer<'_> = RayTracer::new(&scene);
        let lighting_model = LightingModel::from_file(&file, scene);
        Ok(Self { scene, options, ray_tracer, lighting_model })
    }

    fn get_material(&self, id: Uuid) -> Option<&Material> {
        match self.scene.objects.iter().find(|&o| o.id == id) {
            Some(o) => Some(&o.material),
            None => None
        }
    }

    // return the lit value at this position
    fn light(&self, hit: &RayHit) -> Vector3<f64> {
        let material = self.get_material(hit.object_id).unwrap();
        let light = self.lighting_model.light(&hit);
        material.color.component_mul(&light.scale(material.albedo))
    }

    fn recast(&self, _hit: &RayHit) -> Vector3<f64> {
        unimplemented!()
    }

    fn cast_ray(&self, ray: &Ray, depth: usize) -> Option<Vector3<f64>> {
        match self.ray_tracer.trace_ray(ray) {
            Some(hit) => {
                if depth == self.options.max_depth - 1{
                    Some(self.light(&hit))
                } else {
                    Some(self.recast(&hit))
                }
            },
            None => None,
        }
    }

    pub fn render_scene(&self) -> Result<RendererOutput, String> {
        let rays = initialize_rays(&self.options, &self.scene.camera_settings);
        let mut output = RendererOutput::new(self.options.width, self.options.height);

        for (ray, (x,y)) in rays.iter() {
            match self.cast_ray(ray, 0) {
                Some(color) => {
                    output.pixel_buffer[*y][*x] = vec3_to_rgb(&color).to_rgba();
                },
                None => {},
            }
        }

        Ok(output)
    }
}
