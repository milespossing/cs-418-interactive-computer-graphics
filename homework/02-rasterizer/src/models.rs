use std::fmt::Display;

use nalgebra::SVector;

// Vectorizes and Devectorizes a type before it is interpolated between instances.
pub trait InterpolationConverter<T, U, const N: usize> {
    // adds w correction and turns into a vector
    fn to_vector(&self, data: T, width: U, height: U) -> SVector<U, N>;
    fn to_fragment(&self, vec: &SVector<U, N>) -> Fragment;
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub w: f32,
    pub transform: [f32; 3],
    pub rgb: [f32; 3],
    pub alpha: f32,
}

impl Vertex {
    pub fn from_xyzw_rgba(xyzw: [f32; 4], rgb: [f32; 3], alpha: f32) -> Self {
        Vertex {
            w: xyzw[3],
            transform: [xyzw[0], xyzw[1], xyzw[2]],
            rgb,
            alpha,
        }
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transform: ({}, {}, {}, {}); rgba: ({}, {}, {}, {});",
            self.transform[0],
            self.transform[1],
            self.transform[2],
            self.w,
            self.rgb[0],
            self.rgb[1],
            self.rgb[2],
            self.alpha,
        )
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
            && self.transform == other.transform
            && self.rgb == other.rgb
            && self.alpha == other.alpha
    }
}

pub type Triangle = [Vertex; 3];

// An entry in the input file
#[derive(Debug)]
pub enum Entry {
    Xyzw([f32; 4]),
    Rgb([f32; 3]),
    Rgba([f32; 4]),
    Triangle([i8; 3]),
    Comment,
    Depth,
    Srgb,
    Hyp,
}

#[derive(Debug)]
pub struct FileHeader {
    pub size: (u32, u32),
    pub name: String,
}

#[derive(Debug)]
pub struct File {
    pub header: FileHeader,
    pub triangles: Vec<Triangle>,
    pub depth: bool,
    pub srgb: bool,
    pub hyp: bool,
}

#[derive(Clone)]
pub struct Fragment {
    pub transform: SVector<u32, 2>,
    pub depth: f32,
    pub color: SVector<f32, 3>,
    pub alpha: f32,
}

impl Fragment {
    pub fn empty() -> Fragment {
        Fragment {
            transform: SVector::<u32, 2>::zeros(),
            depth: 0f32,
            color: SVector::<f32, 3>::zeros(),
            alpha: 0f32,
        }
    }

    pub fn blend(dest: &Fragment, sor: &Fragment) -> Fragment {
        let alpha = sor.alpha + dest.alpha * (1f32 - sor.alpha);
        let color =
            sor.color * sor.alpha / alpha + ((1f32 - sor.alpha) * dest.alpha * dest.color / alpha);
        Fragment {
            transform: sor.transform,
            depth: sor.depth,
            color,
            alpha,
        }
    }

    pub fn get_transform(&self) -> (usize, usize) {
        let x = self.transform[0] as usize;
        let y = self.transform[1] as usize;
        (x, y)
    }
}

#[cfg(test)]
mod fragment_tests {
    use crate::models::Fragment;
    use nalgebra::SVector;

    #[test]
    fn blends_correctly() {
        let frag1 = Fragment {
            transform: SVector::<u32, 2>::zeros(),
            depth: 0f32,
            color: SVector::<f32, 3>::from_vec(vec![255f32, 125f32, 0f32]),
            alpha: 0.7,
        };
        let frag2 = Fragment {
            transform: SVector::<u32, 2>::zeros(),
            depth: 0f32,
            color: SVector::<f32, 3>::from_vec(vec![0f32, 125f32, 255f32]),
            alpha: 0.5,
        };
        let result = Fragment::blend(&frag1, &frag2);
        assert_eq!(result.alpha, 0.85);
        assert_eq!(
            result.color,
            SVector::<f32, 3>::from_vec(vec![150f32, 95.58823529, 45f32])
        );
    }
}

impl Display for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transform:{}Color:{}Alpha: {}",
            self.transform, self.color, self.alpha
        )
    }
}
