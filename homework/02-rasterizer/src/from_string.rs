use crate::models::{Entry, File, FileHeader, Triangle, Vertex};
use std::str::FromStr;

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = s.split(' ').filter(|&s| !s.is_empty()).collect();
        match segments[0] {
            "xyzw" => {
                let x: f32 = segments[1].parse().unwrap();
                let y: f32 = segments[2].parse().unwrap();
                let z: f32 = segments[3].parse().unwrap();
                let w: f32 = segments[4].parse().unwrap();
                Ok(Self::Xyzw([x, y, z, w]))
            }
            "rgb" => {
                let r: f32 = segments[1].parse().unwrap();
                let g: f32 = segments[2].parse().unwrap();
                let b: f32 = segments[3].parse().unwrap();
                Ok(Self::Rgb([r, g, b]))
            }
            "rgba" => {
                let r: f32 = segments[1].parse().unwrap();
                let g: f32 = segments[2].parse().unwrap();
                let b: f32 = segments[3].parse().unwrap();
                let a: f32 = segments[4].parse().unwrap();
                Ok(Self::Rgba([r, g, b, a]))
            }
            "tri" => {
                let i1: i8 = segments[1].parse().unwrap();
                let i2: i8 = segments[2].parse().unwrap();
                let i3: i8 = segments[3].parse().unwrap();
                Ok(Self::Triangle([i1, i2, i3]))
            }
            "depth" => Ok(Entry::Depth),
            "sRGB" => Ok(Entry::Srgb),
            "#" => Ok(Entry::Comment),
            _ => Err(format!("Could not parse line: {}", s)),
        }
    }
}

impl FromStr for FileHeader {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<&str> = s.split(" ").filter(|&s| !s.is_empty()).collect();
        let width: u32 = segments[1].parse().unwrap();
        let height: u32 = segments[2].parse().unwrap();
        let file_name: String = String::from_str(segments[3]).unwrap();
        Ok(Self {
            size: (width, height),
            name: file_name,
        })
    }
}

impl FromStr for File {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn get_vertex(i: i8, v: &Vec<Vertex>) -> Vertex {
            let neg: bool = i < 0;
            let ind: usize = match neg {
                true => v.len() - usize::try_from(i * -1).unwrap(),
                false => usize::try_from(i - 1).unwrap(),
            };
            v[ind]
        }
        let lines: Vec<&str> = s.split("\n").filter(|&l| !l.is_empty()).collect();
        let header = FileHeader::from_str(lines[0]).unwrap();
        let entries: Vec<Entry> = lines
            .iter()
            .skip(1)
            .map(|l| Entry::from_str(l))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        let mut vertices: Vec<Vertex> = vec![];
        let mut triangles: Vec<Triangle> = vec![];
        let mut current_color: [f32; 4] = [255f32, 255f32, 255f32, 255f32];
        let mut depth: bool = false;
        let mut srgb: bool = false;
        for entry in entries {
            match entry {
                Entry::Xyzw(xyzw) => vertices.push(Vertex::from_xyzw_rgba(xyzw, current_color)),
                Entry::Rgb(rgb) => current_color = [rgb[0], rgb[1], rgb[2], 255f32],
                Entry::Rgba(rgba) => current_color = [rgba[0], rgba[1], rgba[2], rgba[3]],
                Entry::Triangle(indices) => {
                    let i1 = get_vertex(indices[0], &vertices);
                    let i2 = get_vertex(indices[1], &vertices);
                    let i3 = get_vertex(indices[2], &vertices);
                    triangles.push([i1, i2, i3])
                }
                Entry::Depth => {
                    depth = true;
                }
                Entry::Srgb => {
                    srgb = true;
                }
                Entry::Comment => { /* Do nothing */ }
            }
        }
        Ok(File {
            header,
            triangles,
            depth,
            srgb,
        })
    }
}

#[cfg(test)]
mod parsing_tests {
    use super::*;
    use crate::models::*;
    use assert_matches::assert_matches;

    #[test]
    fn xyzw() {
        let t1 = Entry::from_str("xyzw   1  3.5  3 4");
        let t2 = Entry::from_str("xyzw  -1 -2   -3 4");
        assert_matches!(t1, Ok(Entry::Xyzw(x)) if (x[0] == 1f32 && x[1] == 3.5));
        assert_matches!(t2, Ok(Entry::Xyzw(x)) if (x[0] == -1f32 && x[1] == -2f32));
    }

    #[test]
    fn rgb() {
        let t1 = Entry::from_str("rgb 0 0 0");
        let t2 = Entry::from_str("rgb 0 255 0");
        assert_matches!(t1, Ok(Entry::Rgb(x)) if (x[0] == 0f32 && x[1] == 0f32));
        assert_matches!(t2, Ok(Entry::Rgb(x)) if (x[0] == 0f32 && x[1] == 255f32));
    }

    #[test]
    fn file_parse() {
        let file = "png 20 30 mp1indexing.png
xyzw   1  3.5  3 4
xyzw  -1 -2   -3 4
rgb 0 0 0
xyzw   2  0    0 2
tri 1 -1 2
xyzw  -1  0.5  0 1
tri 1 -2 -1";
        let r = File::from_str(file);
        assert_matches!(&r, Ok(f) if f.header.name == "mp1indexing.png" && f.triangles.len() == 2);
        let triangles = r.unwrap().triangles;
        let expected: Triangle = [
            Vertex::from_xyzw_rgba([1f32, 3.5, 3f32, 4f32], [255f32, 255f32, 255f32, 255f32]),
            Vertex::from_xyzw_rgba([2f32, 0f32, 0f32, 2f32], [0f32, 0f32, 0f32, 255f32]),
            Vertex::from_xyzw_rgba(
                [-1f32, -2f32, -3f32, 4f32],
                [255f32, 255f32, 255f32, 255f32],
            ),
        ];
        assert_eq!(triangles[0], expected)
    }
}
