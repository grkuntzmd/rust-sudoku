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

use super::{Cell, Grid, Group, Point, ALL_DIGITS, BOX, COL, ROW};
use log::Level;
use log::{info, log_enabled};

impl Grid {
    // box_line removes candidates. When a candidate within a column or row appears only in a single box that candidate can be removed from all cells in the box, other than those in the column or row. It returns true if it changes any cells.
    pub fn box_line(&mut self) -> bool {
        let col = |p: &Point| p.1;
        let row = |p: &Point| p.0;

        self.box_line_group(&COL, col, row, |i, c| i * 3 + c / 3)
            || self.box_line_group(&ROW, row, col, |i, r| r / 3 * 3 + i)
    }

    fn box_line_group(
        &mut self,
        group: &Group,
        major: fn(&Point) -> usize,
        minor: fn(&Point) -> usize,
        box_sel: fn(usize, usize) -> usize,
    ) -> bool {
        group_loop!(self, res, group, ci, c, {
            let mut boxes = [[false; 3]; 10];
            for p in c {
                let cell = self[p];
                for d in ALL_DIGITS {
                    if cell.0 & (1 << d) != 0 {
                        boxes[d][minor(p) / 3] = true;
                    }
                }
            }

            for d in ALL_DIGITS {
                let index = if boxes[d][0] && !boxes[d][1] && !boxes[d][2] {
                    0
                } else if !boxes[d][0] && boxes[d][1] && !boxes[d][2] {
                    1
                } else if !boxes[d][0] && !boxes[d][1] && boxes[d][2] {
                    2
                } else {
                    continue;
                };

                for pi in 0..9 {
                    let p = BOX.cells[box_sel(index, ci)][pi];
                    if major(&p) == major(&c[index]) {
                        continue;
                    }

                    if self[&p].and_not(&Cell(1 << d)) {
                        res = true;
                        info!(
                            "all {}'s in {} {} appear in box {} removing from {:?}",
                            d,
                            group.name,
                            ci,
                            box_sel(index, ci),
                            p
                        );
                        if log_enabled!(Level::Debug) {
                            self.display();
                        }
                    }
                }
            }
        })
    }
}
