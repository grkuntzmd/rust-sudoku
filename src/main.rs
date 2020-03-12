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

extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate log;

use clap::{crate_authors, crate_version, value_t, values_t, App, Arg};
use solver::grid::{search, Grid, Solution};
use std::fs;

mod solver;

// Maximum difficulty level found on solving.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Level {
    Easy,
    Medium,
    Hard,
    Ridiculous,
    Insane,
}

fn main() {
    env_logger::init();

    let matches = App::new("Sudoku Generator")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Generate sudoku puzzles")
        .arg(
            Arg::with_name("inputs")
                .short("i")
                .long("inputs")
                .value_name("FILE")
                .takes_value(true)
                .multiple(true)
                .help("Read standard-format sudoku puzzle initial layout files"),
        )
        .arg(
            Arg::with_name("level0")
                .short("0")
                .long("easy")
                .value_name("COUNT")
                .takes_value(true)
                .help("Number of easy puzzles to generate"),
        )
        .arg(
            Arg::with_name("level1")
                .short("1")
                .long("medium")
                .value_name("COUNT")
                .takes_value(true)
                .help("Number of medium puzzles to generate"),
        )
        .arg(
            Arg::with_name("level2")
                .short("2")
                .long("hard")
                .value_name("COUNT")
                .takes_value(true)
                .help("Number of hard puzzles to generate"),
        )
        .arg(
            Arg::with_name("level3")
                .short("3")
                .long("ridiculous")
                .value_name("COUNT")
                .takes_value(true)
                .help("Number of ridiculous puzzles to generate"),
        )
        .arg(
            Arg::with_name("level4")
                .short("4")
                .long("insane")
                .value_name("COUNT")
                .takes_value(true)
                .help("Number of insane puzzles to generate"),
        )
        .after_help(
            format!(
                "build timestamp: {}\ngit hash: {}",
                env!("BUILD_TIMESTAMP"),
                env!("GIT_HASH")
            )
            .as_str(),
        )
        .get_matches();

    let level_0_count = value_t!(matches, "level0", u32).unwrap_or(0);
    let level_1_count = value_t!(matches, "level1", u32).unwrap_or(0);
    let level_2_count = value_t!(matches, "level2", u32).unwrap_or(0);
    let level_3_count = value_t!(matches, "level3", u32).unwrap_or(0);
    let level_4_count = value_t!(matches, "level4", u32).unwrap_or(0);
    let inputs = values_t!(matches, "inputs", String).unwrap_or_else(|_e| Vec::new());

    if inputs.len() > 0 {
        let mut all = 0;
        let mut sol = 0;
        for input in inputs {
            let lines = fs::read_to_string(&input);
            match lines {
                Ok(lines) => {
                    for line in lines.lines() {
                        all += 1;
                        println!("Encoded: {}", line);
                        let mut grid = Grid::parse_grid(&line);
                        grid.display();
                        let (level, solved) = grid.reduce();
                        grid.display();
                        println!(
                            "{:?}: {}",
                            level,
                            if solved { "solved" } else { "not solved" }
                        );

                        if !solved {
                            println!("Searching");
                            match search(&grid, None) {
                                Solution::NotFound => println!("No solution"),
                                Solution::Single(grid) => {
                                    grid.display();
                                    println!("solved by search");
                                    sol += 1;
                                }
                                Solution::Multiple(sol1, sol2) => {
                                    sol1.display();
                                    sol2.display();
                                    println!("Multiple solution");
                                },
                            }
                        } else {
                            sol += 1;
                        }
                    }
                }
                Err(_) => eprintln!("cannot open \"{}\" for reading", &input),
            }
        }
        println!("solved {} of {}", sol, all);
    } else {
        println!("level 0 count: {}", level_0_count);
        println!("level 1 count: {}", level_1_count);
        println!("level 2 count: {}", level_2_count);
        println!("level 3 count: {}", level_3_count);
        println!("level 4 count: {}", level_4_count);
    }
}
