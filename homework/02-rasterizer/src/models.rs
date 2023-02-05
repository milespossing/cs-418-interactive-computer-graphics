use nalgebra::SVector;

pub type Xyzw = SVector<f32, 4>;

pub type Rgb = SVector<f32, 3>;

pub type Vertex = SVector<f32, 7>;

pub type Triangle = [Vertex; 3];

// An entry in the input file
#[derive(Debug)]
pub enum Entry {
    Xyzw(Xyzw),
    Rgb(Rgb),
    Triangle([i8; 3]),
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
