use image::{ImageBuffer, Rgba};
use nalgebra::SVector;

use crate::{
    interpolation::perform_scanline_array,
    models::{Fragment, Triangle, Vertex},
};

pub struct RendererSettings {
    pub width: f32,
    pub height: f32,
    pub depth: bool,
    pub srgb: bool,
    pub hyp: bool,
    pub fssa: u8,
}

pub struct Renderer {
    pub frame_buffer: Vec<Vec<Fragment>>,
    pub depth_buffer: Vec<Vec<f32>>,
    settings: RendererSettings,
}

fn srgb_to_linear(srgb: f32) -> f32 {
    if srgb <= 0.04045 {
        return srgb / 12.92;
    } else {
        return ((srgb + 0.055) / 1.055).powf(2.4);
    }
}

fn linear_to_srgb(linear: f32) -> f32 {
    if linear <= 0.0031308 {
        return linear * 12.92;
    } else {
        return (1.055 * (linear.powf(1f32 / 2.4))) - 0.055;
    }
}

impl Renderer {
    pub fn from_settings(settings: RendererSettings) -> Self {
        let uwidth = settings.width as usize;
        let uheight = settings.height as usize;
        let frame_buffer: Vec<Vec<Fragment>> = vec![vec![Fragment::empty(); uwidth]; uheight];
        let depth_buffer: Vec<Vec<f32>> = vec![vec![f32::INFINITY; uwidth]; uheight];
        Renderer {
            frame_buffer,
            depth_buffer,
            settings,
        }
    }

    fn pick_colors_from_vertex(&self, data: Vertex) -> [f32; 3] {
        let colors = [data.rgb[0], data.rgb[1], data.rgb[2]];
        match self.settings.srgb {
            true => colors.map(|c| srgb_to_linear(c / 255f32)),
            false => colors,
        }
    }

    fn vertex_to_vector(&self, data: Vertex) -> SVector<f32, 8> {
        let x_map = ((data.transform[0] / data.w) + 1f32) * (self.settings.width / 2f32);
        let y_map = ((data.transform[1] / data.w) + 1f32) * (self.settings.height / 2f32);
        let w_map = 1f32 / data.w;
        let color = self.pick_colors_from_vertex(data);
        // NTODO: Doesn't matter right now; will matter for hyp
        let (corr_color, corr_alpha, z) = match self.settings.hyp {
            true => (
                color.map(|c| c * w_map),
                data.alpha * w_map,
                data.transform[2] * w_map,
            ),
            false => (color, data.alpha, data.transform[2]),
        };
        SVector::from_vec(vec![
            w_map,
            x_map,
            y_map,
            z,
            corr_color[0],
            corr_color[1],
            corr_color[2],
            corr_alpha,
        ])
    }

    fn point_to_fragment(&self, vec: &SVector<f32, 8>) -> Fragment {
        let w_corr = vec[0];
        let transform: SVector<u32, 2> = SVector::from_vec(vec![vec[1] as u32, vec[2] as u32]);

        let c = SVector::<f32, 3>::from_vec(vec![vec[4], vec[5], vec[6]]);

        let (colors_corr, alpha, depth): (SVector<f32, 3>, f32, f32) = match self.settings.hyp {
            false => (c, vec[7], vec[3]),
            true => (c.map(|c| c / w_corr), vec[7] / w_corr, vec[3] / w_corr),
        };

        // let color: SVector<f32, 3> = match self.settings.srgb {
        //     true => colors_corr.map(|c| linear_to_srgb(c) * 255f32),
        //     false => colors_corr,
        // };

        Fragment {
            transform,
            color: colors_corr,
            depth,
            alpha,
        }
    }

    pub fn run(&mut self, triangles: Vec<Triangle>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        for triangle in triangles {
            let vertex: [SVector<f32, 8>; 3] = triangle.map(|v| self.vertex_to_vector(v));
            let fragments: Vec<Fragment> = perform_scanline_array(vertex)
                .iter()
                .map(|p| self.point_to_fragment(p))
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
                let current = &self.frame_buffer[y][x];
                let new_fragment = Fragment::blend(current, &fragment);
                self.frame_buffer[y][x] = new_fragment;
            }
        }

        // creates the image
        let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(
            self.settings.width as u32,
            self.settings.height as u32,
            Rgba([0u8; 4]),
        );

        // writes the image
        for (idy, row) in self.frame_buffer.iter().enumerate() {
            for (idx, fragment) in row.iter().enumerate() {
                // need to correct
                let color_corr = match self.settings.srgb {
                    true => fragment.color.map(|c| linear_to_srgb(c) * 255f32),
                    false => fragment.color,
                };
                let color = [
                    color_corr[0] as u8,
                    color_corr[1] as u8,
                    color_corr[2] as u8,
                    (fragment.alpha * 255f32) as u8,
                ];
                image.put_pixel(idx as u32, idy as u32, Rgba(color));
            }
        }

        // returns the image
        image
    }
}
