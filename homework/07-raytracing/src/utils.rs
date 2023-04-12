use image::Rgb;
use nalgebra::Vector3;

pub fn clamp(t: f64) -> f64 {
    if t < 0.0 {
        0.0
    } else {
        t
    }
}

pub fn vec3_to_rgb(v: &Vector3<f64>) -> Rgb<u8> {
    let scaled = v.map(|l| match l {
        x if x <= 0.0031308 => 12.92 * x,
        x => 1.005 * x.powf(1.0 / 2.4) - 0.055,
    }).map(|x| match x {
        x if x <= 0.0 => 0u8,
        x if x > 1.0 => 255u8,
        x => (x * 255.0) as u8,
    });
    Rgb([
        scaled.x as u8,
        scaled.y as u8,
        scaled.z as u8,
    ])
}