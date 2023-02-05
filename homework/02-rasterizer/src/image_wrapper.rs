use image::{ImageBuffer, ImageError, Rgba};

pub struct ImageWrapper {
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub width: u32,
    pub height: u32,
}

pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub c: [u8; 4],
}

impl ImageWrapper {
    pub fn new(width: u32, height: u32) -> Self {
        let img = image::RgbaImage::from_pixel(width, height, Rgba([0, 0, 0, 0]));
        Self {
            image: img,
            width,
            height,
        }
    }

    pub fn put_pixel(&mut self, p: Pixel) {
        self.image.put_pixel(p.x, p.y, Rgba(p.c));
    }

    pub fn save(&self, path: String) -> Result<(), ImageError> {
        self.image.save(path)
    }
}
