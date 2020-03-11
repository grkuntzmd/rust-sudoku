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
    // naked_pair checks a group for 2 cells containing only the same pair of values. If present, those values can be removed from all other cells in the group. It returns true if it changes any cells.
    pub fn naked_pair(&mut self) -> bool {
        self.naked_pair_group(&BOX) || self.naked_pair_group(&COL) || self.naked_pair_group(&ROW)
    }

    fn naked_pair_group(&mut self, group: &Group) -> bool {
        group_loop!(self, res, group, ci, c, {
            'outer: for p1 in c {
                let cell1 = self[p1];
                if cell1.count() != 2 {
                    continue;
                }
                for p2 in c {
                    if p1 == p2 {
                        continue;
                    }

                    let cell2 = self[p2];
                    if cell1 != cell2 {
                        continue;
                    }

                    for p3 in c {
                        if p1 == p3 || p2 == p3 {
                            continue;
                        }

                        if self[p3].and_not(&cell1) {
                            res = true;
                            info!(
                                "in {} {} removed {} from {:?}",
                                group.name,
                                ci,
                                cell1.digits(),
                                p3
                            );
                            if log_enabled!(Level::Debug) {
                                self.display();
                            }
                        }
                    }
                    continue 'outer;
                }
            }
        })
    }
}
