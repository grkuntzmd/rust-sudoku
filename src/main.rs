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
use clap::{crate_version, value_t, values_t, App, Arg};
// use std::vec::Vec;

fn main() {
    println!("BUILDINFO: {}", env!("BUILDINFO"));
    
    let matches = App::new("Sudoku Generator")
        .version(crate_version!())
        .author("G. Ralph Kuntz, MD <grk@usa.net>")
        .about("Generate sudoku puzzles")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .value_name("FILE")
                .help("Read a standard-format sudoku puzzle initial layout")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("level 0")
                .short("0")
                .long("easy")
                .value_name("COUNT")
                .help("Number of easy puzzles to generate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("level 1")
                .short("1")
                .long("medium")
                .value_name("COUNT")
                .help("Number of medium puzzles to generate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("level 2")
                .short("2")
                .long("hard")
                .value_name("COUNT")
                .help("Number of hard puzzles to generate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("level 3")
                .short("3")
                .long("ridiculous")
                .value_name("COUNT")
                .help("Number of ridiculous puzzles to generate")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("level 4")
                .short("4")
                .long("insane")
                .value_name("COUNT")
                .help("Number of insane puzzles to generate")
                .takes_value(true),
        )
        .get_matches();

    let level_0_count = value_t!(matches, "level 0", u32).unwrap_or(0);
    let level_1_count = value_t!(matches, "level 1", u32).unwrap_or(0);
    let level_2_count = value_t!(matches, "level 2", u32).unwrap_or(0);
    let level_3_count = value_t!(matches, "level 3", u32).unwrap_or(0);
    let level_4_count = value_t!(matches, "level 4", u32).unwrap_or(0);
    
    let inputs = values_t!(matches, "input", String).unwrap_or_else(|_e| Vec::new());

    println!("level 0 count: {}", level_0_count);
    println!("level 1 count: {}", level_1_count);
    println!("level 2 count: {}", level_2_count);
    println!("level 3 count: {}", level_3_count);
    println!("level 4 count: {}", level_4_count);
    println!("inputs: {:?}", inputs);
}
