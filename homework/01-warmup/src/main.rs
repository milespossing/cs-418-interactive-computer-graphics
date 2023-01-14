use hex::FromHex;
use image::{Rgba, RgbaImage};
use std::{env, str::FromStr};

#[derive(Debug)]
enum Entry {
    Xyrgb { x: u32, y: u32, r: u8, g: u8, b: u8 },
    Xyc { x: u32, y: u32, color: [u8; 3] },
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").filter(|&s| !s.is_empty()).collect();
        match split[0] {
            "xyrgb" => Ok(Entry::Xyrgb {
                x: u32::from_str(split[1]).unwrap(),
                y: u32::from_str(split[2]).unwrap(),
                r: u8::from_str(split[3]).unwrap(),
                g: u8::from_str(split[4]).unwrap(),
                b: u8::from_str(split[5]).unwrap(),
            }),
            "xyc" => {
                let color =
                    <[u8; 3]>::from_hex(split[3].split("#").collect::<Vec<&str>>()[1]).unwrap();
                Ok(Entry::Xyc {
                    x: u32::from_str(split[1]).unwrap(),
                    y: u32::from_str(split[2]).unwrap(),
                    color,
                })
            }
            _ => Err(format!("Not a recognized command: {}", split[0])),
        }
    }
}

#[cfg(test)]
mod entry_tests {
    use crate::*;
    use assert_matches::assert_matches;

    #[test]
    fn can_deserialize_xyrgb() {
        let result = Entry::from_str("xyrgb 0 1 255 255 255");
        assert_matches!(result, Ok(Entry::Xyrgb { x, y, r, g, b }) if x == 0 && y == 1 && r == 255 && g == 255 && b == 255);
    }

    #[test]
    fn can_deserialize_xyc() {
        let result = Entry::from_str("xyc 2 3 #aaaaff");
        assert_matches!(result, Ok(Entry::Xyc { x, y, color }) if x == 2 && y == 3 => {
            assert_eq!([170,170,255], color);
        });
    }
}

#[derive(Debug)]
struct File {
    width: u32,
    height: u32,
    filename: String,
    entries: Vec<Entry>,
}

impl FromStr for File {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let strings: Vec<&str> = s.split("\n").filter(|&s| !s.is_empty()).collect();
        let header_split: Vec<&str> = strings[0].split(" ").collect();
        let width: u32 = u32::from_str(header_split[1]).unwrap();
        let height: u32 = u32::from_str(header_split[2]).unwrap();
        let filename: String = String::from(header_split[3]);
        let entries: Vec<Entry> = strings
            .iter()
            .skip(1)
            .map(|s| Entry::from_str(s))
            .filter(|r| r.is_ok())
            .map(|s| s.unwrap())
            .collect();
        Ok(File {
            width,
            height,
            filename,
            entries,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: String = args[1].to_owned();
    let contents =
        std::fs::read_to_string(std::path::Path::new(&file_name)).expect("Failed to read file");
    let file: File = File::from_str(&contents).unwrap();
    let mut image = RgbaImage::from_pixel(file.width, file.height, Rgba([0, 0, 0, 0]));
    for entry in file.entries.iter() {
        match entry {
            &Entry::Xyrgb { x, y, r, g, b } => image.put_pixel(x, y, Rgba([r, g, b, 255])),
            &Entry::Xyc { x, y, color } => {
                image.put_pixel(x, y, Rgba([color[0], color[1], color[2], 255]))
            }
        }
    }
    image.save(file.filename).expect("Failed to write image");
}
