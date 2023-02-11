use crate::models::{Fragment, InterpolationConverter, Vertex};
use nalgebra::SVector;

// TODO: Hyp would be pretty easy here.

pub struct VertexConverter {
    use_linear: bool,
    use_hyp: bool,
}

impl VertexConverter {
    pub fn new(use_linear: bool, use_hyp: bool) -> Self {
        Self {
            use_linear,
            use_hyp,
        }
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
        let (corr_color, corr_alpha, z) = match self.use_hyp {
            true => (
                color.map(|c| c * w_map),
                data.rgba[3] * w_map,
                data.transform[2] * w_map,
            ),
            false => (color, data.rgba[3], data.transform[2]),
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

    fn to_fragment(&self, vec: &SVector<f32, 8>) -> Fragment {
        let w_corr = vec[0];
        let transform: SVector<u32, 2> = SVector::from_vec(vec![vec[1] as u32, vec[2] as u32]);

        let c = [vec[4], vec[5], vec[6]];

        let (colors_corr, alpha, depth): ([f32; 3], f32, f32) = match self.use_hyp {
            false => (c, vec[7], vec[3]),
            true => (c.map(|c| c / w_corr), vec[7] / w_corr, vec[3] / w_corr),
        };

        let color = match self.use_linear {
            true => colors_corr.map(|c| linear_to_srgb(c) * 255f32),
            false => colors_corr,
        };

        let rgba = SVector::<f32, 4>::from_vec(vec![color[0], color[1], color[2], alpha]);

        Fragment {
            transform,
            color: rgba,
            depth,
        }
    }
}
