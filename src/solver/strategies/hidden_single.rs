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
    // hidden_single solves a cell if it contains the only instance of a digit within its group (box, column, row) and returns true if it changes any cells.
    pub fn hidden_single(&mut self) -> bool {
        self.hidden_single_group(&BOX)
            || self.hidden_single_group(&COL)
            || self.hidden_single_group(&ROW)
    }

    fn hidden_single_group(&mut self, group: &Group) -> bool {
        group_loop!(self, res, group, ci, c, {
            let points = self.digit_points(c);

            for d in ALL_DIGITS {
                if points[d].len() == 1 {
                    let p = points[d][0];
                    if self[&p].replace(&Cell(1 << d)) {
                        cell_change!(self, res, "in {} {} set {:?} to {}", group.name, ci, p, d);
                    }
                }
            }
        })
    }
}
