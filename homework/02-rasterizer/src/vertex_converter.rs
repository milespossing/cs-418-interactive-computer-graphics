use crate::models::{Fragment, InterpolationConverter, Vertex};
use nalgebra::SVector;

pub struct VertexConverter {
    use_linear: bool,
}

impl VertexConverter {
    pub fn new(use_linear: bool) -> Self {
        Self { use_linear }
    }
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

impl InterpolationConverter<Vertex, f32, 8> for VertexConverter {
    fn to_vector(&self, data: Vertex, width: f32, height: f32) -> SVector<f32, 8> {
        let x_map = ((data.transform[0] / data.w) + 1f32) * (width / 2f32);
        let y_map = ((data.transform[1] / data.w) + 1f32) * (height / 2f32);
        let w_map = 1f32 / data.w;
        let colors = [data.rgba[0], data.rgba[1], data.rgba[2]];
        let color = match self.use_linear {
            true => colors.map(|c| srgb_to_linear(c / 255f32)),
            false => colors,
        };
        SVector::from_vec(vec![
            w_map,
            x_map,
            y_map,
            data.transform[2],
            color[0] * w_map,
            color[1] * w_map,
            color[2] * w_map,
            data.rgba[3] * w_map,
        ])
    }

    fn to_fragment(&self, vec: &SVector<f32, 8>) -> Fragment {
        let w_corr = vec[0];
        // TODO: Might need to add some error checking here
        let transform: SVector<u32, 2> = SVector::from_vec(vec![vec[1] as u32, vec[2] as u32]);

        let depth: f32 = vec[3];

        let colors_corr: [f32; 4] = [vec[4], vec[5], vec[6], vec[7]].map(|c| c / w_corr);
        let colors = [colors_corr[0], colors_corr[1], colors_corr[2]];

        let color = match self.use_linear {
            true => colors.map(|c| linear_to_srgb(c) * 255f32),
            false => colors,
        };

        let rgba = SVector::<f32, 4>::from_vec(vec![color[0], color[1], color[2], colors_corr[3]]);

        Fragment {
            transform,
            color: rgba,
            depth,
        }
    }
}
