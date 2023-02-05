use crate::{
    image_wrapper::{ImageWrapper, Pixel},
    interpolation::perform_scanline,
    models::{Triangle, Vertex},
};
use nalgebra::SVector;

pub trait Rasterizer {
    fn new(width: u32, height: u32) -> Self;
    fn rasterize(&self, triangles: &Vec<Triangle>) -> Vec<Pixel>;
}

pub struct BasicRasterizer {
    width: u32,
    height: u32,
}

fn map_vector_with_w(v: SVector<f32, 7>, width: u32, height: u32) -> SVector<f32, 7> {
    let x = v[0];
    let y = v[1];
    let w = v[3];

    let x_map = ((x / w) + 1f32) * (width / 2u32) as f32;
    let y_map = ((y / w) + 1f32) * (height / 2u32) as f32;
    let w_map = 1f32 / w;

    SVector::from_vec(vec![
        x_map,
        y_map,
        v[2],
        w_map,
        v[4] / w,
        v[5] / w,
        v[6] / w,
    ])
}

#[cfg(test)]
mod map_tests {
    use super::map_vector_with_w;
    use crate::models::Triangle;
    use nalgebra::SVector;

    #[test]
    fn maps_triangle_correctly() {
        let width: u32 = 20;
        let height: u32 = 30;
        let v1 = SVector::from_vec(vec![1f32, 3.5, 3f32, 4f32, 0f32, 0f32, 0f32]);
        let v2 = SVector::from_vec(vec![2f32, 0f32, 0f32, 2f32, 0f32, 0f32, 0f32]);
        let v3 = SVector::from_vec(vec![-1f32, -2f32, -3f32, 4f32, 0f32, 0f32, 0f32]);
        let triangle: Triangle = [v1, v2, v3];
        let triangle_c = triangle.map(|v| map_vector_with_w(v, width, height));
        assert_eq!(triangle_c[0][0], 12.5f32);
        assert_eq!(triangle_c[0][1], 28.125f32);
        assert_eq!(triangle_c[1][0], 20f32);
        assert_eq!(triangle_c[2][0], 7.5f32);
    }
}

fn vec_as_pixel(v: Vertex) -> Pixel {
    let w_cor = v[3];
    Pixel {
        x: f32::floor(v[0] - 1f32) as u32,
        y: f32::floor(v[1] - 1f32) as u32,
        c: [
            (f32::floor(v[4]) / w_cor) as u8,
            (f32::floor(v[5]) / w_cor) as u8,
            (f32::floor(v[6]) / w_cor) as u8,
            255u8,
        ],
    }
}

impl Rasterizer for BasicRasterizer {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn rasterize(&self, triangles: &Vec<Triangle>) -> Vec<Pixel> {
        let mut pixels: Vec<Pixel> = vec![];
        for triangle in triangles {
            let corrected = triangle.map(|v| map_vector_with_w(v, self.width, self.height));
            let scan = perform_scanline(corrected[0], corrected[1], corrected[2]);
            let mut new_pixels = scan.iter().map(|&v| vec_as_pixel(v)).collect();
            pixels.append(&mut new_pixels);
        }
        pixels
    }
}

pub struct RasterizationController<'a, T: Rasterizer> {
    rasterizer: T,
    image: &'a mut ImageWrapper,
}

impl<'a, T: Rasterizer> RasterizationController<'a, T> {
    pub fn new(image: &'a mut ImageWrapper) -> Self {
        let rasterizer = T::new(image.width, image.height);
        Self { rasterizer, image }
    }

    pub fn rasterize(&mut self, triangles: Vec<Triangle>) {
        let pixels = self.rasterizer.rasterize(&triangles);
        for pixel in pixels {
            self.image.put_pixel(pixel)
        }
    }
}
