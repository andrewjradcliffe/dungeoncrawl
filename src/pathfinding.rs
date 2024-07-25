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
    pub fn paths(&mut self, src: (usize, usize), dst: (usize, usize)) -> Vec<Vec<(usize, usize)>> {
        if !self.grid.check_bounds(src) || !self.grid.check_bounds(dst) {
            panic!(
                "out of bounds! got (src, dst, bounds): ({:?}, {:?}, {:?})",
                src,
                dst,
                self.grid.shape()
            );
        }
        let (m, n) = self.grid.shape();
        self.reset();
        let mut ps = vec![vec![src]];
        let mut qs = Vec::new();
        let mut rs = Vec::new();
        self.grid[(src.0, src.1)] = 0;

        macro_rules! common {
            ($i:ident, $j:ident, $a_ij:ident, $p:ident, $k:ident) => {
                if ($i, $j) == dst {
                    let mut p = $p.clone();
                    p.push(($i, $j));
                    rs.push(p);
                    if $a_ij == -1 || $a_ij > $k {
                        self.grid[($i, $j)] = $k;
                    }
                } else if $a_ij == -1 || $a_ij > $k {
                    self.grid[($i, $j)] = $k;
                    let mut p = $p.clone();
                    p.push(($i, $j));
                    qs.push(p);
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
                        let a_ij = self.grid[(i, j)];
                        if (i, j) == dst {
                            let mut p = p.clone();
                            p.push((i, j));
                            rs.push(p);
                            if a_ij == -1 || a_ij > k {
                                self.grid[(i, j)] = k;
                            }
                        } else if a_ij == -1 || a_ij > k {
                            self.grid[(i, j)] = k;
                            let mut p = p.clone();
                            p.push((i, j));
                            qs.push(p);
                        }
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
                    }
                }
                // Backward
                {
                    if j > 0 {
                        let j = j - 1;
                        let a_ij = self.grid[(i, j)];
                        if (i, j) == dst {
                            let mut p = p.clone();
                            p.push((i, j));
                            rs.push(p);
                            if a_ij == -1 || a_ij > k {
                                self.grid[(i, j)] = k;
                            }
                        } else if a_ij == -1 || a_ij > k {
                            self.grid[(i, j)] = k;
                            let mut p = p.clone();
                            p.push((i, j));
                            qs.push(p);
                        }
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
                    }
                }
                // Up
                {
                    if i > 0 {
                        let i = i - 1;
                        let a_ij = self.grid[(i, j)];
                        if (i, j) == dst {
                            let mut p = p.clone();
                            p.push((i, j));
                            rs.push(p);
                            if a_ij == -1 || a_ij > k {
                                self.grid[(i, j)] = k;
                            }
                        } else if a_ij == -1 || a_ij > k {
                            self.grid[(i, j)] = k;
                            let mut p = p.clone();
                            p.push((i, j));
                            qs.push(p);
                        }
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
                    }
                }
                // Down
                {
                    let i = i + 1;
                    if i < m {
                        let a_ij = self.grid[(i, j)];
                        if (i, j) == dst {
                            let mut p = p.clone();
                            p.push((i, j));
                            rs.push(p);
                            if a_ij == -1 || a_ij > k {
                                self.grid[(i, j)] = k;
                            }
                        } else if a_ij == -1 || a_ij > k {
                            self.grid[(i, j)] = k;
                            let mut p = p.clone();
                            p.push((i, j));
                            qs.push(p);
                        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five() {
        let mut alg = Algorithm::new(5, 5);
        let paths = alg.paths((0, 0), (2, 2));
        println!("{}", alg.grid);
        println!("{:#?}", paths);
        panic!();
    }
}
