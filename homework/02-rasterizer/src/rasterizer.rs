use crate::{
    interpolation::perform_scanline,
    models::{Fragment, InterpolationConverter, Triangle},
    vertex_converter::VertexConverter,
};

pub trait Rasterizer {
    fn new(width: u32, height: u32) -> Self;
    fn rasterize(&self, triangles: Triangle) -> Vec<Fragment>;
}

pub struct BasicRasterizer {
    width: u32,
    height: u32,
    srgb: bool,
    hyp: bool,
}

#[cfg(test)]
mod map_tests {
    use crate::{
        models::{InterpolationConverter, Vertex},
        vertex_converter::VertexConverter,
    };

    #[test]
    fn maps_triangle_correctly() {
        let conv = VertexConverter::new(false, false);
        let width: f32 = 20f32;
        let height: f32 = 30f32;
        let v1 = Vertex::from_xyzw_rgba([1f32, 3.5, 3f32, 4f32], [0f32, 0f32, 0f32, 255f32]);
        let v2 = Vertex::from_xyzw_rgba([2f32, 0f32, 0f32, 2f32], [0f32, 0f32, 0f32, 255f32]);
        let v3 = Vertex::from_xyzw_rgba([-1f32, -2f32, -3f32, 4f32], [0f32, 0f32, 0f32, 255f32]);
        let c1 = conv.to_vector(v1, width, height);
        let c2 = conv.to_vector(v2, width, height);
        let c3 = conv.to_vector(v3, width, height);
        assert_eq!(c1[1], 12.5f32);
        assert_eq!(c1[2], 28.125f32);
        assert_eq!(c2[1], 20f32);
        assert_eq!(c3[1], 7.5f32);
    }
}

impl BasicRasterizer {
    pub fn new(width: u32, height: u32, srgb: bool, hyp: bool) -> Self {
        Self {
            width,
            height,
            hyp,
            srgb,
        }
    }

    pub fn rasterize(&self, t: Triangle) -> Vec<Fragment> {
        let conv = VertexConverter::new(self.srgb, self.hyp);
        perform_scanline(
            conv.to_vector(t[0], self.width as f32, self.height as f32),
            conv.to_vector(t[1], self.width as f32, self.height as f32),
            conv.to_vector(t[2], self.width as f32, self.height as f32),
        )
        .iter()
        .map(|v| conv.to_fragment(v))
        .collect()
    }
}
