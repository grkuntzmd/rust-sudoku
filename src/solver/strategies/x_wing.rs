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

use super::super::{Cell, Grid, Group, ALL_DIGITS, COL, ROW};
use log::Level;
use log::{info, log_enabled};

impl Grid {
    // x_wing removes candidates. If in 2 columns, say 0 and 7, all instances of a particular digit, say 4, appear in the same two rows, say 4 and 6, then 1 of the 4's must be in (0, 4) or (0, 6) and the other in (7, 4) or (7, 6). Therefore all of the other 4's in those two rows can be removed. The same logic applies if rows and columns are swapped. It returns true if it changes any cells.
    pub fn x_wing(&mut self) -> bool {
        self.x_wing_group(&COL, &ROW) || self.x_wing_group(&ROW, &COL)
    }

    fn x_wing_group(&mut self, major_group: &Group, minor_group: &Group) -> bool {
        let mut res = false;
        let mut digits = [[Cell(0); 10]; 9];
        for (ui, u) in major_group.cells.iter().enumerate() {
            for (pi, p) in u.iter().enumerate() {
                let cell = self[p];
                for d in ALL_DIGITS {
                    if cell.0 & (1 << d) != 0 {
                        digits[ui][d].0 |= 1 << pi;
                    }
                }
            }
        }

        for d in ALL_DIGITS {
            for c1i in 0..9 {
                for c2i in 0..9 {
                    if c1i == c2i {
                        continue;
                    }

                    let proto = digits[c1i][d];
                    if proto.count() == 2 && proto == digits[c2i][d] {
                        for minor in ALL_DIGITS {
                            if proto.0 & (1 << minor) != 0 {
                                for (mi, m) in minor_group.cells[minor].iter().enumerate() {
                                    if mi == c1i || mi == c2i {
                                        continue;
                                    }

                                    if self[m].and_not(&Cell(1 << d)) {
                                        cell_change!(
                                            self,
                                            res,
                                            "in {}s {} and {}, {} appears only in {} {} and 1 other; removing from {:?}",
                                            major_group.name,
                                            c1i,
                                            c2i,
                                            d,
                                            minor_group.name,
                                            minor,
                                            m
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        res
    }
}
