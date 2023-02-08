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
    pub rgba: [f32; 4],
}

impl Vertex {
    pub fn from_xyzw_rgba(xyzw: [f32; 4], rgba: [f32; 4]) -> Self {
        Vertex {
            w: xyzw[3],
            transform: [xyzw[0], xyzw[1], xyzw[2]],
            rgba,
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w && self.transform == other.transform && self.rgba == other.rgba
    }
}

pub struct VertexConverter;

impl InterpolationConverter<Vertex, f32, 8> for VertexConverter {
    fn to_vector(&self, data: Vertex, width: f32, height: f32) -> SVector<f32, 8> {
        let x_map = ((data.transform[0] / data.w) + 1f32) * (width / 2f32);
        let y_map = ((data.transform[1] / data.w) + 1f32) * (height / 2f32);
        let w_map = 1f32 / data.w;
        SVector::from_vec(vec![
            w_map,
            x_map,
            y_map,
            data.transform[2],
            data.rgba[0] * w_map,
            data.rgba[1] * w_map,
            data.rgba[2] * w_map,
            data.rgba[3] * w_map,
        ])
    }

    fn to_fragment(&self, vec: &SVector<f32, 8>) -> Fragment {
        let w_corr = vec[0];
        // TODO: Might need to add some error checking here
        let transform: SVector<u32, 3> =
            SVector::from_vec(vec![vec[1] as u32, vec[2] as u32, vec[3] as u32]);

        let color: SVector<f32, 4> =
            SVector::from_vec(vec![vec[4], vec[5], vec[6], vec[7]]) / w_corr;
        Fragment { transform, color }
    }
}

pub type Triangle = [Vertex; 3];

// An entry in the input file
#[derive(Debug)]
pub enum Entry {
    Xyzw([f32; 4]),
    Rgb([f32; 3]),
    Triangle([i8; 3]),
    Comment,
    Depth,
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
}

#[derive(Clone)]
pub struct Fragment {
    pub transform: SVector<u32, 3>,
    pub color: SVector<f32, 4>,
}

impl Fragment {
    pub fn empty() -> Fragment {
        Fragment {
            transform: SVector::<u32, 3>::zeros(),
            color: SVector::<f32, 4>::zeros(),
        }
    }

    // TODO: Rename to over/under
    pub fn blend(a: &Fragment, b: &Fragment) -> Fragment {
        Fragment {
            transform: a.transform,
            // using 'over' operator
            color: b.color + (255f32 - b.color[3]) * a.color,
        }
    }

    pub fn get_transform(&self) -> (usize, usize) {
        let x = self.transform[0] as usize;
        let y = self.transform[1] as usize;
        (x, y)
    }
}

impl Display for Fragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transform:{}Color:{}", self.transform, self.color)
    }
}
