use image::{Rgba};
use nalgebra::{Vector3, Vector4};

pub const BLACK: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);

pub fn clamp(t: f64) -> f64 {
    if t < 0.0 {
        0.0
    } else {
        t
    }
}

pub fn vec4_to_rgb(v: Vector4<f64>, exposure: Option<f64>) -> Rgba<u8> {
    let rgb = Vector3::<f32>::new(v[0] as f32, v[1] as f32, v[2] as f32).map(|c| match exposure {
        Some(e) => 1.0 - (-c * e as f32).exp(),
        None => c,
    }).map(|v| match v {
        x if x <= 0.0031308 => x * 12.92,
        x => 1.055 * x.powf(1.0 / 2.4) - 0.055,
    }).map(|v| match v {
        x if x < 0.0 => 0u8,
        x if x > 1.0 => 255u8,
        x => (x * 255.0) as u8,
    });
    let a = (v[3] * 255.0) as u8;
    Rgba([rgb[0], rgb[1], rgb[2], a])
    // Rgba::<u8>([v[0] as f32, v[1] as f32, v[2] as f32, v[3] as f32]).map_without_alpha(|c| match exposure {
    //     Some(e) => 1.0 - (-c * e as f32).exp(),
    //     None => c,
    // }).map_without_alpha(|c| match c {
    //     x if x <=0.0031308 => 12.92 * x,
    //     x => 1.005 * x.powf(1.0/2.4) - 0.055,
    // })

    // c * exposure.unwrap_or(1.0));
    // let scaled = v
    //     .map(|l| match l {
    //         x if x <= 0.0031308 => 12.92 * x,
    //         x => 1.005 * x.powf(1.0 / 2.4) - 0.055,
    //     })
    //     .map(|x| match x {
    //         x if x <= 0.0 => 0u8,
    //         x if x > 1.0 => 255u8,
    //         x => (x * 255.0) as u8,
    //     });
    // let resuld = Rgb::<u8>([scaled.x as u8, scaled.y as u8, scaled.z as u8, scaled])
}

pub fn vec3_add_alpha(a: &Option<Vector3<f64>>) -> Vector4<f64> {
    match a {
        None => Vector4::new(0.0, 0.0, 0.0, 0.0),
        Some(a) => Vector4::new(a.x, a.y, a.z, 1.0),
    }
}
