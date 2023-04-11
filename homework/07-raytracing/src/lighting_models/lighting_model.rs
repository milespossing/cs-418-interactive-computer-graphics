use nalgebra::Vector3;

trait LightingModel {
    fn get_distribution(&self, dir: Vector3<f64>) -> f64;
}