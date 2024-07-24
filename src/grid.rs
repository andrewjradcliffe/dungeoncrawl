use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    // Public within this crate since I may (ab)use these in multiple places.
    pub(crate) inner: Vec<T>,
    pub(crate) n_rows: usize,
    pub(crate) n_cols: usize,
}

impl<T> Grid<T> {
    #[inline]
    pub(crate) const fn cartesian_to_linear(n_cols: usize, i: usize, j: usize) -> usize {
        i * n_cols + j
    }
    #[inline]
    pub(crate) const fn linear_to_cartesian(n_cols: usize, l: usize) -> (usize, usize) {
        let q = l / n_cols;
        let m = l - q * n_cols;
        (q, m)
    }
    #[inline]
    pub fn linear_index(&self, i: usize, j: usize) -> usize {
        Self::cartesian_to_linear(self.n_cols, i, j)
    }
    #[inline]
    pub fn cartesian_index(&self, l: usize) -> (usize, usize) {
        Self::linear_to_cartesian(self.n_cols, l)
    }
    #[inline]
    pub fn n_rows(&self) -> usize {
        self.n_rows
    }
    #[inline]
    pub fn n_cols(&self) -> usize {
        self.n_cols
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
    #[inline]
    pub fn shape(&self) -> (usize, usize) {
        (self.n_rows, self.n_cols)
    }
    pub(crate) fn check_bounds(&self, (i, j): (usize, usize)) {
        if i >= self.n_rows || j >= self.n_cols {
            panic!(
                "index out of bounds: the dimensions are ({}, {}) but the index is ({}, {})",
                self.n_rows, self.n_cols, i, j,
            );
        }
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n_rows = self.n_rows();
        let n_cols = self.n_cols();
        for i in 0..n_rows {
            for j in 0..n_cols {
                write!(f, "{}", self[(i, j)])?;
            }
            if i != n_rows - 1 {
                write!(f, "{}", '\n')?;
            }
        }
        Ok(())
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    #[inline]
    fn index(&self, cartesian: (usize, usize)) -> &Self::Output {
        self.check_bounds(cartesian);
        let idx = self.linear_index(cartesian.0, cartesian.1);
        &self.inner[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, cartesian: (usize, usize)) -> &mut Self::Output {
        self.check_bounds(cartesian);
        let idx = self.linear_index(cartesian.0, cartesian.1);
        &mut self.inner[idx]
    }
}
impl<T: Default> Grid<T> {
    pub fn new_default(n_rows: usize, n_cols: usize) -> Self {
        let n = n_rows * n_cols;
        let mut inner = Vec::with_capacity(n);
        inner.resize_with(n, T::default);
        Self {
            inner,
            n_rows,
            n_cols,
        }
    }
}
impl<T: Clone> Grid<T> {
    pub fn transpose(&self) -> Self {
        let n_rows = self.n_rows();
        let n_cols = self.n_cols();
        let mut other = Vec::with_capacity(self.len());
        for j in 0..n_cols {
            for i in 0..n_rows {
                other.push(self[(i, j)].clone());
            }
        }
        Self {
            inner: other,
            n_rows: n_cols,
            n_cols: n_rows,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cart2d_to_linear() {
        macro_rules! test {
            ($n_rows:expr, $n_cols:expr) => {{
                let n_cols: usize = $n_cols;
                let n_rows: usize = $n_rows;
                let mut l: usize = 0;
                for i in 0..n_rows {
                    for j in 0..n_cols {
                        assert_eq!(Grid::<bool>::cartesian_to_linear(n_cols, i, j), l);
                        l += 1;
                    }
                }
            }};
        }
        test!(5, 7);
        test!(7, 5);
        test!(4, 4);
        test!(6, 7);
        test!(1, 1);
        test!(1, 4);
        test!(4, 1);
    }

    #[test]
    fn linear_to_cart2d() {
        assert_eq!(Grid::<bool>::linear_to_cartesian(5, 7), (1, 2));
        assert_eq!(Grid::<bool>::linear_to_cartesian(7, 5), (0, 5));

        assert_eq!(Grid::<bool>::linear_to_cartesian(6, 29), (4, 5));
        assert_eq!(Grid::<bool>::linear_to_cartesian(6, 34), (5, 4));
    }

    #[test]
    fn default() {
        let grid = Grid::<bool>::new_default(5, 7);
        assert!(grid.inner.iter().all(|x| !*x));
        assert_eq!(grid.shape(), (5, 7));
        assert_eq!(grid.len(), 35);
    }
}
