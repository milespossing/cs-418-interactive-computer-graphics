mod lighting_models;
mod models;
mod parser;
mod rasterize;
mod raytracer;
mod renderer;
mod scene;
mod utils;

use crate::parser::{parse_file, ProcFile};
use crate::rasterize::Rasterizer;
use crate::renderer::Renderer;
use std::env;
use std::path::{Path, PathBuf};

fn get_argument(args: &Vec<String>, i: usize) -> Option<String> {
    return if args.len() <= i {
        None
    } else {
        Some(args[i].to_owned())
    };
}

fn get_as_path(args: &Vec<String>, i: usize) -> Option<PathBuf> {
    match get_argument(args, i) {
        Some(path) => Some(Path::new(&path).to_path_buf()),
        None => None,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = get_as_path(&args, 1).unwrap(); //args[1].to_owned();
    let output_path = get_as_path(&args, 2).unwrap_or(Path::new("./").to_path_buf()); //String::from("."));
    let file: ProcFile = parse_file(file_name).unwrap();
    let scene = scene::Scene::from_file(&file).unwrap();
    let renderer = Renderer::from_file(&file, &scene).unwrap();
    let rasterizer = Rasterizer::new(&file);
    let output = renderer.render_scene().unwrap();
    let image = rasterizer.rasterize(&output).unwrap();
    image
        .save(output_path.join(Path::new(&file.header.name)))
        .unwrap();
    println!("Done.");
}
