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
}

#[derive(Clone)]
pub struct Fragment {
    pub transform: SVector<u32, 2>,
    pub depth: f32,
    pub color: SVector<f32, 4>,
}

impl Fragment {
    pub fn empty() -> Fragment {
        Fragment {
            transform: SVector::<u32, 2>::zeros(),
            depth: 0f32,
            color: SVector::<f32, 4>::zeros(),
        }
    }

    pub fn blend(under: &Fragment, over: &Fragment) -> Fragment {
        Fragment {
            transform: under.transform,
            depth: under.depth,
            // using 'over' operator
            color: over.color + (255f32 - over.color[3]) * under.color,
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
