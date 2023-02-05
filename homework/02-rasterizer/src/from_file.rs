use std::str::FromStr;

pub fn from_file<T: FromStr>(path: &std::path::Path) -> Result<T, T::Err> {
    let lines = std::fs::read_to_string(path).unwrap();
    T::from_str(&lines)
}
