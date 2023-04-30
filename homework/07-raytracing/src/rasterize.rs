use crate::parser::ProcFile;
use crate::renderer::RendererOutput;
use crate::utils::vec4_to_rgb;
use image::{ImageBuffer, Rgba};
use nalgebra::{Vector3, Vector4};

pub struct RasterizerSettings {
    width: u32,
    height: u32,
    aa: usize,
    exposure: Option<f64>,
}

pub struct Rasterizer {
    settings: RasterizerSettings,
}

fn get_averaged_color(
    image: &Vec<Vec<Vector4<f64>>>,
    from_x: usize,
    from_y: usize,
    stride: usize,
) -> Vector4<f64> {
    let mut fragments: Vec<Vector4<f64>> = vec![];
    let start_x = from_x * stride;
    let start_y = from_y * stride;
    for y in start_y..start_y + stride {
        for x in start_x..start_x + stride {
            fragments.push(image[y][x]);
        }
    }
    let (rgb, a): (Vec<Vector3<f64>>, Vec<f64>) = fragments
        .iter()
        .map(|v| (Vector3::<f64>::from_vec(vec![v[0], v[1], v[2]]), v[3]))
        .unzip();
    let alpha_sum: f64 = a.iter().sum();
    let rgb_average: Vector3<f64> = rgb.iter().sum::<Vector3<f64>>() / alpha_sum;
    let alpha = alpha_sum / fragments.len() as f64;
    Vector4::<f64>::new(rgb_average[0], rgb_average[1], rgb_average[2], alpha)

    // let alpha_sum = fragments.iter().fold(0.0, |a, b| a + b.w);
    // fragments
    //     .iter()
    //     .map(|v| v.map_with_location(|x,y,z| if x < 3 { z * v[3] } else { z }))
    //     .fold(Vector4::zeros(), |a, b| a + b)//.add(b))
    //     .map_with_location(|x,y,z| if x < 3 { z / alpha_sum } else { z / fragments.len() as f64 })
}

impl Rasterizer {
    pub fn new(file: &ProcFile) -> Rasterizer {
        let aa = file.get_aa();
        let exposure = file.get_exposure();
        let settings = RasterizerSettings {
            width: file.header.width,
            height: file.header.height,
            aa,
            exposure,
        };
        Rasterizer { settings }
    }

    pub fn rasterize(
        &self,
        rendered: RendererOutput,
    ) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, String> {
        let mut image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_pixel(
            self.settings.width,
            self.settings.height,
            Rgba::<u8>([0; 4]),
        );
        let buffer = rendered.into_alpha();
        for idy in 0..self.settings.height {
            for idx in 0..self.settings.width {
                let averaged = get_averaged_color(
                    &buffer,
                    idx.try_into().unwrap(),
                    idy.try_into().unwrap(),
                    self.settings.aa,
                );
                let pixel = vec4_to_rgb(averaged, self.settings.exposure);
                image.put_pixel(idx, idy, pixel);
            }
        }
        Ok(image)
    }
}
