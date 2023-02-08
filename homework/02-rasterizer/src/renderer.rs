use image::{ImageBuffer, Rgba};

use crate::{
    models::{Fragment, Triangle},
    rasterizer::BasicRasterizer,
};

pub struct RendererSettings {
    pub width: u32,
    pub height: u32,
    pub depth: bool,
}

pub struct Renderer {
    pub frame_buffer: Vec<Vec<Fragment>>,
    pub depth_buffer: Vec<Vec<f32>>,
    // pub image_buffer: ImageBuffer<Rgba<f32>, Vec<f32>>,
    rasterizer: BasicRasterizer,
    settings: RendererSettings,
}

impl Renderer {
    pub fn from_settings(settings: RendererSettings) -> Self {
        let uwidth = settings.width as usize;
        let uheight = settings.height as usize;
        let frame_buffer: Vec<Vec<Fragment>> = vec![vec![Fragment::empty(); uwidth]; uheight];
        let depth_buffer: Vec<Vec<f32>> = vec![vec![f32::INFINITY; uwidth]; uheight];
        let rasterizer = BasicRasterizer::new(settings.width, settings.height);
        Renderer {
            frame_buffer,
            depth_buffer,
            rasterizer,
            settings,
        }
    }

    pub fn run(&mut self, triangles: Vec<Triangle>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let fragments: Vec<Fragment> = triangles
            .iter()
            .map(|&t| self.rasterizer.rasterize(t))
            .flatten()
            .collect();

        for fragment in fragments {
            let (x, y) = fragment.get_transform();
            if self.settings.depth {
                let current_depth = self.depth_buffer[y][x];
                if fragment.depth >= current_depth || fragment.depth < -1f32 {
                    continue;
                }
                self.depth_buffer[y][x] = fragment.depth;
            }
            let new_fragment = Fragment::blend(&self.frame_buffer[y][x], &fragment);
            self.frame_buffer[y][x] = new_fragment;
        }

        let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_pixel(self.settings.width, self.settings.height, Rgba([0u8; 4]));

        for (idy, row) in self.frame_buffer.iter().enumerate() {
            for (idx, fragment) in row.iter().enumerate() {
                let color = [
                    fragment.color[0] as u8,
                    fragment.color[1] as u8,
                    fragment.color[2] as u8,
                    fragment.color[3] as u8,
                ];
                image.put_pixel(idx as u32, idy as u32, Rgba(color));
            }
        }
        image
    }
}
