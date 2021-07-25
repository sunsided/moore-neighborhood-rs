pub mod dynamic {
    pub fn moore(range: u32, dimensions: u32) -> Vec<Vec<isize>> {
        let size: usize = range as usize * 2 + 1;
        let length: usize = size.pow(dimensions) - 1;
        let half_length = length / 2;
        let mut neighbors = Vec::with_capacity(length as _);

        for i in 0usize..length {
            let mut neighbor = Vec::with_capacity(dimensions as _);
            let mut index = if i < half_length { i } else { i + 1 };
            let mut prev_divisor = 1;
            for _dimension in 0..dimensions {
                let divisor = prev_divisor * size;
                let value = index % divisor;
                neighbor.push((value / prev_divisor) as isize - range as isize);
                prev_divisor = divisor;
                index -= value;
            }

            neighbors.push(neighbor);
        }
        neighbors
    }
}

pub mod generic_dimension {
    pub fn moore<const DIMENSIONS: usize>(range: u32) -> Vec<[isize; DIMENSIONS]> {
        assert!(DIMENSIONS < u32::MAX as _);

        let size: usize = range as usize * 2 + 1;
        let length: usize = size.pow(DIMENSIONS as _) - 1;
        let half_length = length / 2;
        let mut neighbors = Vec::with_capacity(length as _);

        for i in 0usize..length {
            let mut neighbor = [0; DIMENSIONS];
            let mut index = if i < half_length { i } else { i + 1 };
            let mut prev_divisor = 1;
            for dimension in 0..DIMENSIONS {
                let divisor = prev_divisor * size;
                let value = index % divisor;
                neighbor[dimension] = (value / prev_divisor) as isize - range as isize;
                prev_divisor = divisor;
                index -= value;
            }

            neighbors.push(neighbor);
        }
        neighbors
    }
}

pub mod generic_full {
    #[inline]
    pub fn moore<const RANGE: u32, const DIMENSIONS: usize, const LENGTH: usize>(
    ) -> [[isize; DIMENSIONS]; LENGTH] {
        assert!(DIMENSIONS < u32::MAX as _);

        {
            let size: usize = RANGE as usize * 2 + 1;
            debug_assert_eq!(LENGTH, size.pow(DIMENSIONS as _) - 1);
        }

        let mut neighbors = [[0isize; DIMENSIONS]; LENGTH];
        moore_prealloc::<RANGE, DIMENSIONS, LENGTH>(&mut neighbors);
        neighbors
    }

    pub fn moore_prealloc<const RANGE: u32, const DIMENSIONS: usize, const LENGTH: usize>(
        neighbors: &mut [[isize; DIMENSIONS]; LENGTH],
    ) -> usize {
        assert!(DIMENSIONS < u32::MAX as _);

        let size: usize = RANGE as usize * 2 + 1;
        let length = size.pow(DIMENSIONS as _) - 1;
        assert!(LENGTH >= length);

        let half_length = LENGTH / 2;
        for i in 0usize..LENGTH {
            let neighbor = &mut neighbors[i];

            let mut index = if i < half_length { i } else { i + 1 };
            let mut prev_divisor = 1;
            for dimension in 0..DIMENSIONS {
                let divisor = prev_divisor * size;
                let value = index % divisor;
                neighbor[dimension] = (value / prev_divisor) as isize - RANGE as isize;
                prev_divisor = divisor;
                index -= value;
            }
        }
        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dyn_d2_r1_works() {
        let mut result = dynamic::moore(1, 2);

        #[rustfmt::skip]
        let mut expected = [
            [-1,-1], [ 0,-1], [ 1,-1],
            [-1, 0],          [ 1, 0],
            [-1, 1], [ 0, 1], [ 1, 1]
        ];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn gen_dim_d2_r1_works() {
        let mut result: Vec<[isize; 2]> = generic_dimension::moore(1);

        #[rustfmt::skip]
        let mut expected = [
            [-1,-1], [ 0,-1], [ 1,-1],
            [-1, 0],          [ 1, 0],
            [-1, 1], [ 0, 1], [ 1, 1]
        ];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn gen_x_d2_r1_works() {
        let mut result: [[isize; 2]; 8] = generic_full::moore::<1, 2, 8>();

        #[rustfmt::skip]
        let mut expected = [
            [-1,-1], [ 0,-1], [ 1,-1],
            [-1, 0],          [ 1, 0],
            [-1, 1], [ 0, 1], [ 1, 1]
        ];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn dyn_d2_r2_works() {
        let mut result = dynamic::moore(2, 2);

        #[rustfmt::skip]
        let mut expected = [
            [-2,-2], [-1,-2], [ 0,-2], [ 1,-2], [ 2,-2],
            [-2,-1], [-1,-1], [ 0,-1], [ 1,-1], [ 2,-1],
            [-2, 0], [-1, 0],          [ 1, 0], [ 2, 0],
            [-2, 1], [-1, 1], [ 0, 1], [ 1, 1], [ 2, 1],
            [-2, 2], [-1, 2], [ 0, 2], [ 1, 2], [ 2, 2]
        ];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn gen_dim_d2_r2_works() {
        let mut result: Vec<[isize; 2]> = generic_dimension::moore(2);

        #[rustfmt::skip]
        let mut expected = [
            [-2,-2], [-1,-2], [ 0,-2], [ 1,-2], [ 2,-2],
            [-2,-1], [-1,-1], [ 0,-1], [ 1,-1], [ 2,-1],
            [-2, 0], [-1, 0],          [ 1, 0], [ 2, 0],
            [-2, 1], [-1, 1], [ 0, 1], [ 1, 1], [ 2, 1],
            [-2, 2], [-1, 2], [ 0, 2], [ 1, 2], [ 2, 2]
        ];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn dyn_d3_r1_works() {
        let mut result = dynamic::moore(1, 3);

        #[rustfmt::skip]
        let mut expected = [
            [-1,-1,-1], [ 0,-1,-1], [ 1,-1,-1],
            [-1, 0,-1], [ 0, 0,-1], [ 1, 0,-1],
            [-1, 1,-1], [ 0, 1,-1], [ 1, 1,-1],

            [-1,-1, 0], [ 0,-1, 0], [ 1,-1, 0],
            [-1, 0, 0],             [ 1, 0, 0],
            [-1, 1, 0], [ 0, 1, 0], [ 1, 1, 0],

            [-1,-1, 1], [ 0,-1, 1], [ 1,-1, 1],
            [-1, 0, 1], [ 0, 0, 1], [ 1, 0, 1],
            [-1, 1, 1], [ 0, 1, 1], [ 1, 1, 1]
        ];

        result.sort();
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn dyn_same_as_reference() {
        let result = dynamic::moore(3, 3);
        let expected = reference(3, 3);
        assert_eq!(result, expected);
    }

    pub fn reference(range: u32, dimensions: u32) -> Vec<Vec<isize>> {
        let size: usize = range as usize * 2 + 1;
        let length: usize = size.pow(dimensions) - 1;
        let mut neighbors = Vec::with_capacity(length as _);

        for i in 0usize..length {
            let mut neighbor = Vec::with_capacity(dimensions as _);
            let mut index = if i < length / 2 { i } else { i + 1 };
            for dimension in 1..=dimensions {
                let value = index % size.pow(dimension as _);
                neighbor.push((value / size.pow(dimension - 1)) as isize - range as isize);
                index -= value;
            }

            neighbors.push(neighbor);
        }

        neighbors
    }
}
