use nalgebra::SVector;
use std::iter::zip;

pub fn dda<const D: usize>(
    p1: SVector<f32, D>,
    p2: SVector<f32, D>,
    dimension: usize,
) -> Vec<SVector<f32, D>> {
    if p2[dimension] == p1[dimension] {
        return vec![];
    };
    let multiplier = match p2[dimension] < p1[dimension] {
        true => -1f32,
        false => 1f32,
    };
    let d = multiplier * (p2 - p1) / (p2[dimension] - p1[dimension]);
    let mut current = p1 + (f32::ceil(p1[dimension]) - p1[dimension]) * d;
    let mut output: Vec<SVector<f32, D>> = vec![];
    while multiplier * current[dimension] < multiplier * p2[dimension] {
        output.push(current);
        current = current + d;
    }
    output
}

#[cfg(test)]
mod dda_tests {
    use nalgebra::SVector;

    use super::dda;

    #[test]
    fn performs_dda_on_similar_vectors() {
        let p1: SVector<f32, 3> = SVector::from_vec(vec![0f32, 0f32, 0f32]);
        let p2: SVector<f32, 3> = SVector::from_vec(vec![0f32, 10f32, 10f32]);
        let r1 = dda(p1, p1, 0);
        let r2 = dda(p1, p1, 1);
        let r3 = dda(p1, p2, 0);
        let zero: Vec<SVector<f32, 3>> = vec![];
        assert_eq!(r1, zero);
        assert_eq!(r2, zero);
        assert_eq!(r3, zero);
    }

    #[test]
    fn correctly_performs_dda() {
        let p1: SVector<f32, 3> = SVector::from_vec(vec![0f32, 0f32, 0f32]);
        let p2: SVector<f32, 3> = SVector::from_vec(vec![10f32, 10f32, 10f32]);
        let result = dda(p1, p2, 0);
        assert_eq!(result.len(), 10);
        assert_eq!(
            result[0],
            SVector::<f32, 3>::from_vec(vec![0f32, 0f32, 0f32])
        );
        assert_eq!(
            result[1],
            SVector::<f32, 3>::from_vec(vec![1f32, 1f32, 1f32])
        )
    }

    #[test]
    fn correctly_performs_dda_in_reverse() {
        let p1: SVector<f32, 3> = SVector::from_vec(vec![0f32, 0f32, 0f32]);
        let p2: SVector<f32, 3> = SVector::from_vec(vec![10f32, 10f32, 10f32]);
        let result = dda(p2, p1, 0);
        assert_eq!(result.len(), 10);
        assert_eq!(
            result[0],
            SVector::<f32, 3>::from_vec(vec![10f32, 10f32, 10f32])
        );
        assert_eq!(
            result[1],
            SVector::<f32, 3>::from_vec(vec![9f32, 9f32, 9f32])
        )
    }

    #[test]
    fn normalizes_dimensions() {
        let p1: SVector<f32, 2> = SVector::from_vec(vec![0.3, 1f32]);
        let p2: SVector<f32, 2> = SVector::from_vec(vec![7.2, 5f32]);
        let results = dda(p1, p2, 0);
        assert_eq!(results[0][0], 1f32);
    }
}

pub fn perform_scanline<const D: usize>(
    p1: SVector<f32, D>,
    p2: SVector<f32, D>,
    p3: SVector<f32, D>,
) -> Vec<SVector<f32, D>> {
    let x_dimension = 1;
    let y_dimension = 2;
    let mut s = [p1, p2, p3];
    s.sort_by(|a, b| a[y_dimension].total_cmp(&b[y_dimension]));
    let [s1, s2, s3] = s;
    let is_left_corner = s2[x_dimension] < s1[x_dimension];
    let scan_lines = match is_left_corner {
        true => {
            let mut left = dda(s1, s2, y_dimension);
            left.append(&mut dda(s2, s3, y_dimension));
            let right = dda(s1, s3, y_dimension);
            zip(left, right)
        }
        false => {
            let left = dda(s1, s3, y_dimension);
            let mut right = dda(s1, s2, y_dimension);
            right.append(&mut dda(s2, s3, y_dimension));
            zip(left, right)
        }
    };
    let mut fragments: Vec<SVector<f32, D>> = vec![];
    for (left, right) in scan_lines {
        fragments.append(&mut dda(left, right, x_dimension));
    }
    fragments
}

pub fn perform_scanline_array<const D: usize>(arr: [SVector<f32, D>; 3]) -> Vec<SVector<f32, D>> {
    perform_scanline(arr[0], arr[1], arr[2])
}

#[cfg(test)]
mod scanline_tests {
    use super::perform_scanline;
    use itertools::*;
    use nalgebra::SVector;

    #[test]
    fn correctly_performs_scanline_left_corner() {
        let p1: SVector<f32, 4> = SVector::from_vec(vec![0f32, 5f32, 9f32, 2f32]);
        let p2: SVector<f32, 4> = SVector::from_vec(vec![0f32, 3f32, 5f32, 2f32]);
        let p3: SVector<f32, 4> = SVector::from_vec(vec![0f32, 8f32, 4f32, 2f32]);
        let r1 = perform_scanline(p1, p2, p3);
        let r2 = perform_scanline(p2, p1, p3);
        let r3 = perform_scanline(p3, p1, p2);
        assert_eq!(r1, r2);
        assert_eq!(r2, r3);
    }

    #[test]
    fn normalizes_in_y_dimension() {
        // expected ys: 1, 2, 3, 4, 5
        let p1: SVector<f32, 4> = SVector::from_vec(vec![0f32, 7f32, 0.3, 2f32]);
        let p2: SVector<f32, 4> = SVector::from_vec(vec![0f32, 1f32, 3.8, 2f32]);
        let p3: SVector<f32, 4> = SVector::from_vec(vec![0f32, 10f32, 5.8, 2f32]);
        let points = perform_scanline(p1, p2, p3);
        let y_values: Vec<i32> = points.iter().map(|p| p[2] as i32).unique().collect();
        itertools::assert_equal(y_values, vec![1, 2, 3, 4, 5]);
    }
}
