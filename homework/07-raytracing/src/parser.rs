use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub enum FileType {
    Png,
}

// impl FromStr for FileType {}

impl FromStr for FileType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "png" => Ok(FileType::Png),
            _ => Err(format!("Unknown file type: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct FileHeader {
    pub output_type: FileType,
    pub width: u32,
    pub height: u32,
    pub name: String,
}

impl FromStr for FileHeader {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let output_type = match FileType::from_str(parts[0]) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };
        let width = match parts[1].parse::<u32>() {
            Ok(w) => w,
            Err(e) => return Err(e.to_string()),
        };
        let height = match parts[2].parse::<u32>() {
            Ok(h) => h,
            Err(e) => return Err(e.to_string()),
        };
        let name = parts[3].to_string();
        Ok(FileHeader {
            output_type,
            width,
            height,
            name,
        })
    }
}

#[derive(Debug)]
pub enum FileEntry {
    Sphere { x: f64, y: f64, z: f64, r: f64 },
    Sun { x: f64, y: f64, z: f64 },
    Color { r: f64, g: f64, b: f64 },
    Plane { a: f64, b: f64, c: f64, d: f64 },
    Xyz { x: f64, y: f64, z: f64 },
    Triangle { a: i32, b: i32, c: i32 },
    Bulb { x: f64, y: f64, z: f64 },
    Eye { x: f64, y: f64, z: f64 },
    Forward { x: f64, y: f64, z: f64 },
    Up { x: f64, y: f64, z: f64 },
    Expose { v: f64 },
    Shiny { s: f64 },
    Bounces { b: usize },
    Aa { n: usize },
}

impl FromStr for FileEntry {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "sphere" => {
                let x = match parts[1].parse::<f64>() {
                    Ok(x) => x,
                    Err(e) => return Err(e.to_string()),
                };
                let y = match parts[2].parse::<f64>() {
                    Ok(y) => y,
                    Err(e) => return Err(e.to_string()),
                };
                let z = match parts[3].parse::<f64>() {
                    Ok(z) => z,
                    Err(e) => return Err(e.to_string()),
                };
                let r = match parts[4].parse::<f64>() {
                    Ok(r) => r,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Sphere { x, y, z, r })
            }
            "sun" => {
                let x = match parts[1].parse::<f64>() {
                    Ok(x) => x,
                    Err(e) => return Err(e.to_string()),
                };
                let y = match parts[2].parse::<f64>() {
                    Ok(y) => y,
                    Err(e) => return Err(e.to_string()),
                };
                let z = match parts[3].parse::<f64>() {
                    Ok(z) => z,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Sun { x, y, z })
            }
            "color" => {
                let r = match parts[1].parse::<f64>() {
                    Ok(r) => r,
                    Err(e) => return Err(e.to_string()),
                };
                let g = match parts[2].parse::<f64>() {
                    Ok(g) => g,
                    Err(e) => return Err(e.to_string()),
                };
                let b = match parts[3].parse::<f64>() {
                    Ok(b) => b,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Color { r, g, b })
            }
            "plane" => {
                let a = match parts[1].parse::<f64>() {
                    Ok(a) => a,
                    Err(e) => return Err(e.to_string()),
                };
                let b = match parts[2].parse::<f64>() {
                    Ok(b) => b,
                    Err(e) => return Err(e.to_string()),
                };
                let c = match parts[3].parse::<f64>() {
                    Ok(c) => c,
                    Err(e) => return Err(e.to_string()),
                };
                let d = match parts[4].parse::<f64>() {
                    Ok(d) => d,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Plane { a, b, c, d })
            }
            "xyz" => {
                let x = match parts[1].parse::<f64>() {
                    Ok(x) => x,
                    Err(e) => return Err(e.to_string()),
                };
                let y = match parts[2].parse::<f64>() {
                    Ok(y) => y,
                    Err(e) => return Err(e.to_string()),
                };
                let z = match parts[3].parse::<f64>() {
                    Ok(z) => z,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Xyz { x, y, z })
            }
            "trif" => {
                let a = match parts[1].parse::<i32>() {
                    Ok(a) => a,
                    Err(e) => return Err(e.to_string()),
                };
                let b = match parts[2].parse::<i32>() {
                    Ok(b) => b,
                    Err(e) => return Err(e.to_string()),
                };
                let c = match parts[3].parse::<i32>() {
                    Ok(c) => c,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Triangle { a, b, c })
            }
            "bulb" => {
                let x = match parts[1].parse::<f64>() {
                    Ok(x) => x,
                    Err(e) => return Err(e.to_string()),
                };
                let y = match parts[2].parse::<f64>() {
                    Ok(y) => y,
                    Err(e) => return Err(e.to_string()),
                };
                let z = match parts[3].parse::<f64>() {
                    Ok(z) => z,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Bulb { x, y, z })
            }
            "eye" => {
                let x = match parts[1].parse::<f64>() {
                    Ok(x) => x,
                    Err(e) => return Err(e.to_string()),
                };
                let y = match parts[2].parse::<f64>() {
                    Ok(y) => y,
                    Err(e) => return Err(e.to_string()),
                };
                let z = match parts[3].parse::<f64>() {
                    Ok(z) => z,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Eye { x, y, z })
            }
            "forward" => {
                let x = match parts[1].parse::<f64>() {
                    Ok(x) => x,
                    Err(e) => return Err(e.to_string()),
                };
                let y = match parts[2].parse::<f64>() {
                    Ok(y) => y,
                    Err(e) => return Err(e.to_string()),
                };
                let z = match parts[3].parse::<f64>() {
                    Ok(z) => z,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Forward { x, y, z })
            }
            "up" => {
                let x = match parts[1].parse::<f64>() {
                    Ok(x) => x,
                    Err(e) => return Err(e.to_string()),
                };
                let y = match parts[2].parse::<f64>() {
                    Ok(y) => y,
                    Err(e) => return Err(e.to_string()),
                };
                let z = match parts[3].parse::<f64>() {
                    Ok(z) => z,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Up { x, y, z })
            }
            "expose" => {
                let v = match parts[1].parse::<f64>() {
                    Ok(v) => v,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Expose { v })
            }
            "shininess" => {
                let s = match parts[1].parse::<f64>() {
                    Ok(s) => s,
                    Err(e) => return Err(e.to_string()),
                };
                Ok(FileEntry::Shiny { s })
            }
            "bounces" => match parts[1].parse::<usize>() {
                Ok(b) => Ok(FileEntry::Bounces { b }),
                Err(e) => Err(e.to_string()),
            },
            "aa" => match parts[1].parse::<usize>() {
                Ok(n) => Ok(FileEntry::Aa { n }),
                Err(e) => Err(e.to_string()),
            },
            _ => Err(format!("Unknown file entry: {}", s)),
        }
    }
}

#[derive(Debug)]
// Defines the ray tracing procedure
pub struct ProcFile {
    pub header: FileHeader,
    pub entries: Vec<FileEntry>,
}

impl ProcFile {
    pub fn get_aa(&self) -> usize {
        match self.entries.iter().find_map(|e| match e {
            FileEntry::Aa { n } => Some(*n),
            _ => None,
        }) {
            Some(n) => n,
            None => 1,
        }
    }

    pub fn get_exposure(&self) -> Option<f64> {
        self.entries.iter().find_map(|e| match e {
            FileEntry::Expose { v } => Some(*v),
            _ => None,
        })
    }
}

pub fn parse_file(path: PathBuf) -> Result<ProcFile, String> {
    let contents = std::fs::read_to_string(path).expect("Failed to read file");
    let lines: Vec<&str> = contents.split("\n").filter(|&l| !l.is_empty()).collect();
    let header = FileHeader::from_str(lines[0])?;
    let mut entries: Vec<FileEntry> = vec![];
    for line in lines.iter().skip(1) {
        entries.push(FileEntry::from_str(*line)?);
    }
    Ok(ProcFile { header, entries })
}
