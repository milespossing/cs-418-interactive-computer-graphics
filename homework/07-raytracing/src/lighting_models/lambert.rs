use crate::utils::clamp;
use nalgebra::Vector3;

pub struct LambertLighting {}

impl LambertLighting {
    pub fn get_distribution(&self, dir: &Vector3<f64>, surface_normal: &Vector3<f64>) -> f64 {
        clamp(dir.normalize().dot(&surface_normal.normalize()))
    }
}
