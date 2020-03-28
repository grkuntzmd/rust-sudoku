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
use rayon::prelude::*;
use solver::Grid;
use std::collections::HashSet;
use std::fs;

mod solver;

// Maximum difficulty level found on solving.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Level {
    Easy,
    Standard,
    Hard,
    Expert,
    Extreme,
}

static mut COLORIZE: bool = false;

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
                .long("standard")
                .value_name("COUNT")
                .takes_value(true)
                .help("Number of standard puzzles to generate"),
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
                .long("expert")
                .value_name("COUNT")
                .takes_value(true)
                .help("Number of expert puzzles to generate"),
        )
        .arg(
            Arg::with_name("level4")
                .short("4")
                .long("extreme")
                .value_name("COUNT")
                .takes_value(true)
                .help("Number of extreme puzzles to generate"),
        )
        .arg(
            Arg::with_name("attempts")
                .short("a")
                .long("attempts")
                .value_name("ATTEMPTS")
                .takes_value(true)
                .help("Number of attempts to generate a puzzle"),
        )
        .arg(
            Arg::with_name("colorize")
                .short("c")
                .long("colorize")
                .help("Colorize the output using ANSI escapes"),
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
    // let level_4_count = value_t!(matches, "level4", u32).unwrap_or(0);
    let inputs = values_t!(matches, "inputs", String).unwrap_or_else(|_e| Vec::new());
    let max_attempts = value_t!(matches, "attempts", u32).unwrap_or(100);
    unsafe {
        COLORIZE = value_t!(matches, "colorize", bool).unwrap_or(false);
    }

    if inputs.len() > 0 {
        // Handle -i files.
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

                        let mut strategies = HashSet::<&'static str>::new();
                        let (max_level, solved) = grid.reduce(&mut Some(&mut strategies));
                        grid.display();

                        let mut names: Vec<&str> = strategies.into_iter().collect();
                        names.sort();

                        if solved {
                            sol += 1;
                            println!("level: {:?}, solved, ({})", max_level, names.join(", "));
                        } else {
                            println!("level: {:?}, not solved ({})", max_level, names.join(", "));

                            let mut solutions = Vec::<Grid>::new();
                            grid.search(&mut solutions);

                            match solutions.len() {
                                0 => println!("still not solved after search"),
                                1 => {
                                    sol += 1;
                                    println!("single solution found");
                                    solutions[0].display();
                                }
                                _ => {
                                    println!("multiple solutions found");
                                    for s in solutions {
                                        s.display();
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => eprintln!("cannot open \"{}\" for reading", &input),
            }
        }
        println!("solved {} of {}", sol, all);
    } else {
        // Generate puzzles of levels given in -0, -1, -2, -3, -4.
        let mut tasks = Vec::<Level>::new();

        for _ in 0..level_0_count {
            tasks.push(Level::Easy)
        }
        for _ in 0..level_1_count {
            tasks.push(Level::Standard)
        }
        for _ in 0..level_2_count {
            tasks.push(Level::Hard)
        }
        for _ in 0..level_3_count {
            tasks.push(Level::Expert)
        }
        // for _ in 0..level_4_count { tasks.push(Level::Extreme)}

        tasks
            .par_iter()
            .map(|l| Grid::generate(l, &max_attempts))
            .for_each(|maybe_game| {
                if let Some(game) = maybe_game {
                    println!("{:?} ({}) {:?}", game.level, game.clues, game.strategies);
                    game.puzzle.display();
                    game.solution.display();
                }
            });
    }
}
