use image::{ImageBuffer, Rgba};

use crate::{
    models::{Fragment, Triangle},
    rasterizer::BasicRasterizer,
};

pub struct RendererSettings {
    pub width: u32,
    pub height: u32,
}

pub struct Renderer {
    pub frame_buffer: Vec<Vec<Fragment>>,
    // pub image_buffer: ImageBuffer<Rgba<f32>, Vec<f32>>,
    rasterizer: BasicRasterizer,
    settings: RendererSettings,
}

impl Renderer {
    pub fn from_settings(settings: RendererSettings) -> Self {
        let frame_buffer: Vec<Vec<Fragment>> =
            vec![vec![Fragment::empty(); settings.width as usize]; settings.height as usize];
        let rasterizer = BasicRasterizer::new(settings.width, settings.height);
        Renderer {
            frame_buffer,
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
            let x = fragment.transform[0] as usize;
            let y = fragment.transform[1] as usize;
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
