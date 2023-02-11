mod from_file;
mod from_string;
mod interpolation;
mod models;
mod rasterizer;
mod renderer;
mod vertex_converter;

use from_file::from_file;
use models::File;
use renderer::{Renderer, RendererSettings};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: String = args[1].to_owned();
    let file = from_file::<File>(std::path::Path::new(&file_name)).unwrap();
    let settings: RendererSettings = RendererSettings {
        width: file.header.size.0,
        height: file.header.size.1,
        depth: file.depth,
        srgb: file.srgb,
        hyp: file.hyp,
    };
    let mut renderer: Renderer = Renderer::from_settings(settings);
    let image = renderer.run(file.triangles);
    image.save(file.header.name).expect("Failed to save file");
}

#[cfg(test)]
mod test {
    use crate::renderer::{Renderer, RendererSettings};

    use super::models::Vertex;

    #[test]
    fn double_triangle_test() {
        let v1: Vertex =
            Vertex::from_xyzw_rgba([1f32, 3.5, 3f32, 4f32], [255f32, 255f32, 255f32, 255f32]);
        let v2: Vertex = Vertex::from_xyzw_rgba(
            [-1f32, -2f32, -3f32, 4f32],
            [255f32, 255f32, 255f32, 255f32],
        );
        let v3: Vertex =
            Vertex::from_xyzw_rgba([2f32, 0f32, 0f32, 2f32], [0f32, 0f32, 0f32, 255f32]);
        let v4: Vertex =
            Vertex::from_xyzw_rgba([-1f32, 0.5, 0f32, 1f32], [0f32, 0f32, 0f32, 255f32]);
        let mut renderer = Renderer::from_settings(RendererSettings {
            width: 20u32,
            height: 30u32,
            depth: false,
            srgb: false,
            hyp: false,
        });
        let triangle1 = [v1, v3, v2];
        let triangle2 = [v1, v3, v4];
        let image = renderer.run(vec![triangle1, triangle2]);
        image.save(format!("test.png")).expect("Failed to save");
    }
}
