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

use super::super::{Cell, Grid, Group, ALL_DIGITS, BOX, COL, ROW};
use log::Level;
use log::{info, log_enabled};

impl Grid {
    // hidden_pair removes other digits from a pair of cells in a group (box, column, row) when that pair contains the only occurrances of the digits in the group and returns true if it changes any cells.
    pub fn hidden_pair(&mut self) -> bool {
        self.hidden_pair_group(&BOX) || self.hidden_pair_group(&COL) || self.hidden_pair_group(&ROW)
    }

    fn hidden_pair_group(&mut self, group: &Group) -> bool {
        group_loop!(self, res, group, ci, c, {
            let points = self.digit_points(c);

            for i1 in ALL_DIGITS {
                for i2 in ALL_DIGITS {
                    if i1 == i2 || points[i1].len() != 2 || points[i2].len() != 2 {
                        continue;
                    }

                    if points[i1] == points[i2] {
                        let comb = Cell(1 << i1 | 1 << i2);
                        for k in 0..2 {
                            let p = points[i1][k];
                            if self[&p].and(&comb) {
                                cell_change!(self, res, "in {} {} limits {:?} to {}", group.name, ci, p, comb.to_string());
                            }
                        }
                    }
                }
            }
        })
    }
}
