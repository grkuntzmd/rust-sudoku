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

use super::{box_of, Point, BOX, COL, COLS, ROW, ROWS};

pub mod box_line;
pub mod hidden_pair;
pub mod hidden_quad;
pub mod hidden_single;
pub mod hidden_triple;
pub mod naked_pair;
pub mod naked_single;
pub mod naked_triple;
pub mod pointing_line;
pub mod x_wing;
pub mod y_wing;

fn neighbors(curr: &Point) -> [[bool; COLS]; ROWS] {
    let mut points = [[false; COLS]; ROWS];

    for u in vec![&BOX.cells[box_of(curr.0, curr.1)], &COL.cells[curr.1], &ROW.cells[curr.0]] {
        for p in u {
            if p == curr {
                continue;
            }

            points[p.0][p.1] = true;
        }
    }

    points
}
