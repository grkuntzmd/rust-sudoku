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

use super::super::{Cell, Grid, Point, ALL_DIGITS, BOX, COL, ROW};
use log::Level;
use log::{info, log_enabled};

impl Grid {
    // hidden_triple removes candidates. When a candidate within a box appears only in a single column or row, that candidate can be removed from all cells in the column or row outside of the box. It returns true if it changes any cells.
    pub fn pointing_line(&mut self) -> bool {
        self.pointing_line_group("col", |p| COL.cells[p.1], |p| p.1) || self.pointing_line_group("row", |p| ROW.cells[p.0], |p| p.0)
    }

    fn pointing_line_group(&mut self, group: &str, sel: fn(&Point) -> [Point; 9], axis: fn(&Point) -> usize) -> bool {
        let mut res = false;
        for (bi, b) in BOX.cells.iter().enumerate() {
            let points = self.digit_points(b);

            // Loop through the digits and determine if all of them are on the same line (col or row). If so, then all other cells in that line that are not in the current box can have those digits removed.
            'outer: for d in ALL_DIGITS {
                if points[d].len() == 0 {
                    return false;
                }
                let a = axis(&points[d][0]);
                for p in points[d][1..].iter() {
                    if axis(p) != a {
                        continue 'outer;
                    }
                }

                for p in sel(&points[d][0]).iter() {
                    if p.0 / 3 * 3 + p.1 / 3 == bi {
                        continue;
                    }

                    if self[p].and_not(&Cell(1 << d)) {
                        cell_change!(self, res, "in box {} removing {} from {:?} along {}", bi, d, p, group);
                    }
                }
            }
        }
        res
    }
}
