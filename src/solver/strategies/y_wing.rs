/*
 * Copyright © 2020, G.Ralph Kuntz, MD.
 *
 * Licensed under the Apache License, Version 2.0(the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIC
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::super::{box_of, Grid, Point, BOX, COL, COLS, ROW, ROWS};
use super::neighbors;
use log::Level;
use log::{info, log_enabled};
use std::collections::HashSet;

impl Grid {
    // y_wing removes candidates. If a cell has two candidates (AB) and in a neighboring unit (box, row, or column) of AB is another cell containing AC and in a second neighboring unit of AB is a cell containing BC, then any cell that can be "seen" by AC and BC (in both neighborhoods of AC and BC) that contain C can have C removed. It returns true if it changes any cells.
    pub fn y_wing(&mut self) -> bool {
        let mut res = false;

        // Traverse all cells, using box units for convenience.
        for u in &BOX.cells {
            for p in u {
                let cell = self[&p];

                if cell.count() != 2 {
                    continue;
                }

                let clone = self.clone();
                let candidates = clone.find_y_wing_candidates(p, 1);

                for (c1i, p1) in candidates.iter().enumerate() {
                    let cell1 = self[&p1];
                    let n1 = neighbors(&p1);

                    for (c2i, p2) in candidates.iter().enumerate() {
                        if c1i == c2i {
                            continue;
                        }

                        let cell2 = self[&p2];

                        if (cell1 | cell2).count() != 3 || (cell & cell1 | cell & cell2) != cell {
                            continue;
                        }

                        let n2 = neighbors(&p2);

                        let mut overlap = [[false; COLS]; ROWS];
                        for_all_cells!(r, c, {
                            overlap[r][c] = n1[r][c] && n2[r][c];
                        });

                        overlap[p.0][p.1] = false;

                        for_all_cells!(r, c, {
                            if overlap[r][c] {
                                let bits = (cell1 | cell2) & !cell;
                                if self.cells[r][c].and_not(&bits) {
                                    cell_change!(self, res, "{:?}, {:?}, {:?} causes clearing of {} from ({}, {})", p, p1, p2, bits, r, c);
                                }
                            }
                        });
                    }
                }
            }
        }

        res
    }

    fn find_y_wing_candidates(&self, curr: &Point, overlap: usize) -> Vec<&Point> {
        let mut set = HashSet::<&Point>::new();

        for p in self.find_y_wing_candidates_unit(&BOX.cells[box_of(curr.0, curr.1)], curr, overlap) {
            set.insert(p);
        }

        for p in self.find_y_wing_candidates_unit(&COL.cells[curr.1], curr, overlap) {
            set.insert(p);
        }

        for p in self.find_y_wing_candidates_unit(&ROW.cells[curr.0], curr, overlap) {
            set.insert(p);
        }

        set.into_iter().collect()
    }

    fn find_y_wing_candidates_unit<'a>(&self, unit: &'a [Point; 9], curr: &Point, overlap: usize) -> HashSet<&'a Point> {
        let mut set = HashSet::<&Point>::new();
        let cell = self[&curr];

        for p in unit {
            if p == curr {
                continue;
            }

            let candidate = self[&p];
            if candidate.count() != 2 || (cell & candidate).count() != overlap {
                continue;
            }

            set.insert(p);
        }

        set
    }
}
