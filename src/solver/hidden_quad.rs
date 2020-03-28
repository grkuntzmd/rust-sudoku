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

use super::{count, Cell, Grid, Group, ALL_DIGITS, BOX, COL, ROW};
use log::Level;
use log::{info, log_enabled};

impl Grid {
    // hidden_triple removes other digits from a quad of cells in a group (box, column, row) when that quad contains the only occurrances of the digits in the group. It returns true if it changes any cells.
    pub fn hidden_quad(&mut self) -> bool {
        self.hidden_quad_group(&BOX) || self.hidden_quad_group(&COL) || self.hidden_quad_group(&ROW)
    }

    fn hidden_quad_group(&mut self, group: &Group) -> bool {
        group_loop!(self, res, group, ci, c, {
            let places = self.digit_places(c);

            for i1 in ALL_DIGITS {
                let p1 = places[i1];
                let c1 = count(p1);
                if c1 == 1 || c1 > 4 {
                    continue;
                }

                for i2 in ALL_DIGITS {
                    if i1 == i2 {
                        continue;
                    }

                    let p2 = places[i2];
                    let c2 = count(p2);
                    if c2 == 1 || c2 > 4 || count(p1 | p2) > 4 {
                        continue;
                    }

                    for i3 in ALL_DIGITS {
                        if i1 == i3 || i2 == i3 {
                            continue;
                        }

                        let p3 = places[i3];
                        let c3 = count(p3);
                        if c3 == 1 || c3 > 4 || count(p1 | p2 | p3) > 4 {
                            continue;
                        }

                        for i4 in ALL_DIGITS {
                            if i1 == i4 || i2 == i4 || i3 == i4 {
                                continue;
                            }

                            let p4 = places[i4];
                            let c4 = count(p4);
                            let comb = p1 | p2 | p3 | p4;
                            if c4 == 1 || c4 > 4 || count(comb) != 4 {
                                continue;
                            }

                            let bits = Cell(1 << i1 | 1 << i2 | 1 << i3 | 1 << i4);
                            for (pi, p) in c.iter().enumerate() {
                                if comb & (1 << pi) != 0 {
                                    if self[p].and(&bits) {
                                        res = true;
                                        info!(
                                            "in {} {} limits {:?} to {}",
                                            group.name,
                                            ci,
                                            p,
                                            bits.to_string()
                                        );
                                        if log_enabled!(Level::Debug) {
                                            self.display();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}
