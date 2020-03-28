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

use super::{Grid, Group, BOX, COL, ROW};
use log::Level;
use log::{info, log_enabled};

impl Grid {
    // naked_triple checks a group for 3 cells with the same triple of values. If present, those values can be removed from all other cells in the group. It returns true if it changes any cells.
    pub fn naked_triple(&mut self) -> bool {
        self.naked_triple_group(&BOX)
            || self.naked_triple_group(&COL)
            || self.naked_triple_group(&ROW)
    }

    fn naked_triple_group(&mut self, group: &Group) -> bool {
        group_loop!(self, res, group, ci, c, {
            for p1 in c {
                let cell1 = self[p1];
                let count = cell1.count();
                if count == 1 || count > 3 {
                    continue;
                }

                for p2 in c {
                    if p1 == p2 {
                        continue;
                    }

                    let cell2 = self[p2];
                    let count = cell2.count();
                    if count == 1 || count > 3 {
                        continue;
                    }

                    if (cell1 | cell2).count() > 3 {
                        continue;
                    }

                    for p3 in c {
                        if p1 == p3 || p2 == p3 {
                            continue;
                        }

                        let cell3 = self[p3];
                        let count = cell3.count();
                        if count == 1 || count > 3 {
                            continue;
                        }

                        let comb = cell1 | cell2 | cell3;
                        if comb.count() > 3 {
                            continue;
                        }

                        for p in c {
                            if p1 == p || p2 == p || p3 == p {
                                continue;
                            }

                            if self[p].and_not(&comb) {
                                cell_change!(self, res, "in {} {} ({:?}, {:?}, {:?}) removing {} from {:?}", group.name, ci, p1, p2, p3, comb.to_string(), p);
                            }
                        }
                    }
                }
            }
        })
    }
}
