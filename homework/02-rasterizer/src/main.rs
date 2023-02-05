mod from_file;
mod from_string;
mod image_wrapper;
mod interpolation;
mod models;
mod rasterizer;

use from_file::from_file;
use image_wrapper::ImageWrapper;
use models::File;
use rasterizer::{BasicRasterizer, RasterizationController};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: String = args[1].to_owned();
    let file = from_file::<File>(std::path::Path::new(&file_name)).unwrap();
    let mut image: ImageWrapper = ImageWrapper::new(file.header.size.0, file.header.size.1);
    let mut rasterizer_controller = RasterizationController::<BasicRasterizer>::new(&mut image);
    rasterizer_controller.rasterize(file.triangles);
    image.save(file.header.name).expect("Failed to save file");
}

#[cfg(test)]
mod test {
    use super::{
        image_wrapper::ImageWrapper,
        models::Vertex,
        rasterizer::{BasicRasterizer, RasterizationController},
    };
    use nalgebra::SVector;

    #[test]
    fn double_triangle_test() {
        let v1: Vertex = SVector::from_vec(vec![1f32, 3.5, 3f32, 4f32, 255f32, 255f32, 255f32]);
        let v2: Vertex = SVector::from_vec(vec![-1f32, -2f32, -3f32, 4f32, 255f32, 255f32, 255f32]);
        let v3: Vertex = SVector::from_vec(vec![2f32, 0f32, 0f32, 2f32, 0f32, 0f32, 0f32]);
        let v4: Vertex = SVector::from_vec(vec![-1f32, 0.5, 0f32, 1f32, 0f32, 0f32, 0f32]);
        let mut image: ImageWrapper = ImageWrapper::new(20, 30);
        let mut rasterizer_controller = RasterizationController::<BasicRasterizer>::new(&mut image);
        let triangle1 = [v1, v3, v2];
        let triangle2 = [v1, v3, v4];
        rasterizer_controller.rasterize(vec![triangle1, triangle2]);
        image.save(format!("test.png")).expect("Failed to save");
    }
}
