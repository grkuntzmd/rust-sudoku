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

macro_rules! for_all_cells {
    ($r: ident, $c: ident, $body:tt) => {
        for $r in 0..ROWS {
            for $c in 0..COLS {
                $body
            }
        }
    };
}

macro_rules! group_loop {
    ($self_:ident, $res:ident, $group:ident, $ci:ident, $c:ident, $body:tt) => {{
        let mut $res = false;
        for ($ci, $c) in $group.cells.iter().enumerate() {
            $body
        }

        $res
    }};
}

extern crate rand;

use super::Level;
use cell::{Cell, BIT_COUNT};
use colored::Colorize;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::ops::{Index, IndexMut};
// use super::super::COLORIZE;

mod box_line;
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

const ALL: u16 = 0b_111111111_0;
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
        fn box_of(r: usize, c: usize) -> usize {
            r / 3 * 3 + c / 3
        }

        let mut cells = [[(0, 0); COLS]; ROWS];
        for_all_cells!(r, c, {
            let p = (r, c);
            cells[box_of(r, c)][r % 3 * 3 + c % 3] = p;
        });
        Group {
            name: "box".to_string(),
            cells,
        }
    };
    static ref COL: Group = {
        let mut cells = [[(0, 0); COLS]; ROWS];
        for_all_cells!(r, c, {
            let p = (r, c);
            cells[c][r] = p;
        });
        Group {
            name: "col".to_string(),
            cells,
        }
    };
    static ref ROW: Group = {
        let mut cells = [[(0, 0); ROWS]; COLS];
        for_all_cells!(r, c, {
            let p = (r, c);
            cells[r][c] = p;
        });
        Group {
            name: "row".to_string(),
            cells,
        }
    };
}

pub struct Group {
    pub name: String,
    pub cells: [Unit; 9],
}

pub type Point = (usize, usize);
type Unit = [Point; 9];

#[derive(Clone, Copy, Debug)]
pub struct Grid {
    pub orig: [[bool; COLS]; ROWS],
    pub cells: Cells,
}

type Cells = [[Cell; COLS]; ROWS];

pub struct Game {
    pub level: Level,
    pub clues: u8,
    pub strategies: String,
    pub puzzle: Box<Grid>,
    pub solution: Box<Grid>,
}

impl Grid {
    // generate creates a solvable random puzzle at the given difficulty level and returns it (Some(Grid)) or None if it fails to generate.
    pub fn generate(level: &Level, max_attempts: &u32) -> Option<Game> {
        let mut attempts = *max_attempts;

        'outer: loop {
            let mut grid = Grid::randomize();
            let mut solutions = Vec::<Grid>::new();
            grid.search(&mut solutions);

            if solutions.len() == 0 {
                // The grid has no solution.
                attempts -= 1;
                if attempts == 0 {
                    // If too many attempts, return `None`.
                    return None;
                }

                continue 'outer;
            }

            // From https://stackoverflow.com/a/7280517/96233.

            grid = solutions[0].clone();
            let mut points = grid.all_points();
            points.shuffle(&mut thread_rng());

            loop {
                let curr = match points.pop() {
                    Some(p) => p,
                    None => break,
                };
                let old = grid[&curr];
                grid[&curr] = Cell(ALL);

                solutions.truncate(0);
                grid.search(&mut solutions);

                // If the solution is no longer unique, put back the old value.
                if solutions.len() > 1 {
                    grid[&curr] = old;
                }
            }

            // At this point, grid contains the smallest solution that is unique. Now we test the level.
            let mut copy = grid.clone();
            let mut strategies = HashSet::<&'static str>::new();
            let (l, solved) = copy.reduce(&mut Some(&mut strategies));

            solutions.truncate(0);
            copy.search(&mut solutions);

            if solved && l == *level && solutions.len() == 1 {
                let mut solution = solutions[0];
                let mut clues: u8 = 0;

                for r in 0..ROWS {
                    for c in 0..COLS {
                        if grid.cells[r][c].count() == 1 {
                            solution.orig[r][c] = true;
                            clues += 1;
                        }
                    }
                }

                let mut s: Vec<&str> = strategies.into_iter().collect();
                s.sort();

                return Some(Game {
                    level: *level,
                    clues: clues,
                    strategies: s.join(", "),
                    puzzle: Box::<_>::new(grid),
                    solution: Box::<_>::new(solution),
                });
            }
        }
    }

    fn all_points(&self) -> Vec<Point> {
        let mut points = Vec::<Point>::new();
        for r in 0..ROWS {
            for c in 0..COLS {
                points.push((r, c))
            }
        }

        points
    }

    // display prints a game.
    pub fn display(&self) {
        let width = self.max_width() + 2;
        let bars = HORIZ_BAR.repeat((width * 3) as usize);
        let line = LEFT_T.to_string()
            + &[bars.as_str(), bars.as_str(), bars.as_str()].join(PLUS)
            + RIGHT_T;
        print!("\t   ");
        for d in 0..9 {
            print!("{}", center(&d.to_string(), width).yellow());
            if d == 2 || d == 5 {
                print!(" ");
            }
        }
        println!();
        println!(
            "\t  {}{}{}{}{}{}{}",
            TOP_LEFT, &bars, TOP_T, &bars, TOP_T, &bars, TOP_RIGHT
        );
        for r in 0..ROWS {
            print!("\t{} \u{2502}", r.to_string().yellow());
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
                println!("\t  {}", line);
            }
        }
        println!(
            "\t  {}{}{}{}{}{}{}",
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
            let cell = self[p];
            for d in ALL_DIGITS {
                if cell.0 & (1 << d) != 0 {
                    points[d].push(*p);
                }
            }
        }

        points
    }

    fn empty_cell(&self) -> bool {
        for_all_cells!(r, c, {
            if self.cells[r][c].0 == 0 {
                return true;
            }
        });
        false
    }

    // max_width calculates the maximum width that any cell in the grid takes to display. A grid containing all of the digits ("123456789") will display as a single dot (".") and so has a width of 1.
    fn max_width(&self) -> usize {
        let mut width = 0;
        for_all_cells!(r, c, {
            let mut count = self.cells[r][c].count();
            if count == 9 {
                count = 1;
            }
            if width < count {
                width = count;
            }
        });
        width
    }

    // parse_grid parses a string of digits and dots into a game structure containing two matrices: the orig matrix contains a 'true' where that cell was set in the start-up puzzle and curr contains u16's where the bits are set if that digit is valid. It panics on any illegal input.
    pub fn parse_grid(input: &str) -> Grid {
        let bytes = input.as_bytes();
        let mut orig: [[bool; COLS]; ROWS] = Default::default();
        let mut cells = [[Cell(0); COLS]; ROWS];
        for_all_cells!(r, c, {
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
        });
        Grid {
            orig: orig,
            cells: cells,
        }
    }

    // randomize generates a random grid.
    pub fn randomize() -> Grid {
        let mut cells = [[Cell(ALL); COLS]; ROWS];
        let mut rng = rand::thread_rng();
        let mut digits: Vec<u16> = (1..=9).collect();
        digits.shuffle(&mut thread_rng());
        let mut index = 0;
        for p in &BOX.cells[rng.gen_range(0, 9)] {
            cells[p.0][p.1] = Cell(1 << digits[index]);
            index += 1;
        }

        Grid {
            orig: [[false; COLS]; ROWS],
            cells,
        }
    }

    // reduce reduces all cells to the minimum number of candidates using only logical operations (no brute-froce search) and returns the highest level of operation used and a flag indicating if the puzzle is solved.
    pub fn reduce(&mut self, strategies: &mut Option<&mut HashSet<&'static str>>) -> (Level, bool) {
        if self.empty_cell() {
            return (Level::Easy, false);
        }

        let mut max_level = Level::Easy;
        loop {
            if self.solved() {
                return (max_level, true);
            }
            if self.reduce_level(
                &mut max_level,
                &Level::Easy,
                strategies,
                vec![
                    (Grid::naked_single, "naked_single"),
                    (Grid::hidden_single, "hidden_single"),
                    (Grid::naked_pair, "naked_pair"),
                    (Grid::naked_triple, "naked_triple"),
                    (Grid::hidden_pair, "hidden_pair"),
                    (Grid::hidden_triple, "hidden_triple"),
                    (Grid::hidden_quad, "hidden_quad"),
                    (Grid::pointing_line, "pointing_line"),
                    (Grid::box_line, "box_line"),
                ],
            ) {
                continue;
            }
            if self.reduce_level(&mut max_level, &Level::Standard, strategies, vec![]) {
                continue;
            }
            if self.reduce_level(&mut max_level, &Level::Hard, strategies, vec![]) {
                continue;
            }
            if self.reduce_level(&mut max_level, &Level::Expert, strategies, vec![]) {
                continue;
            }
            if self.reduce_level(&mut max_level, &Level::Extreme, strategies, vec![]) {
                continue;
            }
            break;
        }

        (max_level, false)
    }

    pub fn reduce_level(
        &mut self,
        max_level: &mut Level,
        level: &Level,
        strategies: &mut Option<&mut HashSet<&'static str>>,
        fns: Vec<(fn(&mut Grid) -> bool, &'static str)>,
    ) -> bool {
        for (f, n) in fns {
            if f(self) {
                if let Some(s) = strategies {
                    s.insert(n);
                }
                if *max_level < *level {
                    *max_level = *level;
                }
                return true;
            }
        }
        false
    }

    pub fn search<'a>(&'a self, solutions: &'a mut Vec<Grid>) {
        fn min_point(grid: &Grid) -> Option<Point> {
            let mut min = 10;
            let mut min_points = Vec::<Point>::new();
            let mut found = false;
            for_all_cells!(r, c, {
                let cell = grid.cells[r][c];
                let count = cell.count();
                if count > 1 && count < min {
                    min = count;
                    min_points.truncate(0);
                    min_points.push((r, c));
                    found = true;
                } else if count == min {
                    min_points.push((r, c));
                    found = true;
                }
            });

            if found {
                min_points.shuffle(&mut thread_rng());
                return Some(min_points[0]);
            }

            None
        }

        if self.solved() {
            solutions.push((*self).clone());
        }

        if self.empty_cell() {
            return;
        }

        let point = match min_point(self) {
            Some(p) => p,
            None => return,
        };

        let mut digits = self[&point].digits();
        digits.shuffle(&mut thread_rng());

        for d in digits {
            let mut copy = self.clone();
            copy[&point] = Cell(1 << d);
            let (_, solved) = copy.reduce(&mut None);

            if solved {
                solutions.push(copy);
                if solutions.len() > 1 {
                    return;
                }
                continue;
            }

            copy.search(solutions);
            if solutions.len() > 1 {
                return;
            }
        }
    }

    fn solved(&self) -> bool {
        self.solved_group(&BOX) && self.solved_group(&COL) && self.solved_group(&ROW)
    }

    fn solved_group(&self, group: &Group) -> bool {
        for ps in &group.cells {
            let mut cells = [0; 10];
            for p in ps {
                let cell = self[p];

                if self.orig[p.0][p.1] && cell.count() > 1 {
                    panic!("Changed original cell: ({}, {}): {:b}", p.0, p.1, cell.0);
                }

                if cell == Cell(0) {
                    return false;
                }

                for d in ALL_DIGITS {
                    if cell.0 & (1 << d) != 0 {
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

// center centers a string in a field of the given size.
fn center(s: &str, width: usize) -> String {
    format!("{:^width$}", s, width = width)
}

fn count<T: Into<usize>>(cell: T) -> u8 {
    BIT_COUNT[cell.into() as usize]
}
