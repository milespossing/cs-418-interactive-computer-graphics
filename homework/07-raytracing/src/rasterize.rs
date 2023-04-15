use crate::parser::ProcFile;
use crate::renderer::RendererOutput;
use image::{ImageBuffer, Rgba};

pub struct RasterizerSettings {
    width: u32,
    height: u32,
}

pub struct Rasterizer {
    settings: RasterizerSettings,
}

impl Rasterizer {
    pub fn new(file: &ProcFile) -> Rasterizer {
        let settings = RasterizerSettings {
            width: file.header.width,
            height: file.header.height,
        };
        Rasterizer { settings }
    }

    pub fn rasterize(
        &self,
        rendered: &RendererOutput,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
        let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_pixel(self.settings.width, self.settings.height, Rgba([0u8; 4]));
        for (idy, row) in rendered.pixel_buffer.iter().enumerate() {
            for (idx, fragment) in row.iter().enumerate() {
                image.put_pixel(idx as u32, idy as u32, *fragment);
            }
        }
        Ok(image)
    }
}
