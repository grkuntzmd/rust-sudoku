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

macro_rules! group_loop {
    ($self_:ident, $res:ident, $group:ident, $ci:ident, $c:ident, $body:tt) => {{
        let mut $res = false;
        for ($ci, $c) in $group.cells.iter().enumerate() {
            $body
        }

        $res
    }};
}

use super::super::Level;
use super::{Group, Point, Unit};
use cell::{Cell, BIT_COUNT};
use colored::Colorize;
use std::ops::RangeInclusive;
use std::ops::{Index, IndexMut};

mod cell;
mod hidden_pair;
mod hidden_quad;
mod hidden_single;
mod hidden_triple;
mod naked_pair;
mod naked_single;
mod naked_triple;
mod pointing_line;

const ALL_DIGITS: RangeInclusive<usize> = RangeInclusive::new(1, 9);

const ROWS: usize = 9;
const COLS: usize = 9;

const BOT_LEFT: &str = "\u{2514}";
const BOT_RIGHT: &str = "\u{2518}";
const BOT_T: &str = "\u{2534}";
const HORIZ_BAR: &str = "\u{2500}";
const LEFT_T: &str = "\u{251c}";
const PLUS: &str = "\u{253c}";
const RIGHT_T: &str = "\u{2524}";
const TOP_LEFT: &str = "\u{250c}";
const TOP_RIGHT: &str = "\u{2510}";
const TOP_T: &str = "\u{252c}";
const VERT_BAR: &str = "\u{2502}";

lazy_static! {
    static ref BOX: Group = {
        let mut cells = [[(0, 0); ROWS]; COLS];
        for r in 0..ROWS {
            for c in 0..COLS {
                let p = (r, c);
                cells[box_of(r, c)][r % 3 * 3 + c % 3] = p;
            }
        }
        Group {
            name: "box".to_string(),
            cells,
        }
    };
    static ref COL: Group = {
        let mut cells = [[(0, 0); ROWS]; COLS];
        for r in 0..ROWS {
            for c in 0..COLS {
                let p = (r, c);
                cells[c][r] = p;
            }
        }
        Group {
            name: "col".to_string(),
            cells,
        }
    };
    static ref ROW: Group = {
        let mut cells = [[(0, 0); ROWS]; COLS];
        for r in 0..ROWS {
            for c in 0..COLS {
                let p = (r, c);
                cells[r][c] = p;
            }
        }
        Group {
            name: "row".to_string(),
            cells,
        }
    };
}

#[derive(Debug)]
pub struct Grid {
    pub orig: [[bool; ROWS]; COLS],
    pub cells: [[Cell; ROWS]; COLS],
}

impl Grid {
    // display prints a game.
    pub fn display(&self) {
        let width = self.max_width() + 2;
        let bars = HORIZ_BAR.repeat((width * 3) as usize);
        let line = LEFT_T.to_string()
            + &[bars.as_str(), bars.as_str(), bars.as_str()].join(PLUS)
            + RIGHT_T;
        println!(
            "\t{}{}{}{}{}{}{}",
            TOP_LEFT, &bars, TOP_T, &bars, TOP_T, &bars, TOP_RIGHT
        );
        for r in 0..ROWS {
            print!("\t\u{2502}");
            for c in 0..COLS {
                let mut s = String::new();
                for i in ALL_DIGITS {
                    if self.cells[r][c].0 & (1 << i) != 0 {
                        s += &i.to_string();
                    }
                }
                let orig = self.orig[r][c];
                if s == "123456789" {
                    print!("{}", center(".", width));
                } else {
                    if orig {
                        print!("{}", center(&s, width).green());
                    } else {
                        print!("{}", center(&s, width));
                    };
                }
                if c == 2 || c == 5 {
                    print!("{}", VERT_BAR);
                }
            }
            println!("{}", VERT_BAR);
            if r == 2 || r == 5 {
                println!("\t{}", line);
            }
        }
        println!(
            "\t{}{}{}{}{}{}{}",
            BOT_LEFT, &bars, BOT_T, &bars, BOT_T, &bars, BOT_RIGHT
        );
    }

    // digit_places returns an array of digits containing values where the bits (1 - 9) are set if the corresponding digit appears in that cell.
    fn digit_places(&self, unit: &Unit) -> [u16; 10] {
        let mut places = [0; 10];
        for (pi, p) in unit.iter().enumerate() {
            let cell = self[p];
            for d in ALL_DIGITS {
                if cell.0 & (1 << d) != 0 {
                    places[d] |= 1 << pi;
                }
            }
        }
        places
    }

    // digit_points builds a table of points that contain each digit.
    fn digit_points(&self, unit: &Unit) -> [Vec<Point>; 10] {
        let mut points: [Vec<Point>; 10] = Default::default();
        for p in unit {
            let val = self[p];
            for d in ALL_DIGITS {
                if val.0 & (1 << d) != 0 {
                    points[d].push(*p);
                }
            }
        }

        points
    }

    // max_width calculates the maximum width that any cell in the grid takes to display. A grid containing all of the digits ("123456789") will display as a single dot (".") and so has a width of 1.
    fn max_width(&self) -> usize {
        let mut width = 0;
        for r in 0..ROWS {
            for c in 0..COLS {
                let mut count = self.cells[r][c].count();
                if count == 9 {
                    count = 1;
                }
                if width < count {
                    width = count;
                }
            }
        }
        width
    }

    // parse_grid parses a string of digits and dots into a game structure containing two matrices: the orig matrix contains a 'true' where that cell was set in the start-up puzzle and curr contains u16's where the bits are set if that digit is valid. It panics on any illegal input.
    pub fn parse_grid(input: &str) -> Grid {
        let bytes = input.as_bytes();
        const ALL: u16 = 0b_111111111_0;
        let mut orig: [[bool; ROWS]; COLS] = Default::default();
        let mut cells = [[Cell(0); ROWS]; COLS];
        for r in 0..ROWS {
            for c in 0..COLS {
                let chr = bytes[r * 9 + c];
                if chr == b'.' {
                    cells[r][c].0 |= ALL;
                } else {
                    let digit: u16 = match atoi::ascii_to_digit(chr) {
                        Some(digit) => digit,
                        None => panic!("illegal character in input grid: {} (\"{}\")", &input, chr),
                    };
                    orig[r][c] = true;
                    cells[r][c].0 |= 1 << digit;
                }
            }
        }

        Grid {
            orig: orig,
            cells: cells,
        }
    }

    // Solve solves the current grid of the given game.
    pub fn solve(&mut self) -> (Level, bool) {
        self.display();

        let mut max_level = Level::Easy;
        while !self.validate() {
            if self.solve_level(
                &mut max_level,
                &Level::Easy,
                vec![
                    Grid::naked_single,
                    Grid::hidden_single,
                    Grid::naked_pair,
                    Grid::naked_triple,
                    Grid::hidden_pair,
                    Grid::hidden_triple,
                    Grid::hidden_quad,
                    Grid::pointing_line,
                ],
            ) {
                continue;
            }
            if self.solve_level(&mut max_level, &Level::Medium, vec![]) {
                continue;
            }
            if self.solve_level(&mut max_level, &Level::Hard, vec![]) {
                continue;
            }
            if self.solve_level(&mut max_level, &Level::Ridiculous, vec![]) {
                continue;
            }
            if self.solve_level(&mut max_level, &Level::Insane, vec![]) {
                continue;
            }
            break;
        }

        let valid = self.validate();
        if !valid {
            println!("Not solved");
        }
        self.display();
        println!("{:?}", max_level);
        (max_level, valid)
    }

    pub fn solve_level(
        &mut self,
        max_level: &mut Level,
        level: &Level,
        fns: Vec<fn(&mut Grid) -> bool>,
    ) -> bool {
        for f in fns {
            if f(self) {
                if *max_level < *level {
                    *max_level = *level;
                }
                return true;
            }
        }
        false
    }

    fn validate(&self) -> bool {
        self.validate_group(&BOX) && self.validate_group(&COL) && self.validate_group(&ROW)
    }

    fn validate_group(&self, group: &Group) -> bool {
        for c in &group.cells {
            let mut cells = [0; 10];
            for p in c {
                let val = self[p];
                let bits = val.count();
                if bits == 0 {
                    self.display();
                    panic!("Empty cell");
                }
                if self.orig[p.0][p.1] && bits > 1 {
                    panic!("Changed original cell: ({}, {}): {:b}", p.0, p.1, val.0);
                }
                for d in ALL_DIGITS {
                    if val.0 & (1 << d) != 0 {
                        cells[d] += 1;
                    }
                }
            }

            for d in ALL_DIGITS {
                if cells[d] != 1 {
                    return false;
                }
            }
        }

        true
    }
}

impl Index<&Point> for Grid {
    type Output = Cell;
    fn index<'a>(&'a self, p: &Point) -> &Cell {
        &self.cells[p.0][p.1]
    }
}

impl IndexMut<&Point> for Grid {
    fn index_mut<'a>(&'a mut self, p: &Point) -> &mut Cell {
        &mut self.cells[p.0][p.1]
    }
}

fn box_of(r: usize, c: usize) -> usize {
    r / 3 * 3 + c / 3
}

// center centers a string in a field of the given size.
fn center(s: &str, width: usize) -> String {
    format!("{:^width$}", s, width = width)
}

fn count<T: Into<usize>>(val: T) -> u8 {
    BIT_COUNT[val.into() as usize]
}
