use crate::maze::{core::*, element::*};

use Element::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MazeGraph(pub(crate) Vec<Maze>);

impl MazeGraph {
    pub fn single_connect(
        &mut self,
        i: usize,
        src_position: (usize, usize),
        j: usize,
        dst_position: (usize, usize),
    ) {
        let n = self.0.len();
        assert!(i < n);
        assert!(j < n);
        self.0[i].create_portal(
            src_position,
            Destination {
                index: j,
                position: dst_position,
            },
        );
    }
    pub fn twoway_connect(
        &mut self,
        i: usize,
        src_portal: (usize, usize),
        j: usize,
        dst_portal: (usize, usize),
    ) {
        let n = self.0.len();
        assert!(i < n);
        assert!(j < n);
        if i == j {
            if let Some(dst_position) = self.0[j].first_portal_proposal(dst_portal, src_portal) {
                if let Some(src_position) = self.0[i].first_portal_proposal(src_portal, dst_portal)
                {
                    self.single_connect(i, src_portal, j, dst_position);
                    self.single_connect(j, dst_portal, i, src_position);
                }
            }
        } else {
            if let Some(dst_position) = self.0[j].first_movement_proposal(dst_portal) {
                if let Some(src_position) = self.0[i].first_movement_proposal(src_portal) {
                    self.single_connect(i, src_portal, j, dst_position);
                    self.single_connect(j, dst_portal, i, src_position);
                }
            }
        }
    }

    pub fn remove(&mut self, i_star: usize) {
        self.0.remove(i_star);
        let mut t = Vec::new();
        for node in self.0.iter_mut() {
            for pos in node.active_portals.drain(..) {
                let e = &mut node.grid[pos];
                match e {
                    ActivePortal(dst) => {
                        if dst.index < i_star {
                            t.push(pos);
                        } else if dst.index > i_star {
                            dst.index -= 1;
                            t.push(pos);
                        } else
                        /* dst.index == i_star */
                        {
                            *e = InactivePortal;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            node.active_portals.append(&mut t);
        }
    }
}

impl MazeGraph {
    pub fn new_demo() -> Self {
        let starting_area = Maze::new_demo();
        let room1 = Maze::new_room();
        let room2 = Maze::new_room();
        let mut graph = Self(vec![starting_area, room1, room2]);
        graph.single_connect(0, (0, 0), 1, (1, 2));
        graph.single_connect(1, (0, 2), 0, (0, 1));
        graph.single_connect(0, (8, 8), 2, (1, 2));
        graph.single_connect(2, (0, 2), 0, (8, 9));
        graph.single_connect(2, (4, 2), 0, (9, 8));
        graph.twoway_connect(0, (13, 13), 0, (13, 14));
        graph.twoway_connect(0, (0, 5), 0, (17, 17));
        graph
    }
}
