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
    // naked_single removes digits from other items in a group (box, column, row) when a cell contains a solved value and returns true if it changes any cells.
    pub fn naked_single(&mut self) -> bool {
        self.naked_single_group(&BOX)
            || self.naked_single_group(&COL)
            || self.naked_single_group(&ROW)
    }

    fn naked_single_group(&mut self, group: &Group) -> bool {
        group_loop!(self, res, group, ci, c, {
            for p1 in c {
                let val = self[p1];
                if val.count() != 1 {
                    continue;
                }

                for p2 in c {
                    if p1 == p2 {
                        continue;
                    }

                    if self[p2].and_not(&val) {
                        res = true;
                        info!(
                            "in {} {} cell {:?} allows only {}, removed from {:?}",
                            group.name,
                            ci,
                            p1,
                            val.digits(),
                            p2
                        );
                        if log_enabled!(Level::Trace) {
                            self.display();
                        }
                    }
                }
            }
        })
    }
}
