use std::ops::Add;
use nalgebra::{Point3, Vector3};
use crate::parser::ProcFile;
use crate::raytracer::{Ray, RayTracer};
use crate::scene::Scene;

pub type Rgba = [f32; 4];

#[derive(Debug)]
pub struct RendererOutput {
    pub pixel_buffer: Vec<Vec<Rgba>>,
}

impl RendererOutput {
    pub fn new(width: usize, height: usize) -> RendererOutput {
        let default_pixel = [0.0, 0.0, 0.0, 0.0];
        let mut pixel_buffer: Vec<Vec<Rgba>> = Vec::new();
        for _i in 0..height {
            let mut row: Vec<Rgba> = Vec::new();
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

pub struct Renderer {
    scene: Scene,
    options: RendererOptions,
}

struct RendererOptions {
    eye: Point3<f64>,
    forward: Vector3<f64>,
    right: Vector3<f64>,
    up: Vector3<f64>,
    width: usize,
    height: usize,
}

impl RendererOptions {
    fn from_file(file: &ProcFile) -> Result<Self, String> {
        Ok(RendererOptions {
            eye: Point3::new(0.0, 0.0, 0.0),
            forward: Vector3::new(0.0, 0.0, -1.0),
            right: Vector3::new(1.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            width: file.header.width as usize,
            height: file.header.height as usize,
        })
    }
}

type Position = (usize, usize);

fn initialize_rays(options: &RendererOptions) -> Vec<(Ray, Position)> {
    let w: f64 = options.width as f64;
    let h: f64 = options.height as f64;
    let mut rays: Vec<(Ray, Position)> = Vec::new();
    for y in 0..options.height {
        let s_y = (h - 2.0 * y as f64) / w.max(h);
        for x in 0..options.width {
            let s_x = (2.0 * x as f64 - w) / w.max(h);
            rays.push((Ray::new(
                options.eye,
                options.forward.add(options.right.scale(s_x)).add(options.up.scale(s_y)).normalize(),
            ), (x, y)));
        }
    }
    return rays;
}

impl Renderer {
    pub fn from_file(file: &ProcFile) -> Result<Self, String> {
        let scene = Scene::from_file(file)?;
        let options = RendererOptions::from_file(file)?;
        Ok(Self { scene, options })
    }

    pub fn render_scene(&self) -> Result<RendererOutput, String> {
        let ray_tracer = RayTracer::new(&self.scene.objects);
        let rays = initialize_rays(&self.options);
        let mut output = RendererOutput::new(self.options.width, self.options.height);
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        for ray in rays.iter() {
            match ray_tracer.trace_ray(&ray.0) {
                Some(_) => {
                    output.pixel_buffer[ray.1.1][ray.1.0] = BLACK;
                },
                None => {},
            }
        }

        Ok(output)
    }
}
