use crate::grid::*;
use std::mem;

#[derive(Debug, Clone, PartialEq)]
pub struct Algorithm {
    pub(crate) grid: Grid<isize>,
}

impl Algorithm {
    pub fn new(n_rows: usize, n_cols: usize) -> Self {
        let grid = Grid::<isize>::new_fill(n_rows, n_cols, -1);
        Self { grid }
    }

    pub fn reset(&mut self) {
        self.grid.fill(-1);
    }
    pub(crate) fn is_barrier(&self, i: usize, j: usize) -> bool {
        self.grid[(i, j)] == -2
    }
    #[allow(dead_code)]
    pub(crate) fn multi_path(
        &mut self,
        i: usize,
        j: usize,
        p: &[(usize, usize)],
        k: isize,
        dst: (usize, usize),
        rs: &mut Vec<Vec<(usize, usize)>>,
        qs: &mut Vec<Vec<(usize, usize)>>,
    ) {
        let a_ij = self.grid[(i, j)];
        if (i, j) == dst {
            let mut p = p.to_vec();
            p.push((i, j));
            rs.push(p);
            if a_ij == -1 || a_ij > k {
                self.grid[(i, j)] = k;
            }
            // println!("{}\n", self.grid);
        } else if a_ij == -1 || a_ij > k {
            self.grid[(i, j)] = k;
            let mut p = p.to_vec();
            p.push((i, j));
            qs.push(p);
            // println!("{}\n", self.grid);
        }
    }
    pub(crate) fn single_path(
        &mut self,
        i: usize,
        j: usize,
        p: &[(usize, usize)],
        k: isize,
        dst: (usize, usize),
        rs: &mut Vec<Vec<(usize, usize)>>,
        qs: &mut Vec<Vec<(usize, usize)>>,
    ) {
        let a_ij = self.grid[(i, j)];
        if a_ij == -1 || a_ij > k {
            // println!("{}\n", self.grid);
            self.grid[(i, j)] = k;
            let mut p = p.to_vec();
            p.push((i, j));
            if (i, j) == dst {
                rs.push(p);
            } else {
                qs.push(p);
            }
        }
    }
    pub fn paths(&mut self, src: (usize, usize), dst: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
        if !self.grid.check_bounds(src) || !self.grid.check_bounds(dst) {
            panic!(
                "out of bounds! got (src, dst, bounds): ({:?}, {:?}, {:?})",
                src,
                dst,
                self.grid.shape()
            );
        } else if src == dst {
            return vec![vec![src]];
        }
        let (m, n) = self.grid.shape();
        // self.reset();
        let mut ps = vec![vec![src]];
        let mut qs = Vec::new();
        let mut rs = Vec::new();
        self.grid[(src.0, src.1)] = 0;

        // macro_rules! common {
        //     ($i:ident, $j:ident, $a_ij:ident, $p:ident, $k:ident) => {
        //         if ($i, $j) == dst {
        //             let mut p = $p.clone();
        //             p.push(($i, $j));
        //             rs.push(p);
        //             if $a_ij == -1 || $a_ij > $k {
        //                 self.grid[($i, $j)] = $k;
        //             }
        //         } else if $a_ij == -1 || $a_ij > $k {
        //             self.grid[($i, $j)] = $k;
        //             let mut p = $p.clone();
        //             p.push(($i, $j));
        //             qs.push(p);
        //         }
        //     }; // Single path
        //        // ($i:ident, $j:ident, $a_ij:ident, $p:ident, $k:ident) => {
        //        //     if $a_ij == -1 || $a_ij > k {
        //        //         self.grid[($i, $j)] = $k;
        //        //         let mut p = $p.clone();
        //        //         p.push(($i, $j));
        //        //         if ($i, $j) == dst {
        //        //             rs.push(p);
        //        //         } else {
        //        //             qs.push(p);
        //        //         }
        //        //     }
        //        // }
        // }

        loop {
            while let Some(p) = ps.pop() {
                let k = p.len();
                let (i, j) = p[k - 1];
                let k = k as isize;
                // Forward
                {
                    let j = j + 1;
                    if j < n {
                        // let a_ij = self.grid[(i, j)];
                        // if (i, j) == dst {
                        //     let mut p = p.clone();
                        //     p.push((i, j));
                        //     rs.push(p);
                        //     if a_ij == -1 || a_ij > k {
                        //         self.grid[(i, j)] = k;
                        //     }
                        // } else if a_ij == -1 || a_ij > k {
                        //     self.grid[(i, j)] = k;
                        //     let mut p = p.clone();
                        //     p.push((i, j));
                        //     qs.push(p);
                        // }
                        // common!(i, j, a_ij, p, k);
                        // if a_ij == -1 || a_ij > k {
                        //     self.grid[(i, j)] = k;
                        //     let mut p = p.clone();
                        //     p.push((i, j));
                        //     if (i, j) == dst {
                        //         rs.push(p);
                        //     } else {
                        //         qs.push(p);
                        //     }
                        // }
                        if !self.is_barrier(i, j) {
                            self.single_path(i, j, &p, k, dst, &mut rs, &mut qs);
                            // self.multi_path(i, j, &p, k, dst, &mut rs, &mut qs);
                        }
                    }
                }
                // Backward
                {
                    if j > 0 {
                        let j = j - 1;
                        if !self.is_barrier(i, j) {
                            self.single_path(i, j, &p, k, dst, &mut rs, &mut qs);
                            // self.multi_path(i, j, &p, k, dst, &mut rs, &mut qs);
                        }
                    }
                }
                // Up
                {
                    if i > 0 {
                        let i = i - 1;
                        if !self.is_barrier(i, j) {
                            self.single_path(i, j, &p, k, dst, &mut rs, &mut qs);
                            // self.multi_path(i, j, &p, k, dst, &mut rs, &mut qs);
                        }
                    }
                }
                // Down
                {
                    let i = i + 1;
                    if i < m {
                        if !self.is_barrier(i, j) {
                            self.single_path(i, j, &p, k, dst, &mut rs, &mut qs);
                            // self.multi_path(i, j, &p, k, dst, &mut rs, &mut qs);
                        }
                    }
                }
            }
            if qs.is_empty() {
                break;
            } else {
                mem::swap(&mut ps, &mut qs);
            }
        }
        rs
    }

    pub fn shorted_path(
        &mut self,
        src: (usize, usize),
        dst: (usize, usize),
    ) -> Option<Vec<(usize, usize)>> {
        if !self.grid.check_bounds(src) || !self.grid.check_bounds(dst) {
            panic!(
                "out of bounds! got (src, dst, bounds): ({:?}, {:?}, {:?})",
                src,
                dst,
                self.grid.shape()
            );
        } else if src == dst {
            return Some(vec![src]);
        }
        let (m, n) = self.grid.shape();
        // self.reset();
        let mut ps = vec![vec![src]];
        let mut qs = Vec::new();
        let mut r: Option<Vec<(usize, usize)>> = None;
        self.grid[(src.0, src.1)] = 0;

        macro_rules! common {
            //Single path
            ($i:ident, $j:ident, $p:ident, $k:ident) => {
                let a_ij = self.grid[($i, $j)];
                if a_ij == -1 || a_ij > $k {
                    self.grid[($i, $j)] = $k;
                    let mut p = $p.clone();
                    p.push(($i, $j));
                    if ($i, $j) == dst {
                        r = Some(p);
                    } else {
                        qs.push(p);
                    }
                }
            };
        }

        loop {
            while let Some(p) = ps.pop() {
                let k = p.len();
                let (i, j) = p[k - 1];
                let k = k as isize;
                // Forward
                {
                    let j = j + 1;
                    if j < n {
                        if !self.is_barrier(i, j) {
                            common!(i, j, p, k);
                        }
                    }
                }
                // Backward
                {
                    if j > 0 {
                        let j = j - 1;
                        if !self.is_barrier(i, j) {
                            common!(i, j, p, k);
                        }
                    }
                }
                // Up
                {
                    if i > 0 {
                        let i = i - 1;
                        if !self.is_barrier(i, j) {
                            common!(i, j, p, k);
                        }
                    }
                }
                // Down
                {
                    let i = i + 1;
                    if i < m {
                        if !self.is_barrier(i, j) {
                            common!(i, j, p, k);
                        }
                    }
                }
            }
            if qs.is_empty() {
                break;
            } else {
                mem::swap(&mut ps, &mut qs);
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_result(alg: &Algorithm, path: &Vec<(usize, usize)>, barrier: &Vec<(usize, usize)>) {
        println!("{}", alg.grid);
        println!("{:#?}", path);
        let mut grid = alg.grid.clone();
        grid.fill(0);
        for p in path.iter().cloned() {
            grid[p] = 1;
        }
        for p in barrier.iter().cloned() {
            grid[p] = 2;
        }
        println!("{}\n", grid);
    }

    #[test]
    fn five() {
        let origin = (0, 0);
        let dst = (4, 4);
        let distances = [16, 8];
        let barriers = vec![
            vec![
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (3, 1),
                (3, 2),
                (3, 3),
                (3, 4),
            ],
            vec![
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (3, 0),
                (3, 1),
                (3, 2),
                (3, 3),
            ],
        ];
        for (barrier, distance) in barriers.into_iter().zip(distances) {
            let mut alg = Algorithm::new(5, 5);
            for p in barrier.clone() {
                alg.grid[p] = -2;
            }
            let path = alg.shorted_path(origin, dst).unwrap();
            print_result(&alg, &path, &barrier);
            assert_eq!(path.len() - 1, distance);
        }
    }

    #[test]
    fn diagonal_barrier() {
        let n = 10;
        let origin = (0, 0);
        let barrier: Vec<_> = (0..n - 2)
            .map(|offset| (1 + offset, n - 2 - offset))
            .collect();
        let distances = [9, 9, 18];
        let dsts = [(0, 9), (9, 0), (9, 9)];
        for (dst, distance) in dsts.into_iter().zip(distances) {
            let mut alg = Algorithm::new(n, n);
            for p in barrier.clone() {
                alg.grid[p] = -2;
            }
            let path = alg.shorted_path(origin, dst).unwrap();
            print_result(&alg, &path, &barrier);
            assert_eq!(path.len() - 1, distance);
        }
    }

    #[test]
    fn horizontal_barrier() {
        let n = 10;
        let origin = (0, 0);
        let barrier = vec![
            (1, 0),
            (1, 1),
            (1, 2),
            (1, 3),
            (3, 0),
            (3, 1),
            (3, 2),
            (3, 3),
        ];
        let distances = [9, 17, 18];
        let dsts = [(0, 9), (9, 0), (9, 9)];
        for (dst, distance) in dsts.into_iter().zip(distances) {
            let mut alg = Algorithm::new(n, n);
            for p in barrier.clone() {
                alg.grid[p] = -2;
            }
            let path = alg.shorted_path(origin, dst).unwrap();
            print_result(&alg, &path, &barrier);
            assert_eq!(path.len() - 1, distance);
        }
    }

    #[test]
    fn escape() {
        let origin = (6, 4);
        let distances = [10, 11, 13, 14, 12];
        let dsts = [(0, 0), (0, 9), (9, 0), (9, 9), (8, 4)];
        let barrier: Vec<_> = (0..4)
            .map(|offset| (4 + offset, 3))
            .into_iter()
            .chain((0..4).map(|offset| (4 + offset, 5)))
            .chain(std::iter::once((7, 4)))
            .collect();

        for (dst, distance) in dsts.into_iter().zip(distances) {
            let mut alg = Algorithm::new(10, 10);
            for p in barrier.clone() {
                alg.grid[p] = -2;
            }
            let path = alg.shorted_path(origin, dst).unwrap();

            println!("{}", alg.grid);
            println!("{:#?}", path);
            print_result(&alg, &path, &barrier);
            assert_eq!(path.len() - 1, distance);
        }
    }
    #[test]
    fn single_move() {
        let origin = (6, 4);
        let dst = (7, 4);
        let mut alg = Algorithm::new(10, 10);
        let path = alg.shorted_path(origin, dst);
        assert_eq!(path.unwrap().len() - 1, 1);
    }
    #[test]
    fn no_movement() {
        let origin = (6, 4);
        let mut alg = Algorithm::new(10, 10);
        let path = alg.shorted_path(origin, origin);
        assert_eq!(path.unwrap().len() - 1, 0);
    }
}
